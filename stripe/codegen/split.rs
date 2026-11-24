//! Splits the generated crate into a fixed number of type shards.
//!
//! The type graph is nearly acyclic. We emit one crate per group of dependency
//! levels, sharding the heavy levels so Cargo can type-check them in parallel.
//! The public `stripe` crate re-exports everything, so this is invisible to
//! users.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    sync::OnceLock,
};

use regex::Regex;

use crate::{
    emit::{MODULES_BEGIN, MODULES_END, OutputFile},
    schema::Model,
};

/// Number of generated type shards.
pub const SHARDS: usize = 16;
/// Aim for this many level bands; extra shards are allocated by weight.
const TARGET_BANDS: usize = 8;

struct Group {
    module: String,
    name: String,
    parts: Vec<String>,
    weight: usize,
}

struct Band {
    levels: Vec<usize>,
    weight: usize,
    shards: usize,
}

struct Parsed {
    groups: Vec<Group>,
    index: HashMap<(String, String), usize>,
    methods: BTreeMap<String, Vec<String>>,
}

fn item_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^pub (?:struct|enum|type) ([A-Za-z_][A-Za-z0-9_]*)").unwrap())
}

fn impl_for_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\bfor\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap())
}

fn impl_head_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^impl(?:<[^>]*>)?\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap())
}

fn ref_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"crate::([a-zA-Z0-9_]+)::([A-Za-z_][A-Za-z0-9_]*)").unwrap())
}

fn strip_strings_and_comments(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    let mut in_string = false;
    let mut escaped = false;
    while let Some(ch) = chars.next() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            out.push(' ');
            continue;
        }
        if ch == '/' && chars.peek() == Some(&'/') {
            break;
        }
        if ch == '"' {
            in_string = true;
            out.push(' ');
        } else {
            out.push(ch);
        }
    }
    out
}

fn brace_delta(line: &str) -> i32 {
    let line = strip_strings_and_comments(line);
    line.matches('{').count() as i32 - line.matches('}').count() as i32
}

fn bracket_delta(line: &str) -> i32 {
    let line = strip_strings_and_comments(line);
    (line.matches('(').count() + line.matches('[').count()) as i32
        - (line.matches(')').count() + line.matches(']').count()) as i32
}

/// Splits a generated module into top-level items, keeping attributes attached.
fn split_items(source: &str) -> Vec<String> {
    let lines: Vec<&str> = source.lines().collect();
    let mut items = Vec::new();
    let mut current: Vec<&str> = Vec::new();
    let mut depth = 0i32;
    let mut started = false;
    let mut i = 0usize;

    while i < lines.len() {
        let line = lines[i];
        let text = line.trim();
        if !started {
            if text.is_empty() {
                i += 1;
                continue;
            }
            if text.starts_with("//!") {
                i += 1;
                continue;
            }
            if text.starts_with("#![") {
                let mut d = bracket_delta(line);
                i += 1;
                while d > 0 && i < lines.len() {
                    d += bracket_delta(lines[i]);
                    i += 1;
                }
                continue;
            }
            current.push(line);
            if text.starts_with("#[") {
                let mut d = bracket_delta(line);
                i += 1;
                while d > 0 && i < lines.len() {
                    current.push(lines[i]);
                    d += bracket_delta(lines[i]);
                    i += 1;
                }
                continue;
            }
            if text.starts_with("///") {
                i += 1;
                continue;
            }
            started = true;
            depth += brace_delta(line);
            i += 1;
            if depth == 0 && (text.ends_with(';') || text.ends_with('}')) {
                items.push(current.join("\n"));
                current.clear();
                started = false;
            }
            continue;
        }

        current.push(line);
        depth += brace_delta(line);
        i += 1;
        if depth == 0 && (text.ends_with('}') || text.ends_with(';')) {
            items.push(current.join("\n"));
            current.clear();
            started = false;
        }
    }
    if !current.is_empty() {
        items.push(current.join("\n"));
    }
    items
}

