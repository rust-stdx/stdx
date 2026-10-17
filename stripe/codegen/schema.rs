//! Builds the intermediate representation from Stripe's OpenAPI document.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use serde_json::Value;

use crate::{
    ir::{Field, Mode, TypeDef, TypeKind, TypeRef, UnionArm, Variant},
    naming,
};

/// Maps a schema-name prefix to the generated module that owns it.
///
/// Most types are placed by matching their name against the resource roots
/// derived from the API paths (see [`naming::module_of_path`]). The entries here
/// cover names that do not match a resource root, either because Stripe groups
/// them under a different internal name (`portal` -> `billing_portal`) or
/// because they are shared plumbing used by several resources.
const SCHEMA_MODULE_ALIASES: &[(&str, &str)] = &[
    ("api_errors", "common"),
    ("error", "common"),
    ("payment_flows", "payment_intents"),
    ("payments_primitives", "payment_records"),
    ("gelato", "identity"),
    ("portal", "billing_portal"),
    ("payment_pages", "checkout"),
    ("connect", "accounts"),
    ("insights", "radar"),
    ("reserve", "treasury"),
    ("reserves", "treasury"),
    ("outbound_payments", "treasury"),
    ("outbound_transfers", "treasury"),
    ("inbound_transfers", "treasury"),
    ("received_credits", "treasury"),
    ("received_debits", "treasury"),
    ("received_payment_method_details", "treasury"),
    ("funding_instructions", "payment_intents"),
    ("legal_entity", "accounts"),
    ("person", "accounts"),
    ("external_account", "accounts"),
    ("capability", "accounts"),
    ("platform_earning_fee_source", "application_fees"),
    ("bank_account", "payment_methods"),
    ("bank", "payment_methods"),
    ("card", "payment_methods"),
    ("source", "sources"),
    ("line_item", "invoices"),
    ("invoice_setting", "invoices"),
    ("invoicing", "invoices"),
    ("credited_items", "invoices"),
    ("proration_details", "invoices"),
    ("discount", "coupons"),
    ("stackable_discount", "coupons"),
    ("thresholds_resource", "billing"),
    ("level3", "charges"),
    ("transform_quantity", "prices"),
    ("transform_usage", "prices"),
    ("notification_event", "events"),
    ("forwarded_request", "forwarding"),
    ("forwarded_response", "forwarding"),
    ("internal_card", "issuing"),
    ("address_api_resource", "terminal"),
    ("offline_acceptance", "terminal"),
    ("online_acceptance", "terminal"),
    ("scheduled_query_run", "sigma"),
    ("financial_reporting", "reporting"),
    ("secret_service", "apps"),
    ("verification_session", "identity"),
    ("linked_account_options", "financial_connections"),
    ("schedules_phase", "subscription_schedules"),
    ("sepa_debit_generated_from", "payment_methods"),
    ("networks", "payment_methods"),
    ("klarna", "payment_methods"),
    ("amazon_pay", "payment_methods"),
    ("revolut_pay", "payment_methods"),
    ("paypal_seller_protection", "payment_methods"),
    ("cash_balance", "customers"),
    ("credit_balance", "billing"),
    ("rule", "radar"),
];

/// A generated API method.
#[derive(Debug, Clone)]
pub struct Method {
    /// Generated module this method lives in.
    pub module: String,
    /// Path template, e.g. `/v1/customers/{customer}`.
    pub template: String,
    /// `get`, `post` or `delete`.
    pub verb: Verb,
    /// Whether the request body is JSON (v2) rather than form-encoded.
    pub json: bool,
    /// Whether the request body uses `multipart/form-data`, which the client
    /// does not encode yet.
    pub multipart: bool,
    /// Generated function name.
    pub function: String,
    /// Fully-qualified params type, if the operation takes parameters.
    pub params_type: Option<String>,
    /// Rust parameter name for each `{placeholder}` in the path, in order.
    pub path_args: Vec<(String, String)>,
    /// The response type.
    pub response: TypeRef,
    /// Documentation.
    pub docs: String,
}

/// The HTTP verb of a generated method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verb {
    /// `GET`
    Get,
    /// `POST`
    Post,
    /// `DELETE`
    Delete,
}

/// The complete generated model.
pub struct Model {
    /// All generated types, grouped implicitly by module.
    pub types: Vec<TypeDef>,
    /// All generated methods.
    pub methods: Vec<Method>,
    /// Every generated module; each one is also a cargo feature.
    pub modules: BTreeSet<String>,
    /// Types dropped because no endpoint or webhook payload can reach them.
    pub pruned: usize,
    /// The API version from the spec.
    pub spec_version: String,
    /// For each type (by FQN), the modules whose generated methods can reach
    /// it. Response types are compiled only when one of these features is
    /// enabled; see [`Builder::compute_gating`].
    pub gating: BTreeMap<String, BTreeSet<String>>,
}

struct SchemaMeta {
    module: String,
    name: String,
    fqn: String,
}

/// Incrementally builds the model while assigning unique type names.
pub struct Builder {
    spec: Value,
    types: Vec<TypeDef>,
    schema_meta: HashMap<String, SchemaMeta>,
    /// Names already used, per module: two modules may both define `Card`.
    used_names: HashMap<String, HashSet<String>>,
    synth_counts: HashMap<String, u32>,
    /// FQNs of generated enums, which implement `Default`.
    enum_fqns: HashSet<String>,
    /// FQN of each generated type alias and the type it points at, so the
    /// "can this field fall back to a default?" check can see through aliases.
    alias_targets: HashMap<String, TypeRef>,
    /// Types dropped as unreachable, for the generation report.
    pruned: usize,
    methods: Vec<Method>,
    modules: BTreeSet<String>,
    dedup: HashMap<String, String>,
}

/// Where a synthesized type lives and how it is encoded.
#[derive(Clone)]
struct Ctx {
    module: String,
    mode: Mode,
}

/// Extracts `{placeholder}` names from a path template, in order of appearance.
fn path_placeholders(path: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = path;
    while let Some(open) = rest.find('{') {
        let after = &rest[open + 1..];
        match after.find('}') {
            Some(close) => {
                let name = &after[..close];
                if !names.iter().any(|existing| existing == name) {
                    names.push(name.to_string());
                }
                rest = &after[close + 1..];
            }
            None => break,
        }
    }
    names
}

impl Builder {
    /// Creates a builder over a parsed OpenAPI document.
    pub fn new(spec: Value) -> Self {
        Self {
            spec,
            types: Vec::new(),
            schema_meta: HashMap::new(),
            used_names: HashMap::new(),
            synth_counts: HashMap::new(),
            enum_fqns: HashSet::new(),
            alias_targets: HashMap::new(),
            pruned: 0,
            methods: Vec::new(),
            modules: BTreeSet::new(),
            dedup: HashMap::new(),
        }
    }

    /// Builds the full model.
    ///
    /// # Errors
    ///
    /// Returns a description of the first naming conflict, which means the
    /// generator needs a rule for the operation that collided.
    pub fn build(mut self) -> Result<Model, String> {
        let spec_version = self
            .spec
            .pointer("/info/version")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();

        self.collect_modules();
        self.reserve_schema_names();

        // Build every component schema into a named type. Schemas for deleted
        // resources are built last, so the live shape owns the shared
        // name (`PriceObject`) and the deleted one aliases it.
        let mut keys: Vec<String> = self.schemas().keys().cloned().collect();
        keys.sort_by(|a, b| {
            let deleted = |key: &String| key.starts_with("deleted_") || key.contains(".deleted_");
            deleted(a).cmp(&deleted(b)).then_with(|| a.cmp(b))
        });
        for key in &keys {
            self.build_schema_type(key);
        }

        self.build_methods();
        self.compute_enum_usage();
        self.mark_serialize();
        self.prune_unreachable();
        self.apply_boxing();
        let gating = self.compute_gating();

        let mut seen: HashMap<(&str, &str), &str> = HashMap::new();
        for method in &self.methods {
            let key = (method.module.as_str(), method.function.as_str());
            if let Some(previous) = seen.insert(key, method.template.as_str()) {
                return Err(format!(
                    "duplicate method `{}::{}` for `{}` and `{}`",
                    method.module, method.function, previous, method.template
                ));
            }
        }

        Ok(Model {
            types: self.types,
            methods: self.methods,
            modules: self.modules,
            spec_version,
            pruned: self.pruned,
            gating,
        })
    }

    /// Every module is derived from the API paths; each becomes a cargo feature.
    fn collect_modules(&mut self) {
        let paths = self
            .spec
            .pointer("/paths")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        for path in paths.keys() {
            self.modules.insert(naming::module_of_path(path));
        }
        // Types that belong to no resource module live here.
        self.modules.insert("common".to_string());
        self.modules.insert("events".to_string());
    }

    fn schemas(&self) -> &serde_json::Map<String, Value> {
        self.spec
            .pointer("/components/schemas")
            .and_then(Value::as_object)
            .expect("spec has components.schemas")
    }

    fn schema(&self, key: &str) -> Option<&Value> {
        self.schemas().get(key)
    }

    /// The module that owns the type generated for `key`.
    ///
    /// Webhook payload schemas are grouped in the `events` module, v2 schemas by
    /// API area, and everything else by the longest resource prefix of its name
    /// (with [`SCHEMA_MODULE_ALIASES`] covering the exceptions). Unmatched names
    /// are shared plumbing and live in `common`.
    fn schema_module(&self, key: &str) -> String {
        if self
            .schema(key)
            .and_then(|schema| schema.get("x-stripeEvent"))
            .is_some()
        {
            return "events".to_string();
        }
        if let Some(rest) = key.strip_prefix("v2.") {
            let head = rest.split('.').next().unwrap_or("core");
            if head == "error" || head.starts_with("deleted_") {
                return "common".to_string();
            }
            return naming::plain_ident(&format!("v2_{head}"));
        }
        // Dotted keys such as `issuing.card` are owned by their namespace.
        let prefix = key.split('.').next().unwrap_or(key);
        let name = prefix.strip_prefix("deleted_").unwrap_or(prefix);
        let mut best: Option<(&str, &str)> = None;
        let roots = self.schema_roots();
        for (root, module) in &roots {
            if (name == root || name.starts_with(&format!("{root}_")))
                && best.is_none_or(|(current, _)| root.len() > current.len())
            {
                best = Some((root.as_str(), module.as_str()));
            }
        }
        best.map_or_else(|| "common".to_string(), |(_, module)| module.to_string())
    }