/// The first non-attribute, non-doc line of an item.
fn item_head(item: &str) -> &str {
    let lines: Vec<&str> = item.lines().collect();
    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i];
        let text = line.trim();
        if text.is_empty() {
            i += 1;
            continue;
        }
        if text.starts_with("#[") {
            let mut d = bracket_delta(line);
            i += 1;
            while d > 0 && i < lines.len() {
                d += bracket_delta(lines[i]);
                i += 1;
            }
            continue;
        }
        if text.starts_with("///") || text.starts_with("//!") {
            i += 1;
            continue;
        }
        return text;
    }
    ""
}

fn type_name(item: &str) -> Option<String> {
    item_re()
        .captures(item_head(item))
        .map(|captures| captures[1].to_string())
}

fn impl_target(item: &str) -> Option<String> {
    let head = item_head(item);
    if !head.starts_with("impl") {
        return None;
    }
    if let Some(captures) = impl_for_re().captures(head) {
        return Some(captures[1].to_string());
    }
    impl_head_re().captures(head).map(|captures| captures[1].to_string())
}

fn parse(files: &[OutputFile]) -> Parsed {
    let mut groups = Vec::new();
    let mut index = HashMap::new();
    let mut methods = BTreeMap::new();

    for file in files {
        let Some(module) = file.path.strip_prefix("api/").and_then(|path| path.strip_suffix(".rs")) else {
            continue;
        };
        if module == "modules" {
            continue;
        }
        let mut last: Option<usize> = None;
        for item in split_items(&file.contents) {
            let head = item_head(&item);
            if let Some(name) = type_name(&item) {
                let key = (module.to_string(), name.to_string());
                let group = Group {
                    module: module.to_string(),
                    name: name.to_string(),
                    parts: vec![item],
                    weight: 0,
                };
                let id = groups.len();
                groups.push(group);
                index.insert(key, id);
                last = Some(id);
            } else if head.starts_with("impl") {
                let key = impl_target(&item).map(|target| (module.to_string(), target.to_string()));
                let id = key.and_then(|key| index.get(&key).copied()).or(last);
                if let Some(id) = id {
                    groups[id].parts.push(item);
                }
            } else if head.starts_with("pub async fn") {
                methods.entry(module.to_string()).or_insert_with(Vec::new).push(item);
            }
        }
    }

    for group in &mut groups {
        group.weight = group.parts.iter().map(String::len).sum();
    }

    Parsed {
        groups,
        index,
        methods,
    }
}

fn strongly_connected_components(adjacency: &[Vec<usize>]) -> Vec<usize> {
    let n = adjacency.len();
    let mut visited = vec![false; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);
    for start in 0..n {
        if visited[start] {
            continue;
        }
        let mut stack = vec![(start, 0usize)];
        visited[start] = true;
        while let Some((node, edge)) = stack.pop() {
            if edge < adjacency[node].len() {
                stack.push((node, edge + 1));
                let next = adjacency[node][edge];
                if !visited[next] {
                    visited[next] = true;
                    stack.push((next, 0));
                }
            } else {
                order.push(node);
            }
        }
    }
    let mut reverse: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (from, targets) in adjacency.iter().enumerate() {
        for &to in targets {
            reverse[to].push(from);
        }
    }
    let mut component = vec![usize::MAX; n];
    let mut current = 0usize;
    for &start in order.iter().rev() {
        if component[start] != usize::MAX {
            continue;
        }
        let mut stack = vec![start];
        component[start] = current;
        while let Some(node) = stack.pop() {
            for &next in &reverse[node] {
                if component[next] == usize::MAX {
                    component[next] = current;
                    stack.push(next);
                }
            }
        }
        current += 1;
    }
    component
}

fn components_and_levels(parsed: &Parsed) -> (Vec<usize>, Vec<usize>) {
    let mut adjacency = vec![Vec::new(); parsed.groups.len()];
    for (id, group) in parsed.groups.iter().enumerate() {
        let text = group.parts.join("\n");
        for captures in ref_re().captures_iter(&text) {
            let key = (captures[1].to_string(), captures[2].to_string());
            if let Some(&target) = parsed.index.get(&key)
                && target != id
                && !adjacency[id].contains(&target)
            {
                adjacency[id].push(target);
            }
        }
    }
    let component = strongly_connected_components(&adjacency);
    let components = component.iter().copied().max().map_or(0, |max| max + 1);
    let mut edges = vec![HashSet::new(); components];
    for (from, targets) in adjacency.iter().enumerate() {
        for &to in targets {
            if component[from] != component[to] {
                edges[component[from]].insert(component[to]);
            }
        }
    }
    fn level_of(node: usize, edges: &[HashSet<usize>], memo: &mut [usize]) -> usize {
        if memo[node] != usize::MAX {
            return memo[node];
        }
        let mut level = 0;
        for &next in &edges[node] {
            level = level.max(level_of(next, edges, memo) + 1);
        }
        memo[node] = level;
        level
    }
    let mut memo = vec![usize::MAX; components];
    for node in 0..components {
        level_of(node, &edges, &mut memo);
    }
    let group_levels = parsed
        .groups
        .iter()
        .map(|group| memo[component[parsed.index[&(group.module.clone(), group.name.clone())]]])
        .collect();
    (component, group_levels)
}

struct Unit {
    members: Vec<usize>,
    weight: usize,
    level: usize,
}

fn allocate(units: &[Unit]) -> Vec<usize> {
    let mut level_weights: BTreeMap<usize, usize> = BTreeMap::new();
    for unit in units {
        *level_weights.entry(unit.level).or_default() += unit.weight;
    }
    let mut bands: Vec<Band> = level_weights
        .into_iter()
        .map(|(level, weight)| Band {
            levels: vec![level],
            weight,
            shards: 1,
        })
        .collect();

    while bands.len() > TARGET_BANDS {
        let mut best = 0usize;
        let mut best_weight = usize::MAX;
        for index in 0..bands.len() - 1 {
            let combined = bands[index].weight + bands[index + 1].weight;
            if combined < best_weight {
                best_weight = combined;
                best = index;
            }
        }
        let next = bands.remove(best + 1);
        let band = &mut bands[best];
        band.levels.extend(next.levels);
        band.weight += next.weight;
    }

    while bands.iter().map(|band| band.shards).sum::<usize>() < SHARDS {
        let mut best = 0usize;
        let mut best_marginal = 0.0f64;
        for (index, band) in bands.iter().enumerate() {
            let marginal = band.weight as f64 / (band.shards * (band.shards + 1)) as f64;
            if marginal > best_marginal {
                best_marginal = marginal;
                best = index;
            }
        }
        bands[best].shards += 1;
    }

    let mut shard_of_unit = vec![0usize; units.len()];
    let mut next_shard = 0usize;
    for band in &bands {
        let start = next_shard;
        next_shard += band.shards;
        let mut ids: Vec<usize> = units
            .iter()
            .enumerate()
            .filter(|(_, unit)| band.levels.contains(&unit.level))
            .map(|(id, _)| id)
            .collect();
        if band.shards == 1 {
            for id in ids {
                shard_of_unit[id] = start;
            }
        } else if band.levels.len() == 1 {
            let mut loads = vec![0usize; band.shards];
            ids.sort_by(|a, b| {
                units[*b]
                    .weight
                    .cmp(&units[*a].weight)
                    .then_with(|| units[*a].members[0].cmp(&units[*b].members[0]))
            });
            for id in ids {
                let slot = (0..band.shards).min_by_key(|slot| (loads[*slot], *slot)).unwrap();
                shard_of_unit[id] = start + slot;
                loads[slot] += units[id].weight;
            }
        } else {
            // Keep shards level-contiguous so the crate graph cannot cycle.
            let total: usize = band
                .levels
                .iter()
                .map(|level| {
                    units
                        .iter()
                        .filter(|unit| unit.level == *level)
                        .map(|unit| unit.weight)
                        .sum::<usize>()
                })
                .sum();
            let mut slot = 0usize;
            let mut accumulated = 0usize;
            for level in &band.levels {
                let level_weight: usize = units
                    .iter()
                    .filter(|unit| unit.level == *level)
                    .map(|unit| unit.weight)
                    .sum();
                if slot + 1 < band.shards && accumulated + level_weight > total * (slot + 1) / band.shards {
                    slot += 1;
                }
                for id in ids.iter().copied().filter(|id| units[*id].level == *level) {
                    shard_of_unit[id] = start + slot;
                }
                accumulated += level_weight;
            }
        }
    }
    shard_of_unit
}