    /// Resource prefixes that map to modules, longest first.
    fn schema_roots(&self) -> Vec<(String, String)> {
        let mut roots: Vec<(String, String)> = Vec::new();
        for module in &self.modules {
            roots.push((module.clone(), module.clone()));
            roots.push((naming::singular(module), module.clone()));
        }
        for (prefix, module) in SCHEMA_MODULE_ALIASES {
            roots.push(((*prefix).to_string(), (*module).to_string()));
        }
        roots.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));
        roots.dedup_by(|a, b| a.0 == b.0);
        roots
    }

    fn reserve_schema_names(&mut self) {
        let mut keys: Vec<String> = self.schemas().keys().cloned().collect();
        keys.sort();
        for key in keys {
            let module = self.schema_module(&key);
            let base = naming::type_name_from_schema(&key);
            let name = self.unique_name(&module, &base);
            let fqn = format!("crate::{module}::{name}");
            self.schema_meta.insert(
                key,
                SchemaMeta {
                    module,
                    name,
                    fqn,
                },
            );
        }
    }

    fn unique_name(&mut self, module: &str, base: &str) -> String {
        let used = self.used_names.entry(module.to_string()).or_default();
        if used.insert(base.to_string()) {
            return base.to_string();
        }
        let mut index = 2u32;
        loop {
            let candidate = format!("{base}{index}");
            if used.insert(candidate.clone()) {
                return candidate;
            }
            index += 1;
        }
    }

    fn synthesize_name(&mut self, module: &str, hint: &str) -> String {
        let base = naming::pascal_case(hint);
        let count = {
            // Count per module: two modules may both synthesize `GetCustomerParams`.
            let entry = self.synth_counts.entry(format!("{module}|{base}")).or_insert(0);
            *entry += 1;
            *entry
        };
        let candidate = if count == 1 { base } else { format!("{base}{count}") };
        self.unique_name(module, &candidate)
    }

    fn fqn_for_ref(&self, reference: &str) -> Option<&str> {
        let key = reference.rsplit('/').next()?;
        self.schema_meta.get(key).map(|meta| meta.fqn.as_str())
    }

    /// Boxes direct struct fields that participate in a reference cycle, so
    /// recursive types are representable without boxing non-recursive fields.
    fn apply_boxing(&mut self) {
        // Index struct definitions.
        let mut index: HashMap<String, usize> = HashMap::new();
        let mut structs: Vec<usize> = Vec::new();
        for (position, ty) in self.types.iter().enumerate() {
            if matches!(ty.kind, TypeKind::Struct(_)) {
                index.insert(ty.fqn.clone(), position);
                structs.push(position);
            }
        }
        let node_of = |fqn: &str| index.get(fqn).copied();

        let node_count = self.types.len();
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); node_count];
        for &position in &structs {
            let TypeKind::Struct(fields) = &self.types[position].kind else {
                continue;
            };
            for field in fields {
                if let Some(target) = direct_struct_target(&field.ty)
                    && let Some(target_position) = node_of(target)
                {
                    adjacency[position].push(target_position);
                }
            }
        }

        let component = strongly_connected_components(&adjacency);

        for &position in &structs {
            let TypeKind::Struct(fields) = &self.types[position].kind else {
                continue;
            };
            let mut boxed_flags = Vec::with_capacity(fields.len());
            for field in fields {
                let should_box = match direct_struct_target(&field.ty) {
                    Some(target) => {
                        node_of(target).is_some_and(|target_position| component[position] == component[target_position])
                    }
                    None => false,
                };
                boxed_flags.push(should_box);
            }
            if let TypeKind::Struct(fields) = &mut self.types[position].kind {
                for (field, should_box) in fields.iter_mut().zip(boxed_flags) {
                    field.boxed = should_box;
                }
            }
        }
    }

    /// Marks response types reachable from a JSON (v2) request body as
    /// `Serialize`, so only the handful of types that v2 bodies reference pay
    /// the codegen cost.
    fn mark_serialize(&mut self) {
        let mut index: HashMap<String, usize> = HashMap::new();
        for (position, ty) in self.types.iter().enumerate() {
            index.insert(ty.fqn.clone(), position);
        }
        let mut queue: Vec<usize> = Vec::new();
        for (position, ty) in self.types.iter().enumerate() {
            if ty.mode == Mode::Json {
                queue.push(position);
            }
        }
        let mut seen: HashSet<usize> = HashSet::new();
        while let Some(position) = queue.pop() {
            if !seen.insert(position) {
                continue;
            }
            self.types[position].serialize = true;
            let targets: Vec<String> = match &self.types[position].kind {
                TypeKind::Struct(fields) => fields
                    .iter()
                    .filter_map(|field| direct_named(&field.ty).map(str::to_string))
                    .collect(),
                TypeKind::Alias(target) | TypeKind::Map(target) => {
                    direct_named(target).map(str::to_string).into_iter().collect()
                }
                TypeKind::Union(arms) => arms
                    .iter()
                    .filter_map(|arm| direct_named(&arm.ty).map(str::to_string))
                    .collect(),
                TypeKind::Enum(_) => Vec::new(),
            };
            for target in targets {
                if let Some(&next) = index.get(&target) {
                    queue.push(next);
                }
            }
        }
    }

    /// Records which enums need a `Default` impl (they are the type of a
    /// response field that is not an `Option`) and which need `FormEncode` (a v1
    /// request body can reach them). Everything else is left out to keep the
    /// generated code small.
    fn compute_enum_usage(&mut self) {
        let index: HashMap<String, usize> = self
            .types
            .iter()
            .enumerate()
            .map(|(position, ty)| (ty.fqn.clone(), position))
            .collect();

        // Enums a non-`Option` response field falls back to.
        let mut needs_default: HashSet<String> = HashSet::new();
        for ty in &self.types {
            if ty.mode != Mode::Response {
                continue;
            }
            let TypeKind::Struct(fields) = &ty.kind else {
                continue;
            };
            for field in fields {
                if matches!(field.ty, TypeRef::Opt(_)) {
                    continue;
                }
                if let Some(fqn) = self.enum_target(&index, &field.ty) {
                    needs_default.insert(fqn);
                }
            }
        }

        // Enums a form request body can reach.
        let adjacency = self.reference_graph(&index);
        let mut form_reachable: HashSet<usize> = HashSet::new();
        let mut stack: Vec<usize> = self
            .types
            .iter()
            .enumerate()
            .filter(|(_, ty)| ty.mode == Mode::Form)
            .map(|(position, _)| position)
            .collect();
        while let Some(position) = stack.pop() {
            if !form_reachable.insert(position) {
                continue;
            }
            stack.extend(adjacency[position].iter().copied());
        }

        for (position, ty) in self.types.iter_mut().enumerate() {
            if !matches!(ty.kind, TypeKind::Enum(_)) {
                continue;
            }
            ty.needs_default = needs_default.contains(&ty.fqn);
            ty.needs_form_encode = form_reachable.contains(&position);
        }
    }

    /// The enum `ty` refers to, following type aliases, if any.
    fn enum_target(&self, index: &HashMap<String, usize>, ty: &TypeRef) -> Option<String> {
        let TypeRef::Named(fqn) = ty else {
            return None;
        };
        let mut current = fqn.as_str();
        for _ in 0..8 {
            let position = *index.get(current)?;
            let candidate = &self.types[position];
            match &candidate.kind {
                TypeKind::Enum(_) => return Some(candidate.fqn.clone()),
                TypeKind::Alias(target) => {
                    let TypeRef::Named(next) = target else {
                        return None;
                    };
                    current = next.as_str();
                }
                _ => return None,
            }
        }
        None
    }

    /// Edges between generated types, in `self.types` order.
    fn reference_graph(&self, index: &HashMap<String, usize>) -> Vec<Vec<usize>> {
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); self.types.len()];
        for (position, ty) in self.types.iter().enumerate() {
            let mut refs = HashSet::new();
            type_def_refs(ty, &mut refs);
            for reference in refs {
                if let Some(&target) = index.get(&reference) {
                    adjacency[position].push(target);
                }
            }
        }
        adjacency
    }

    /// Drops types no endpoint (or webhook payload) can reach.
    ///
    /// Stripe's specification includes schemas that no request or response of
    /// this client refers to — the v2 error catalogue, for example. Generating
    /// them would cost compile time and binary size for nothing, so they are
    /// skipped. Webhook payloads are roots, because application code names them
    /// directly.
    fn prune_unreachable(&mut self) {
        let index: HashMap<String, usize> = self
            .types
            .iter()
            .enumerate()
            .map(|(position, ty)| (ty.fqn.clone(), position))
            .collect();
        let adjacency = self.reference_graph(&index);

        let mut roots: Vec<usize> = Vec::new();
        for method in &self.methods {
            let mut refs = HashSet::new();
            type_refs(&method.response, &mut refs);
            if let Some(params) = &method.params_type {
                refs.insert(params.clone());
            }
            for reference in refs {
                if let Some(&position) = index.get(&reference) {
                    roots.push(position);
                }
            }
        }
        for (position, ty) in self.types.iter().enumerate() {
            if ty.module == "events" {
                roots.push(position);
            }
        }

        let mut reachable = vec![false; self.types.len()];
        while let Some(position) = roots.pop() {
            if reachable[position] {
                continue;
            }
            reachable[position] = true;
            roots.extend(adjacency[position].iter().copied());
        }

        let previous = std::mem::take(&mut self.types);
        let mut kept = Vec::with_capacity(previous.len());
        let mut pruned = 0usize;
        for (position, ty) in previous.into_iter().enumerate() {
            if reachable[position] {
                kept.push(ty);
            } else {
                pruned += 1;
            }
        }
        self.pruned = pruned;
        self.types = kept;
    }

    /// Maps every surviving type to the modules whose generated methods can
    /// reach it.
    ///
    /// A response type is only useful to a build that can return it, so the
    /// emitter gates it with `#[cfg(any(feature = "<module>", ...))]` over this
    /// set. The property that makes that sound is monotonicity: if `U`
    /// references `T`, then `needed(T)` is a superset of `needed(U)`, so an
    /// enabled module can never reference a type that is gated out.
    ///
    /// Roots are each method's response and parameter types (tagged with the
    /// method's module) plus every webhook payload, which application code
    /// names directly even though no method returns it.
    fn compute_gating(&self) -> BTreeMap<String, BTreeSet<String>> {
        let index: HashMap<String, usize> = self
            .types
            .iter()
            .enumerate()
            .map(|(position, ty)| (ty.fqn.clone(), position))
            .collect();
        let adjacency = self.reference_graph(&index);

        let mut needed: Vec<BTreeSet<String>> = vec![BTreeSet::new(); self.types.len()];
        let mut queue: Vec<usize> = Vec::new();

        for method in &self.methods {
            let mut refs = HashSet::new();
            type_refs(&method.response, &mut refs);
            if let Some(params) = &method.params_type {
                refs.insert(params.clone());
            }
            for reference in refs {
                if let Some(&position) = index.get(reference.as_str())
                    && needed[position].insert(method.module.clone())
                {
                    queue.push(position);
                }
            }
        }
        for (position, ty) in self.types.iter().enumerate() {
            if ty.module == "events" && needed[position].insert("events".to_string()) {
                queue.push(position);
            }
        }

        while let Some(position) = queue.pop() {
            let inherited: Vec<String> = needed[position].iter().cloned().collect();
            for &target in &adjacency[position] {
                let mut changed = false;
                for module in &inherited {
                    changed |= needed[target].insert(module.clone());
                }
                if changed {
                    queue.push(target);
                }
            }
        }

        self.types
            .iter()
            .zip(needed)
            .filter(|(_, needed)| !needed.is_empty())
            .map(|(ty, needed)| (ty.fqn.clone(), needed))
            .collect()
    }

    fn build_schema_type(&mut self, key: &str) {
        let meta = match self.schema_meta.get(key) {
            Some(meta) => (meta.module.clone(), meta.name.clone(), meta.fqn.clone()),
            None => return,
        };
        let (module, name, fqn) = meta;
        if self.types.iter().any(|ty| ty.fqn == fqn) {
            return;
        }
        let Some(raw) = self.schema(key) else {
            return;
        };
        let schema = self.resolve(raw);
        let docs = schema
            .get("description")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let ctx = Ctx {
            module: module.clone(),
            mode: Mode::Response,
        };

        let (kind, mode) = if let Some(variants) = self.enum_variants(&schema) {
            self.enum_fqns.insert(fqn.clone());
            (TypeKind::Enum(variants), Mode::Response)
        } else if let Some(arms) = self.union_arms(&schema, &name) {
            (TypeKind::Union(arms), Mode::Response)
        } else if schema.get("type").and_then(Value::as_str) == Some("object") || schema.get("properties").is_some() {
            if self.is_list_shape(&schema) {
                let items = self.list_item_type(&schema, &ctx, &name);
                (TypeKind::Alias(TypeRef::List(Box::new(items))), Mode::Response)
            } else if let Some(ap) = schema.get("additionalProperties") {
                let value = self.type_of(ap, &ctx, &format!("{name}Value"));
                (TypeKind::Map(value), Mode::Response)
            } else {
                let fields = self.struct_fields(&schema, &ctx, &name);
                (TypeKind::Struct(fields), Mode::Response)
            }
        } else {
            let ty = self.type_of(&schema, &ctx, &name);
            self.alias_targets.insert(fqn.clone(), ty.clone());
            (TypeKind::Alias(ty), Mode::Response)
        };

        self.types.push(TypeDef {
            name,
            fqn,
            module,
            docs: naming::doc_attribute(&docs),
            kind,
            mode,
            serialize: false,
            needs_default: false,
            needs_form_encode: false,
        });
    }

    /// Resolves `allOf` by merging members into a single object schema.
    fn resolve(&self, schema: &Value) -> Value {
        let Some(all_of) = schema.get("allOf").and_then(Value::as_array) else {
            return schema.clone();
        };
        let mut merged = schema.clone();
        let obj = merged.as_object_mut().expect("schema is an object");
        obj.remove("allOf");
        let mut properties = obj
            .get("properties")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        let mut required: Vec<Value> = obj
            .get("required")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for member in all_of {
            let resolved = match member.get("$ref").and_then(Value::as_str) {
                Some(reference) => {
                    let key = reference.rsplit('/').next().unwrap_or("");
                    self.schema(key).map(|s| self.resolve(s)).unwrap_or(Value::Null)
                }
                None => self.resolve(member),
            };
            if let Some(props) = resolved.get("properties").and_then(Value::as_object) {
                for (name, value) in props {
                    properties.entry(name.clone()).or_insert_with(|| value.clone());
                }
            }
            if let Some(req) = resolved.get("required").and_then(Value::as_array) {
                for item in req {
                    if !required.contains(item) {
                        required.push(item.clone());
                    }
                }
            }
        }
        obj.insert("properties".to_string(), Value::Object(properties));
        if !required.is_empty() {
            obj.insert("required".to_string(), Value::Array(required));
        }
        merged
    }

    fn enum_variants(&self, schema: &Value) -> Option<Vec<Variant>> {
        let values = schema.get("enum")?.as_array()?;
        if schema.get("type").and_then(Value::as_str) != Some("string")
            && !values.iter().all(|v| v.is_string() || v.is_null())
        {
            return None;
        }
        let mut variants = Vec::new();
        let mut seen = HashSet::new();
        for value in values {
            let Some(wire) = value.as_str() else {
                continue;
            };
            let rust_name = self.unique_variant(wire, &mut seen);
            variants.push(Variant {
                rust_name,
                wire: wire.to_string(),
                docs: String::new(),
            });
        }
        if variants.is_empty() { None } else { Some(variants) }
    }

    fn unique_variant(&self, wire: &str, seen: &mut HashSet<String>) -> String {
        let mut base = naming::pascal_case(wire);
        // `Self` is not a legal variant name; `Unknown` is reserved for the
        // generated catch-all variant.
        if base == "Self" {
            base = "SelfValue".to_string();
        } else if base == "Unknown" {
            base = "UnknownValue".to_string();
        }
        let mut candidate = base.clone();
        let mut index = 2;
        while seen.contains(&candidate) {
            candidate = format!("{base}{index}");
            index += 1;
        }
        seen.insert(candidate.clone());
        candidate
    }

    /// Builds union arms for a schema-level `anyOf`/`oneOf` of object refs.
    fn union_arms(&self, schema: &Value, hint: &str) -> Option<Vec<UnionArm>> {
        let branches = schema.get("anyOf").or_else(|| schema.get("oneOf"))?.as_array()?;
        let mut refs: Vec<String> = Vec::new();
        for branch in branches {
            if let Some(reference) = branch.get("$ref").and_then(Value::as_str) {
                let key = reference.rsplit('/').next().unwrap_or("").to_string();
                if !refs.contains(&key) {
                    refs.push(key);
                }
            }
        }
        if refs.len() < 2 || refs.len() > 40 {
            return None;
        }
        Some(self.build_union_arms(&refs, hint))
    }

    fn build_union_arms(&self, refs: &[String], _hint: &str) -> Vec<UnionArm> {
        let mut arms = Vec::new();
        let mut seen = HashSet::new();
        let mut seen_wires = HashSet::new();
        for key in refs {
            let Some(meta) = self.schema_meta.get(key) else {
                continue;
            };
            let wire = self
                .schema(key)
                .and_then(|schema| schema.pointer("/properties/object/enum/0"))
                .and_then(Value::as_str)
                .unwrap_or(key)
                .to_string();
            if !seen_wires.insert(wire.clone()) {
                continue;
            }
            let rust_name = {
                let mut base = naming::pascal_case(&wire);
                if base == "Self" {
                    base = "SelfValue".to_string();
                } else if base == "Other" {
                    base = "OtherValue".to_string();
                }
                let mut candidate = base.clone();
                let mut index = 2;
                while seen.contains(&candidate) {
                    candidate = format!("{base}{index}");
                    index += 1;
                }
                seen.insert(candidate.clone());
                candidate
            };
            arms.push(UnionArm {
                rust_name,
                wire,
                ty: TypeRef::Named(meta.fqn.clone()),
            });
        }
        arms
    }

    /// Registers a synthesized union type and returns its FQN.
    fn union_type(&mut self, refs: &[String], ctx: &Ctx, hint: &str) -> String {
        let arms = self.build_union_arms(refs, hint);
        let signature = union_signature(ctx, &arms);
        if let Some(existing) = self.dedup.get(&signature).cloned() {
            let TypeRef::Named(fqn) = self.register_alias(hint, ctx, existing) else {
                unreachable!("register_alias returns a named type")
            };
            return fqn;
        }
        let name = self.synthesize_name(&ctx.module, hint);
        let fqn = self.inline_fqn(&name, ctx);
        self.types.push(TypeDef {
            name,
            fqn: fqn.clone(),
            module: ctx.module.clone(),
            docs: String::new(),
            kind: TypeKind::Union(arms),
            mode: ctx.mode,
            serialize: false,
            needs_default: false,
            needs_form_encode: false,
        });
        if ctx.mode == Mode::Response {
            self.dedup.insert(signature, fqn.clone());
        }
        fqn
    }

    fn is_list_shape(&self, schema: &Value) -> bool {
        let Some(props) = schema.get("properties").and_then(Value::as_object) else {
            return false;
        };
        let has_data = props
            .get("data")
            .is_some_and(|d| d.get("type").and_then(Value::as_str) == Some("array"));
        if !has_data {
            return false;
        }
        // v1 lists carry `has_more`; v2 lists carry `next_page_url`.
        props
            .get("has_more")
            .is_some_and(|h| h.get("type").and_then(Value::as_str) == Some("boolean"))
            || props.contains_key("next_page_url")
    }

    fn list_item_type(&mut self, schema: &Value, ctx: &Ctx, hint: &str) -> TypeRef {
        let items = schema.pointer("/properties/data/items").cloned().unwrap_or(Value::Null);
        self.type_of(&items, ctx, &format!("{hint}Item"))
    }

    fn struct_fields(&mut self, schema: &Value, ctx: &Ctx, hint: &str) -> Vec<Field> {
        let mut fields = Vec::new();
        let required: HashSet<String> = schema
            .get("required")
            .and_then(Value::as_array)
            .map(|items| items.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
            .unwrap_or_default();
        let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
            return fields;
        };
        let mut names: Vec<&String> = properties.keys().collect();
        // Deterministic order: required fields first is not needed, but keep the
        // spec's property order by sorting for reproducibility.
        names.sort();
        for json_name in names {
            let property = &properties[json_name];
            let docs = property.get("description").and_then(Value::as_str).unwrap_or("");
            let rust_name = naming::field_ident(json_name);
            let mut ty = self.type_of(property, ctx, &format!("{hint}{}", naming::pascal_case(json_name)));
            let nullable = property.get("nullable").and_then(Value::as_bool).unwrap_or(false);
            let is_required = required.contains(json_name);
            // A response field whose type has no `Default` becomes optional even
            // when the specification calls it required: deleted objects and
            // specification drift omit fields, and a missing field must not fail
            // the whole response.
            let needs_option = nullable || !is_required || (ctx.mode == Mode::Response && !self.has_default(&ty));
            if needs_option {
                ty = TypeRef::Opt(Box::new(ty));
            }
            fields.push(Field {
                rust_name: rust_name.clone(),
                json_name: if rust_name == *json_name {
                    None
                } else {
                    Some(json_name.clone())
                },
                ty,
                docs: naming::doc_attribute(docs),
                boxed: false,
            });
        }
        fields
    }

    /// Maps a schema to a Rust type, synthesizing named types as needed.
    fn type_of(&mut self, schema: &Value, ctx: &Ctx, hint: &str) -> TypeRef {
        if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
            if let Some(fqn) = self.fqn_for_ref(reference) {
                return TypeRef::Named(fqn.to_string());
            }
            return TypeRef::Json;
        }

        // Expandable field: a string id or one of a set of object types.
        if let Some(expansion) = schema.get("x-expansionResources") {
            let refs = self.expansion_refs(expansion);
            if !refs.is_empty() {
                let inner = self.expandable_inner(&refs, ctx, hint);
                return TypeRef::Expandable(Box::new(inner));
            }
        }

        if let Some(branches) = schema
            .get("anyOf")
            .or_else(|| schema.get("oneOf"))
            .and_then(Value::as_array)
        {
            return self.union_branch_type(branches, ctx, hint);
        }

        if schema.get("allOf").is_some() {
            let resolved = self.resolve(schema);
            return self.type_of(&resolved, ctx, hint);
        }

        match schema.get("type").and_then(Value::as_str) {
            Some("object") | None if schema.get("properties").is_some() => {
                if self.is_list_shape(schema) {
                    let items = self.list_item_type(schema, ctx, hint);
                    TypeRef::List(Box::new(items))
                } else {
                    let fields = self.struct_fields(schema, ctx, hint);
                    self.register_inline_struct(hint, ctx, fields)
                }
            }
            Some("object") => {
                if let Some(ap) = schema.get("additionalProperties") {
                    if ap.as_bool() == Some(true) {
                        TypeRef::Map(Box::new(TypeRef::Json))
                    } else if ap.is_object() {
                        let value = self.type_of(ap, ctx, &format!("{hint}Value"));
                        TypeRef::Map(Box::new(value))
                    } else {
                        TypeRef::Json
                    }
                } else {
                    TypeRef::Json
                }
            }
            Some("array") => {
                let items = schema.get("items").cloned().unwrap_or(Value::Null);
                let item = self.type_of(&items, ctx, &format!("{hint}Item"));
                TypeRef::Vec(Box::new(item))
            }
            Some("string") => {
                if let Some(variants) = self.enum_variants(schema) {
                    self.register_inline_enum(hint, ctx, variants)
                } else {
                    TypeRef::Str
                }
            }
            Some("integer") => TypeRef::I64,
            Some("number") => TypeRef::F64,
            Some("boolean") => TypeRef::Bool,
            _ => {
                if let Some(variants) = self.enum_variants(schema) {
                    self.register_inline_enum(hint, ctx, variants)
                } else {
                    TypeRef::Json
                }
            }
        }
    }

    /// Returns true when values of `ty` implement [`Default`], so a missing or
    /// `null` response field can fall back to it instead of failing.
    fn has_default(&self, ty: &TypeRef) -> bool {
        let mut current = ty;
        for _ in 0..8 {
            match current {
                TypeRef::Str
                | TypeRef::Bool
                | TypeRef::I64
                | TypeRef::U64
                | TypeRef::F64
                | TypeRef::Unit
                | TypeRef::Vec(_)
                | TypeRef::Map(_)
                | TypeRef::Json
                | TypeRef::Opt(_) => return true,
                TypeRef::Named(fqn) => {
                    if self.enum_fqns.contains(fqn) {
                        return true;
                    }
                    match self.alias_targets.get(fqn) {
                        Some(target) => current = target,
                        None => return false,
                    }
                }
                TypeRef::List(_) | TypeRef::Expandable(_) => return false,
            }
        }
        false
    }

    fn expansion_refs(&self, expansion: &Value) -> Vec<String> {
        let mut refs = Vec::new();
        let branches = expansion
            .get("oneOf")
            .or_else(|| expansion.get("anyOf"))
            .and_then(Value::as_array);
        if let Some(branches) = branches {
            for branch in branches {
                if let Some(reference) = branch.get("$ref").and_then(Value::as_str) {
                    let key = reference.rsplit('/').next().unwrap_or("").to_string();
                    if !refs.contains(&key) {
                        refs.push(key);
                    }
                }
            }
        }
        refs
    }

    fn expandable_inner(&mut self, refs: &[String], ctx: &Ctx, hint: &str) -> TypeRef {
        if refs.len() == 1 {
            if let Some(meta) = self.schema_meta.get(&refs[0]) {
                return TypeRef::Named(meta.fqn.clone());
            }
            return TypeRef::Json;
        }
        let fqn = self.union_type(refs, ctx, &format!("{hint}Expanded"));
        TypeRef::Named(fqn)
    }

    fn union_branch_type(&mut self, branches: &[Value], ctx: &Ctx, hint: &str) -> TypeRef {
        let mut strings = 0usize;
        let mut refs: Vec<String> = Vec::new();
        let mut inline_objects: Vec<&Value> = Vec::new();
        for branch in branches {
            if let Some(reference) = branch.get("$ref").and_then(Value::as_str) {
                let key = reference.rsplit('/').next().unwrap_or("").to_string();
                if !refs.contains(&key) {
                    refs.push(key);
                }
            } else if branch.get("type").and_then(Value::as_str) == Some("string") {
                strings += 1;
            } else if branch.get("type").and_then(Value::as_str) == Some("object")
                || branch.get("properties").is_some()
                || branch.get("additionalProperties").is_some()
            {
                inline_objects.push(branch);
            }
        }

        if !refs.is_empty() && (strings > 0 || refs.len() > 1) {
            if refs.len() == 1 && strings > 0 {
                let meta = &self.schema_meta[&refs[0]];
                return TypeRef::Expandable(Box::new(TypeRef::Named(meta.fqn.clone())));
            }
            let fqn = self.union_type(&refs, ctx, &format!("{hint}Expanded"));
            let inner = TypeRef::Named(fqn);
            if strings > 0 {
                return TypeRef::Expandable(Box::new(inner));
            }
            return inner;
        }
        if refs.len() == 1 {
            let meta = &self.schema_meta[&refs[0]];
            return TypeRef::Named(meta.fqn.clone());
        }
        if inline_objects.len() == 1 {
            return self.type_of(inline_objects[0], ctx, hint);
        }
        TypeRef::Json
    }

    fn register_inline_struct(&mut self, hint: &str, ctx: &Ctx, fields: Vec<Field>) -> TypeRef {
        let signature = struct_signature(ctx, &fields);
        if let Some(existing) = self.dedup.get(&signature).cloned() {
            return self.register_alias(hint, ctx, existing);
        }
        let name = self.synthesize_name(&ctx.module, hint);
        let fqn = self.inline_fqn(&name, ctx);
        self.types.push(TypeDef {
            name,
            fqn: fqn.clone(),
            module: ctx.module.clone(),
            docs: String::new(),
            kind: TypeKind::Struct(fields),
            mode: ctx.mode,
            serialize: ctx.mode == Mode::Json,
            needs_default: false,
            needs_form_encode: false,
        });
        self.dedup.insert(signature, fqn.clone());
        TypeRef::Named(fqn)
    }

    /// Emits a type alias so a deduplicated type keeps its per-endpoint name.
    ///
    /// Response types do not need that name: they appear only as field types, so
    /// the canonical type is used directly and no alias is generated.
    fn register_alias(&mut self, hint: &str, ctx: &Ctx, target: String) -> TypeRef {
        if ctx.mode == Mode::Response {
            return TypeRef::Named(target);
        }
        let name = self.synthesize_name(&ctx.module, hint);
        let fqn = self.inline_fqn(&name, ctx);
        self.alias_targets.insert(fqn.clone(), TypeRef::Named(target.clone()));
        self.types.push(TypeDef {
            name,
            fqn: fqn.clone(),
            module: ctx.module.clone(),
            docs: String::new(),
            kind: TypeKind::Alias(TypeRef::Named(target)),
            mode: ctx.mode,
            serialize: false,
            needs_default: false,
            needs_form_encode: false,
        });
        TypeRef::Named(fqn)
    }

    fn register_inline_enum(&mut self, hint: &str, ctx: &Ctx, variants: Vec<Variant>) -> TypeRef {
        let signature = enum_signature(ctx, &variants);
        if let Some(existing) = self.dedup.get(&signature).cloned() {
            return self.register_alias(hint, ctx, existing);
        }
        let name = self.synthesize_name(&ctx.module, hint);
        let fqn = self.inline_fqn(&name, ctx);
        self.enum_fqns.insert(fqn.clone());
        self.types.push(TypeDef {
            name,
            fqn: fqn.clone(),
            module: ctx.module.clone(),
            docs: String::new(),
            kind: TypeKind::Enum(variants),
            mode: ctx.mode,
            serialize: ctx.mode == Mode::Json,
            needs_default: false,
            needs_form_encode: false,
        });
        self.dedup.insert(signature, fqn.clone());
        TypeRef::Named(fqn)
    }

    fn inline_fqn(&self, name: &str, ctx: &Ctx) -> String {
        format!("crate::{}::{name}", ctx.module)
    }

    fn build_methods(&mut self) {
        let paths = self
            .spec
            .pointer("/paths")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        let mut keys: Vec<String> = paths.keys().cloned().collect();
        keys.sort();
        for path in keys {
            let Some(item) = paths.get(&path) else {
                continue;
            };
            for (verb, verb_name) in [(Verb::Get, "get"), (Verb::Post, "post"), (Verb::Delete, "delete")] {
                let Some(op) = item.get(verb_name) else {
                    continue;
                };
                self.build_method(&path, verb, op);
            }
        }
    }

    /// The schema keys an operation's 200 response can decode to.
    ///
    /// Most responses are a single `$ref`; some (for example `GET
    /// /v1/customers/{customer}`) are a union of the live and deleted shapes.
    fn response_refs(&self, op: &Value) -> Vec<String> {
        let Some(schema) = op
            .get("responses")
            .and_then(|responses| responses.get("200"))
            .and_then(|response| response.get("content"))
            .and_then(|content| content.get("application/json"))
            .and_then(|media| media.get("schema"))
        else {
            return Vec::new();
        };
        let mut refs = Vec::new();
        push_ref(schema, &mut refs);
        for branch in schema
            .get("anyOf")
            .or_else(|| schema.get("oneOf"))
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            push_ref(branch, &mut refs);
        }
        refs
    }

    /// The resources a sub-path acts on: the types returned by `GET` on the path
    /// with its last segment removed.
    fn parent_refs(&self, path: &str) -> Vec<String> {
        let mut segments = naming::path_segments(path);
        if segments.pop().is_none() {
            return Vec::new();
        }
        let pointer = format!("/paths/{}/get", json_pointer(&format!("/{}", segments.join("/"))));
        let Some(op) = self.spec.pointer(&pointer) else {
            return Vec::new();
        };
        self.response_refs(op)
    }

    /// Returns true when a trailing fixed path segment names an operation rather
    /// than a resource, so `/v1/payment_intents/{intent}/confirm` becomes
    /// `confirm_payment_intent` instead of creating a `confirm` resource.
    ///
    /// The rules, in order: an operation that returns the resource it acts on is
    /// an action; a path that also serves `GET` addresses a resource; a plural
    /// trailing segment names a sub-collection; anything else is an action.
    fn is_action(&self, path: &str, op: &Value, last: &str) -> bool {
        let parents = self.parent_refs(path);
        let responses = self.response_refs(op);
        if parents.iter().any(|parent| responses.contains(parent)) {
            return true;
        }
        let pointer = format!("/paths/{}/get", json_pointer(path));
        if self.spec.pointer(&pointer).is_some() {
            return false;
        }
        !is_plural(last)
    }

    fn build_method(&mut self, path: &str, verb: Verb, op: &Value) {
        let module = naming::module_of_path(path);
        self.modules.insert(module.clone());

        let segments: Vec<String> = naming::path_segments(path).into_iter().skip(1).collect();
        let last = segments.last().cloned().unwrap_or_default();
        let last_is_placeholder = naming::is_placeholder(&last);

        // Name the operation: a verb (`get`, `list`, `create`, `update`,
        // `delete`, `search`) or a trailing action segment.
        let mut dropped_segment: Option<String> = None;
        let mut verb_word = String::new();
        if verb == Verb::Post && !last_is_placeholder && segments.len() > 1 && self.is_action(path, op, &last) {
            verb_word.clone_from(&last);
            dropped_segment = Some(last.clone());
        } else if verb == Verb::Get && last == "search" {
            verb_word = "search".to_string();
            dropped_segment = Some(last.clone());
        } else {
            let response = op
                .get("responses")
                .and_then(|responses| responses.get("200"))
                .and_then(|response| response.get("content"))
                .and_then(|content| content.get("application/json"))
                .and_then(|media| media.get("schema"));
            verb_word = match verb {
                Verb::Get => {
                    if response.is_some_and(|schema| self.is_list_shape(&self.resolve(schema))) {
                        "list"
                    } else {
                        "get"
                    }
                }
                Verb::Post if last_is_placeholder => "update",
                Verb::Post if is_plural(&last) => "create",
                Verb::Post => "update",
                Verb::Delete => "delete",
            }
            .to_string();
        }

        let chain = naming::chain_of_segments(&segments, dropped_segment.as_deref());
        // A multi-word action already reads as a whole operation
        // (`confirm_payment_intent`); a single word is prefixed to the resource
        // it acts on (`expire_checkout_session`).
        let multi_word_action = dropped_segment.is_some() && verb_word.contains('_');
        let mut function = if multi_word_action || chain.is_empty() {
            verb_word.clone()
        } else {
            format!("{verb_word}_{chain}")
        };
        if verb_word == "list" || verb_word == "search" {
            function = pluralize_last(&function);
        }

        let docs = op
            .get("summary")
            .or_else(|| op.get("description"))
            .and_then(Value::as_str)
            .unwrap_or("");

        // Collect parameters.
        let mut path_param_names: HashMap<String, String> = HashMap::new();
        let mut query_properties = serde_json::Map::new();
        let mut query_required: Vec<Value> = Vec::new();
        if let Some(parameters) = op.get("parameters").and_then(Value::as_array) {
            for parameter in parameters {
                let name = parameter.get("name").and_then(Value::as_str).unwrap_or("");
                let location = parameter.get("in").and_then(Value::as_str).unwrap_or("");
                match location {
                    "path" => {
                        path_param_names.insert(name.to_string(), naming::field_ident(name));
                    }
                    "query" => {
                        if let Some(schema) = parameter.get("schema") {
                            query_properties.insert(name.to_string(), schema.clone());
                            if parameter.get("required").and_then(Value::as_bool) == Some(true) {
                                query_required.push(Value::String(name.to_string()));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        // Every `{placeholder}` in the template becomes a function argument.
        let mut path_args: Vec<(String, String)> = Vec::new();
        for wire in path_placeholders(path) {
            let rust = path_param_names
                .get(&wire)
                .cloned()
                .unwrap_or_else(|| naming::field_ident(&wire));
            path_args.push((wire, rust));
        }

        // Request body.
        let mut json = false;
        let mut body_properties = serde_json::Map::new();
        let mut body_required: Vec<Value> = Vec::new();
        let mut multipart = false;
        if let Some(request_body) = op.get("requestBody") {
            let content = request_body.get("content").and_then(Value::as_object);
            if let Some(content) = content {
                if content.contains_key("multipart/form-data")
                    && !content.contains_key("application/x-www-form-urlencoded")
                {
                    multipart = true;
                }
                let (content_type, media) = if let Some(media) = content.get("application/x-www-form-urlencoded") {
                    ("form", media)
                } else if let Some(media) = content.get("application/json") {
                    ("json", media)
                } else {
                    ("other", content.values().next().unwrap_or(&Value::Null))
                };
                if content_type == "json" {
                    json = true;
                }
                if let Some(schema) = media.get("schema") {
                    let resolved = self.resolve(schema);
                    if let Some(props) = resolved.get("properties").and_then(Value::as_object) {
                        for (name, value) in props {
                            body_properties.insert(name.clone(), value.clone());
                        }
                    }
                    if let Some(req) = resolved.get("required").and_then(Value::as_array) {
                        body_required.extend(req.iter().cloned());
                    }
                }
            }
        }

        let mode = if json { Mode::Json } else { Mode::Form };
        let ctx = Ctx {
            module: module.clone(),
            mode,
        };

        // For GET requests the query parameters are the body; for others the
        // requestBody is. Combine both when present.
        let mut merged = serde_json::Map::new();
        for (name, value) in &query_properties {
            merged.insert(name.clone(), value.clone());
        }
        for (name, value) in &body_properties {
            merged.insert(name.clone(), value.clone());
        }
        let mut required_list = query_required.clone();
        required_list.extend(body_required.clone());
        let merged_schema = Value::Object({
            let mut object = serde_json::Map::new();
            object.insert("type".to_string(), Value::String("object".to_string()));
            object.insert("properties".to_string(), Value::Object(merged));
            if !required_list.is_empty() {
                object.insert("required".to_string(), Value::Array(required_list));
            }
            object
        });

        let params_type = if merged_schema
            .get("properties")
            .and_then(Value::as_object)
            .is_some_and(|props| !props.is_empty())
        {
            let hint = format!("{function}Params");
            let fields = self.struct_fields(&merged_schema, &ctx, &hint);
            let TypeRef::Named(fqn) = self.register_inline_struct(&hint, &ctx, fields) else {
                unreachable!()
            };
            Some(fqn)
        } else {
            None
        };

        // Response type.
        let response = op
            .get("responses")
            .and_then(|responses| responses.get("200"))
            .and_then(|response| response.get("content"))
            .and_then(|content| content.get("application/json"))
            .and_then(|media| media.get("schema"))
            .cloned()
            .unwrap_or(Value::Null);
        let response_ctx = Ctx {
            module: module.clone(),
            mode: Mode::Response,
        };
        let response_ty = if response.is_null() {
            TypeRef::Unit
        } else {
            self.type_of(&response, &response_ctx, &format!("{}Response", naming::pascal_case(&function)))
        };

        self.methods.push(Method {
            module,
            template: path.to_string(),
            verb,
            json,
            function,
            params_type,
            path_args,
            response: response_ty,
            docs: naming::doc_attribute(docs),
            multipart,
        });
    }
}

/// Collects every generated type a type reference can reach.
fn type_refs(ty: &TypeRef, out: &mut HashSet<String>) {
    match ty {
        TypeRef::Named(fqn) => {
            out.insert(fqn.clone());
        }
        TypeRef::Opt(inner)
        | TypeRef::Vec(inner)
        | TypeRef::Map(inner)
        | TypeRef::List(inner)
        | TypeRef::Expandable(inner) => {
            type_refs(inner, out);
        }
        _ => {}
    }
}

/// Collects every generated type a definition can reach.
fn type_def_refs(ty: &TypeDef, out: &mut HashSet<String>) {
    match &ty.kind {
        TypeKind::Struct(fields) => {
            for field in fields {
                type_refs(&field.ty, out);
            }
        }
        TypeKind::Alias(target) | TypeKind::Map(target) => type_refs(target, out),
        TypeKind::Union(arms) => {
            for arm in arms {
                type_refs(&arm.ty, out);
            }
        }
        TypeKind::Enum(_) => {}
    }
}

/// Escapes a JSON pointer token for `serde_json::Value::pointer`.
fn json_pointer(path: &str) -> String {
    path.replace('~', "~0").replace('/', "~1")
}

/// Appends the schema key a `$ref` points at, if any.
fn push_ref(schema: &Value, out: &mut Vec<String>) {
    if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
        out.push(reference.rsplit('/').next().unwrap_or("").to_string());
    }
}

/// Returns true when `segment` is a plural resource name.
fn is_plural(segment: &str) -> bool {
    segment.ends_with('s') && !segment.ends_with("ss")
}

/// Pluralizes the last word of a snake_case identifier (`list_customer` ->
/// `list_customers`).
fn pluralize_last(name: &str) -> String {
    match name.rsplit_once('_') {
        Some((head, tail)) => format!("{head}_{}", naming::plural(tail)),
        None => naming::plural(name),
    }
}

/// Returns the type name referenced by a type, through any transparent
/// wrappers (`Option`, `Vec`, `HashMap`, `List`, `Expandable`).
fn direct_named(ty: &TypeRef) -> Option<&str> {
    match ty {
        TypeRef::Named(name) => Some(name.as_str()),
        TypeRef::Opt(inner)
        | TypeRef::Vec(inner)
        | TypeRef::Map(inner)
        | TypeRef::List(inner)
        | TypeRef::Expandable(inner) => direct_named(inner),
        _ => None,
    }
}

fn struct_signature(ctx: &Ctx, fields: &[Field]) -> String {
    let mut out = format!("struct|{:?}|{}", ctx.mode, ctx.module);
    for field in fields {
        out.push_str(&format!(
            "|{}:{}:{}",
            field.rust_name,
            field.ty.render(),
            field.json_name.as_deref().unwrap_or("")
        ));
    }
    out
}

fn enum_signature(ctx: &Ctx, variants: &[Variant]) -> String {
    let mut out = format!("enum|{:?}|{}", ctx.mode, ctx.module);
    for variant in variants {
        out.push_str(&format!("|{}={}", variant.rust_name, variant.wire));
    }
    out
}

fn union_signature(ctx: &Ctx, arms: &[UnionArm]) -> String {
    let mut out = format!("union|{:?}|{}", ctx.mode, ctx.module);
    for arm in arms {
        out.push_str(&format!("|{}={}={}", arm.rust_name, arm.wire, arm.ty.render()));
    }
    out
}

/// Returns the struct name directly reachable from a field type, ignoring
/// indirection (`Vec`, `HashMap`, `Box`, `Expandable`) that already breaks
/// size cycles.
fn direct_struct_target(ty: &TypeRef) -> Option<&str> {
    match ty {
        TypeRef::Named(name) => Some(name.as_str()),
        TypeRef::Opt(inner) => direct_struct_target(inner),
        _ => None,
    }
}

/// Iterative Kosaraju strongly-connected-components, returning a component id
/// per node. Iterative to avoid deep recursion on large graphs.
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