fn rewrite(text: &str, from: Option<usize>, parsed: &Parsed, shard_of: &[usize]) -> String {
    ref_re()
        .replace_all(text, |captures: &regex::Captures<'_>| {
            let key = (captures[1].to_string(), captures[2].to_string());
            let Some(&target) = parsed.index.get(&key) else {
                return captures[0].to_string();
            };
            let module = &captures[1];
            let name = &captures[2];
            match from {
                Some(shard) if shard_of[target] == shard => format!("crate::{module}::{name}"),
                _ => format!("stripe_types_{:02}::{module}::{name}", shard_of[target]),
            }
        })
        .into_owned()
}

fn features(model: &Model) -> String {
    model.modules.iter().map(|module| format!("{module} = []\n")).collect()
}

/// Emits the sharded layout and the facade module block.
pub fn emit(model: &Model) -> Vec<OutputFile> {
    let monolithic = crate::emit::emit(model);
    let parsed = parse(&monolithic);
    let (component_of, group_levels) = components_and_levels(&parsed);
    let component_count = component_of.iter().copied().max().map_or(0, |max| max + 1);
    let mut units: Vec<Unit> = (0..component_count)
        .map(|_| Unit {
            members: Vec::new(),
            weight: 0,
            level: 0,
        })
        .collect();
    for (id, group) in parsed.groups.iter().enumerate() {
        let component = component_of[id];
        units[component].members.push(id);
        units[component].weight += group.weight;
        units[component].level = group_levels[id];
    }
    let shard_of_unit = allocate(&units);
    let shard_of: Vec<usize> = component_of.iter().map(|component| shard_of_unit[*component]).collect();

    let mut shard_modules: Vec<BTreeMap<String, Vec<usize>>> = vec![BTreeMap::new(); SHARDS];
    for (id, group) in parsed.groups.iter().enumerate() {
        shard_modules[shard_of[id]]
            .entry(group.module.clone())
            .or_default()
            .push(id);
    }

    let mut shard_deps: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); SHARDS];
    for (id, group) in parsed.groups.iter().enumerate() {
        let text = group.parts.join("\n");
        for captures in ref_re().captures_iter(&text) {
            let key = (captures[1].to_string(), captures[2].to_string());
            if let Some(&target) = parsed.index.get(&key) {
                let target_shard = shard_of[target];
                if target_shard != shard_of[id] {
                    shard_deps[shard_of[id]].insert(target_shard);
                }
            }
        }
    }

    let mut files = Vec::new();
    for shard in 0..SHARDS {
        let mut lib = String::from(
            "#![allow(missing_docs)]\n#![allow(clippy::all)]\n#![allow(non_camel_case_types)]\n#![allow(unused_crate_dependencies)]\n\npub use stripe_core::*;\n\n",
        );
        for (module, ids) in &shard_modules[shard] {
            lib.push_str(&format!("pub mod {module};\n"));
            let mut text = String::new();
            for id in ids {
                if !text.is_empty() {
                    text.push_str("\n\n");
                }
                text.push_str(&parsed.groups[*id].parts.join("\n\n"));
            }
            files.push(OutputFile {
                path: format!("types_{shard:02}/{module}.rs"),
                contents: format!("{}\n", rewrite(&text, Some(shard), &parsed, &shard_of)),
            });
        }
        for dep in &shard_deps[shard] {
            lib.push_str(&format!("#[allow(unused_imports)]\nuse stripe_types_{dep:02} as _;\n"));
        }
        files.push(OutputFile {
            path: format!("types_{shard:02}/lib.rs"),
            contents: lib,
        });
        let lower: String = shard_deps[shard]
            .iter()
            .map(|lower| format!("stripe_types_{lower:02} = {{ path = \"../types_{lower:02}\" }}\n"))
            .collect();
        files.push(OutputFile {
            path: format!("types_{shard:02}/Cargo.toml"),
            contents: format!(
                "[package]\nname = \"stripe_types_{shard:02}\"\nversion = \"0.1.0\"\ndescription = \"Generated Stripe API types (shard {shard}).\"\nedition.workspace = true\nrust-version.workspace = true\nlicense.workspace = true\nrepository.workspace = true\n\n[lints]\nworkspace = true\n\n[lib]\npath = \"lib.rs\"\n\n[features]\ndefault = []\n{}[dependencies]\nstripe_core = {{ path = \"../core\" }}\n{lower}\njson = {{ path = \"../../json\", features = [\"derive\"] }}\n",
                features(model)
            ),
        });
    }

    let mut api_lib = String::from(
        "#![allow(missing_docs)]\n#![allow(clippy::all)]\n#![allow(non_camel_case_types)]\n#![allow(unused_crate_dependencies)]\n\npub use stripe_core::*;\n\n",
    );
    for module in &model.modules {
        let Some(methods) = parsed.methods.get(module) else {
            continue;
        };
        api_lib.push_str(&format!("pub mod {module};\n"));
        let text = methods.join("\n\n");
        files.push(OutputFile {
            path: format!("api/{module}.rs"),
            contents: format!("{}\n", rewrite(&text, None, &parsed, &shard_of)),
        });
    }
    let mut api_deps: BTreeSet<usize> = BTreeSet::new();
    for methods in parsed.methods.values() {
        for text in methods {
            for captures in ref_re().captures_iter(text) {
                let key = (captures[1].to_string(), captures[2].to_string());
                if let Some(&target) = parsed.index.get(&key) {
                    api_deps.insert(shard_of[target]);
                }
            }
        }
    }
    for dep in &api_deps {
        api_lib.push_str(&format!("#[allow(unused_imports)]\nuse stripe_types_{dep:02} as _;\n"));
    }
    files.push(OutputFile {
        path: "api/lib.rs".to_string(),
        contents: api_lib,
    });
    let type_deps: String = api_deps
        .iter()
        .map(|shard| format!("stripe_types_{shard:02} = {{ path = \"../types_{shard:02}\" }}\n"))
        .collect();
    files.push(OutputFile {
        path: "api/Cargo.toml".to_string(),
        contents: format!(
            "[package]\nname = \"stripe_api\"\nversion = \"0.1.0\"\ndescription = \"Generated Stripe API methods.\"\nedition.workspace = true\nrust-version.workspace = true\nlicense.workspace = true\nrepository.workspace = true\n\n[lints]\nworkspace = true\n\n[lib]\npath = \"lib.rs\"\n\n[features]\ndefault = []\n{}[dependencies]\nstripe_core = {{ path = \"../core\" }}\n{type_deps}",
            features(model)
        ),
    });

    let mut modules = String::new();
    modules.push_str(MODULES_BEGIN);
    modules.push_str("\n\npub use stripe_core::*;\n\n");
    for module in &model.modules {
        if module == "events" {
            modules.push_str("#[cfg(feature = \"events\")]\n");
        }
        modules.push_str(&format!("pub mod {module} {{\n"));
        if parsed.methods.contains_key(module) {
            modules.push_str(&format!("    #[allow(unused_imports)]\n    pub use stripe_api::{module}::*;\n"));
        }
        for shard in 0..SHARDS {
            if shard_modules[shard].contains_key(module) {
                modules.push_str(&format!(
                    "    #[allow(unused_imports)]\n    pub use stripe_types_{shard:02}::{module}::*;\n"
                ));
            }
        }
        modules.push_str("}\n\n");
    }
    modules.push_str(MODULES_END);
    modules.push('\n');
    files.push(OutputFile {
        path: "modules.rs".to_string(),
        contents: modules,
    });

    // Keep the report useful without changing the output format.
    files
}
