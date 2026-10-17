//! Emits Rust source from the model.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::{
    ir::{Field, Mode, TypeDef, TypeKind, TypeRef, UnionArm},
    schema::{Method, Model, Verb},
};

/// A generated file: path relative to the crate root and its contents.
pub struct OutputFile {
    /// Relative path, e.g. `api/customers.rs`.
    pub path: String,
    /// File contents.
    pub contents: String,
}

/// `true` when every type in the module is always compiled.
pub const GENERATED_DIR: &str = "api";

/// Modules whose declaration is gated by their feature.
///
/// Their contents are unreachable from the rest of the crate, so leaving them
/// out of a build costs nothing: `events` holds the typed webhook payloads,
/// which no endpoint returns.
pub const GATED_MODULES: &[&str] = &["events"];

/// The marker lines that bracket the generated module declarations in the
/// crate root. Everything between them is rewritten by the generator.
pub const MODULES_BEGIN: &str = "// region: generated modules (do not edit; `make -C stripe codegen`)";
/// End marker; see [`MODULES_BEGIN`].
pub const MODULES_END: &str = "// endregion: generated modules";

/// The longest `any(...)` feature list a gated response type may carry.
///
/// This is a safety valve, not a tuning knob: the widest set the specification
/// produces is 54 of the 79 modules. Reachability gating costs nothing
/// measurable on a default build — the long lists only appear on shared types
/// that such a build compiles anyway — and it is what makes a
/// `--no-default-features` or single-module build cheap.
pub const MAX_CFG_FEATURES: usize = 64;

struct Emitter<'a> {
    types: HashMap<&'a str, &'a TypeDef>,
    /// For each type FQN, the modules whose methods can reach it.
    gating: HashMap<&'a str, &'a BTreeSet<String>>,
}

/// Turns a model into files.
pub fn emit(model: &Model) -> Vec<OutputFile> {
    let types: HashMap<&str, &TypeDef> = model.types.iter().map(|t| (t.fqn.as_str(), t)).collect();
    let gating: HashMap<&str, &BTreeSet<String>> = model
        .gating
        .iter()
        .map(|(fqn, modules)| (fqn.as_str(), modules))
        .collect();
    let emitter = Emitter {
        types,
        gating,
    };

    let mut types_by_module: BTreeMap<String, Vec<&TypeDef>> = BTreeMap::new();
    for ty in &model.types {
        types_by_module.entry(ty.module.clone()).or_default().push(ty);
    }

    let mut methods_by_module: BTreeMap<String, Vec<&Method>> = BTreeMap::new();
    for method in &model.methods {
        methods_by_module.entry(method.module.clone()).or_default().push(method);
    }

    let mut modules: BTreeSet<String> = model.modules.clone();
    modules.extend(types_by_module.keys().cloned());
    modules.extend(methods_by_module.keys().cloned());
    modules.insert("common".to_string());

    let mut files = Vec::new();
    files.push(emitter.emit_modules_block(&modules));
    for module in &modules {
        let empty_types = Vec::new();
        let empty_methods = Vec::new();
        let module_types = types_by_module.get(module).unwrap_or(&empty_types);
        let module_methods = methods_by_module.get(module).unwrap_or(&empty_methods);
        files.push(OutputFile {
            path: format!("{GENERATED_DIR}/{module}.rs"),
            contents: emitter.emit_module(module, module_types, module_methods),
        });
    }
    files
}

impl Emitter<'_> {
    /// The block of `#[path]` module declarations spliced into the crate root.
    fn emit_modules_block(&self, modules: &BTreeSet<String>) -> OutputFile {
        let mut out = String::new();
        out.push_str(MODULES_BEGIN);
        out.push('\n');
        out.push_str("//\n");
        out.push_str("// Every module is also a cargo feature of the same name: it gates the request\n");
        out.push_str("// builders and methods for that part of the API. Response types are gated by\n");
        out.push_str("// reachability instead: a type is compiled only when one of the features on its\n");
        out.push_str("// `#[cfg]` (the modules whose methods can reach it) is enabled. The modules in\n");
        out.push_str("// `GATED_MODULES` only application code can reach, so they are gated as a whole.\n");
        for module in modules {
            if GATED_MODULES.contains(&module.as_str()) {
                out.push_str(&format!("#[cfg(feature = \"{module}\")]\n"));
            }
            out.push_str(&format!("#[path = \"{GENERATED_DIR}/{module}.rs\"]\n"));
            out.push_str(&format!("pub mod {module};\n"));
        }
        out.push_str(MODULES_END);
        out.push('\n');
        OutputFile {
            path: "modules.rs".to_string(),
            contents: out,
        }
    }

    /// The `#[cfg]` lines that gate every item of `ty` (empty when the type is
    /// always compiled).
    ///
    /// A response type is compiled only when a module that can reach it is
    /// enabled; a request type is compiled with its own module, which is what
    /// gates its methods too.
    fn cfg_for(&self, ty: &TypeDef) -> String {
        if ty.mode != Mode::Response {
            return format!("#[cfg(feature = \"{}\")]\n", ty.module);
        }
        let Some(modules) = self.gating.get(ty.fqn.as_str()) else {
            // Unreachable from every method: unreachable types are pruned, so
            // this only happens if a future generator change drops the pruning.
            return String::new();
        };
        if modules.len() > MAX_CFG_FEATURES {
            // Safety valve for a specification that grows far beyond today's 79
            // modules; see MAX_CFG_FEATURES. Any set the vendored spec produces
            // is gated exactly.
            return String::new();
        }
        let mut features: Vec<String> = modules.iter().map(|module| format!("feature = \"{module}\"")).collect();
        features.sort();
        match features.len() {
            0 => String::new(),
            1 => format!("#[cfg({})]\n", features[0]),
            _ => format!("#[cfg(any({}))]\n", features.join(", ")),
        }
    }

    fn emit_module(&self, module: &str, types: &[&TypeDef], methods: &[&Method]) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "//! The `{module}` part of the Stripe API. Generated by `stripe_codegen`; do not edit.\n//!\n"
        ));
        out.push_str(
            "//! A type is compiled when one of the cargo features that can reach it (listed on the\n\
             //! `#[cfg]` above it) is enabled; the request parameter types and methods in this module\n\
             //! require its own feature.\n\n",
        );
        out.push_str("#![allow(missing_docs)]\n#![allow(clippy::all)]\n#![allow(non_camel_case_types)]\n\n");

        // Response types are gated by reachability: only the types an enabled
        // module can actually return are compiled. Request types and methods are
        // gated by their own module's feature. Every emitted item (a struct and
        // its `impl` blocks alike) needs its own `#[cfg]`: the attribute only
        // applies to the item it precedes.
        let mut sorted: Vec<&&TypeDef> = types.iter().collect();
        sorted.sort_by(|a, b| a.name.cmp(&b.name));
        for ty in &sorted {
            let items = self.emit_type_items(ty);
            let gate = self.cfg_for(ty);
            for item in items {
                out.push_str(&gate);
                out.push_str(&item);
                out.push('\n');
            }
        }

        let mut sorted_methods: Vec<&&Method> = methods.iter().collect();
        sorted_methods.sort_by(|a, b| a.function.cmp(&b.function));
        for method in sorted_methods {
            out.push_str(&format!("#[cfg(feature = \"{module}\")]\n"));
            out.push_str(&self.emit_method(method));
            out.push('\n');
        }
        out
    }

    fn emit_type_items(&self, ty: &TypeDef) -> Vec<String> {
        let mut header = String::new();
        if !ty.docs.is_empty() {
            header.push_str(&format!("#[doc = \"{}\"]\n", ty.docs));
        }
        match &ty.kind {
            TypeKind::Struct(fields) => self.emit_struct(ty, fields),
            TypeKind::Enum(variants) => self.emit_enum(ty, variants),
            TypeKind::Alias(target) => {
                vec![format!("{header}pub type {} = {};\n", ty.name, target.render())]
            }
            TypeKind::Map(value) => {
                vec![format!(
                    "{header}pub type {} = std::collections::HashMap<String, {}>;\n",
                    ty.name,
                    value.render()
                )]
            }
            TypeKind::Union(arms) => self.emit_union(ty, arms),
        }
    }

    fn emit_struct(&self, ty: &TypeDef, fields: &[Field]) -> Vec<String> {
        let mut out = String::new();
        // Copy so we can comma-separate the derive list.
        let mut derives = String::from("Debug, Clone");
        match ty.mode {
            Mode::Response => {
                derives.push_str(", serde::Deserialize");
                if ty.serialize {
                    derives.push_str(", serde::Serialize");
                }
            }
            Mode::Json => derives.push_str(", serde::Serialize"),
            Mode::Form => {}
        }
        let all_defaultable = fields.iter().all(|f| self.is_defaultable(&f.ty));
        if ty.mode != Mode::Response && all_defaultable {
            derives.push_str(", Default");
        }
        out.push_str(&format!("#[derive({derives})]\n"));
        out.push_str(&format!("pub struct {} {{\n", ty.name));
        for field in fields {
            if !field.docs.is_empty() {
                out.push_str(&format!("    #[doc = \"{}\"]\n", field.docs));
            }
            if ty.mode != Mode::Form {
                let mut attributes = Vec::new();
                if let Some(json_name) = &field.json_name {
                    attributes.push(format!("rename = \"{json_name}\""));
                }
                if ty.mode == Mode::Response && self.field_defaults(&field.ty) {
                    // A field the response omits decodes to its default. Enums do
                    // it themselves when the value is explicitly `null`.
                    attributes.push("default".to_string());
                }
                if matches!(field.ty, TypeRef::Opt(_)) {
                    attributes.push("skip_serializing_if = \"Option::is_none\"".to_string());
                }
                if !attributes.is_empty() {
                    out.push_str(&format!("    #[serde({})]\n", attributes.join(", ")));
                }
            }
            out.push_str(&format!("    pub {}: {},\n", field.rust_name, field.rendered_type()));
        }
        out.push_str("}\n");

        let mut items = vec![out];
        if ty.mode == Mode::Form {
            items.push(self.emit_form_encode(ty, fields));
        }
        // Resource types with an `id` can be paginated.
        if ty.mode == Mode::Response {
            let has_string_id = fields
                .iter()
                .any(|field| field.rust_name == "id" && matches!(field.ty, TypeRef::Str));
            if has_string_id {
                items.push(format!(
                    "impl crate::PagedItem for {} {{\n    fn id(&self) -> &str {{\n        &self.id\n    }}\n}}\n",
                    ty.name
                ));
            }
        }
        items
    }

    fn emit_form_encode(&self, ty: &TypeDef, fields: &[Field]) -> String {
        let mut out = String::new();
        let mut body = String::new();
        for field in fields {
            body.push_str(&self.emit_field_encode(&ty.module, field));
        }
        out.push_str(&format!(
            "impl crate::params::FormEncode for {} {{\n    fn encode_form(&self, form: &mut crate::params::FormBuilder) {{\n",
            ty.name
        ));
        if body.is_empty() {
            out.push_str("        let _ = form;\n");
        } else {
            out.push_str(&body);
        }
        out.push_str("    }\n}\n");
        out
    }

    fn emit_field_encode(&self, module: &str, field: &Field) -> String {
        let name = field.json_name.clone().unwrap_or_else(|| field.rust_name.clone());
        let accessor = format!("self.{}", field.rust_name);
        let mut out = String::new();
        match &field.ty {
            TypeRef::Opt(inner) => match inner.as_ref() {
                TypeRef::Vec(element) => {
                    let body = self.emit_vec_encode(module, "            ", &name, element, "values");
                    if !body.is_empty() {
                        out.push_str(&format!("        if let Some(values) = {accessor}.as_ref() {{\n"));
                        out.push_str(&body);
                        out.push_str("        }\n");
                    }
                }
                TypeRef::Map(value) => {
                    let body = self.emit_map_encode("            ", &name, value, "values");
                    if !body.is_empty() {
                        out.push_str(&format!("        if let Some(values) = {accessor}.as_ref() {{\n"));
                        out.push_str(&body);
                        out.push_str("        }\n");
                    }
                }
                TypeRef::Json | TypeRef::Expandable(_) => {
                    // Not representable as a v1 parameter; omit rather than
                    // emit an incorrect value.
                }
                _ => {
                    out.push_str(&format!("        form.field_opt(\"{name}\", {accessor}.as_ref());\n"));
                }
            },
            TypeRef::Vec(element) => {
                out.push_str(&self.emit_vec_encode(module, "        ", &name, element, &accessor));
            }
            TypeRef::Map(value) => {
                out.push_str(&self.emit_map_encode("        ", &name, value, &accessor));
            }
            TypeRef::Json | TypeRef::Expandable(_) => {}
            _ => {
                out.push_str(&format!("        form.field(\"{name}\", &{accessor});\n"));
            }
        }
        out
    }

    fn emit_vec_encode(&self, module: &str, indent: &str, name: &str, element: &TypeRef, expr: &str) -> String {
        let mut out = String::new();
        if self.is_scalar(element) {
            // v1 arrays use `name[]=`, v2 query arrays repeat the key
            // (`include=a&include=b`).
            let method = if module.starts_with("v2_") {
                "field_repeated"
            } else {
                "field_array"
            };
            out.push_str(&format!("{indent}form.{method}(\"{name}\", {expr}.as_slice());\n"));
        } else if matches!(element, TypeRef::Named(_)) {
            out.push_str(&format!(
                "{indent}for (index, item) in {expr}.iter().enumerate() {{\n{indent}    form.field_indexed(\"{name}\", index, item);\n{indent}}}\n"
            ));
        } else {
            // Nested containers are not encodable as v1 parameters.
        }
        out
    }

    fn emit_map_encode(&self, indent: &str, name: &str, value: &TypeRef, expr: &str) -> String {
        let mut out = String::new();
        if self.is_scalar(value) || matches!(value, TypeRef::Named(_)) {
            out.push_str(&format!(
                "{indent}for (key, value) in {expr}.iter() {{\n{indent}    form.field_map(\"{name}\", key, value);\n{indent}}}\n"
            ));
        }
        out
    }

    /// Whether a value of this type always has a `Default`, which is what lets a
    /// request type derive `Default`.
    fn is_defaultable(&self, ty: &TypeRef) -> bool {
        match ty {
            TypeRef::Opt(_)
            | TypeRef::Str
            | TypeRef::Bool
            | TypeRef::I64
            | TypeRef::U64
            | TypeRef::F64
            | TypeRef::Unit
            | TypeRef::Vec(_)
            | TypeRef::Map(_)
            | TypeRef::Json => true,
            // Generated enums implement `Default` when a response field needs it,
            // but the flag is not known here, so request types never rely on it.
            TypeRef::Named(_) | TypeRef::List(_) | TypeRef::Expandable(_) => false,
        }
    }

    /// Whether a response field of this type can fall back to a default when the
    /// payload omits it.
    fn field_defaults(&self, ty: &TypeRef) -> bool {
        self.is_defaultable(ty) || self.resolves_to_enum(ty)
    }

    /// Whether `ty` names a generated enum, following type aliases.
    fn resolves_to_enum(&self, ty: &TypeRef) -> bool {
        let TypeRef::Named(fqn) = ty else {
            return false;
        };
        let mut current = fqn.as_str();
        for _ in 0..8 {
            let Some(candidate) = self.types.get(current) else {
                return false;
            };
            match &candidate.kind {
                TypeKind::Enum(_) => return true,
                TypeKind::Alias(target) => {
                    let TypeRef::Named(next) = target else {
                        return false;
                    };
                    current = next.as_str();
                }
                _ => return false,
            }
        }
        false
    }

    fn is_scalar(&self, ty: &TypeRef) -> bool {
        match ty {
            TypeRef::Str | TypeRef::Bool | TypeRef::I64 | TypeRef::U64 | TypeRef::F64 | TypeRef::Unit => true,
            TypeRef::Named(fqn) => self
                .types
                .get(fqn.as_str())
                .is_some_and(|t| matches!(t.kind, TypeKind::Enum(_))),
            _ => false,
        }
    }

    fn emit_enum(&self, ty: &TypeDef, variants: &[crate::ir::Variant]) -> Vec<String> {
        let mut items = Vec::new();
        let mut out = String::new();
        out.push_str("#[derive(Debug, Clone, PartialEq)]\n");
        out.push_str(&format!("pub enum {} {{\n", ty.name));
        for variant in variants {
            if !variant.docs.is_empty() {
                out.push_str(&format!("    #[doc = \"{}\"]\n", variant.docs));
            }
            out.push_str(&format!("    {},\n", variant.rust_name));
        }
        out.push_str("    /// A value not known to this version of the crate.\n");
        out.push_str("    Unknown(String),\n");
        out.push_str("}\n");
        items.push(std::mem::take(&mut out));

        // as_str
        out.push_str(&format!("impl {} {{\n", ty.name));
        out.push_str("    /// Returns the value as it appears on the wire.\n");
        out.push_str("    pub fn as_str(&self) -> &str {\n        match self {\n");
        for variant in variants {
            out.push_str(&format!(
                "            {}::{} => \"{}\",\n",
                ty.name, variant.rust_name, variant.wire
            ));
        }
        out.push_str(&format!("            {}::Unknown(value) => value,\n", ty.name));
        out.push_str("        }\n    }\n}\n");
        items.push(std::mem::take(&mut out));

        // A response field that the payload omits decodes to `Default`, so the
        // impl exists exactly where a field needs it.
        if ty.needs_default {
            out.push_str(&format!(
                "impl Default for {} {{\n    fn default() -> Self {{\n        {}::Unknown(String::new())\n    }}\n}}\n",
                ty.name, ty.name
            ));
            items.push(std::mem::take(&mut out));
        }

        // Only types a JSON (v2) request can reference are ever serialized.
        if ty.serialize {
            out.push_str(&format!(
                "impl serde::Serialize for {} {{\n    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>\n    where\n        S: serde::Serializer,\n    {{\n        serializer.serialize_str(self.as_str())\n    }}\n}}\n",
                ty.name
            ));
            items.push(std::mem::take(&mut out));
        }

        // Deserialize (tolerant: unknown values are preserved).
        out.push_str(&format!("impl<'de> serde::Deserialize<'de> for {} {{\n", ty.name));
        out.push_str("    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>\n    where\n        D: serde::Deserializer<'de>,\n    {\n");
        out.push_str("        struct EnumVisitor;\n");
        out.push_str("        impl<'de> serde::de::Visitor<'de> for EnumVisitor {\n");
        out.push_str(&format!("            type Value = {};\n", ty.name));
        out.push_str("            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n                formatter.write_str(\"a string\")\n            }\n");
        out.push_str("            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>\n            where\n                E: serde::de::Error,\n            {\n                Ok(match value {\n");
        for variant in variants {
            out.push_str(&format!(
                "                    \"{}\" => {}::{},\n",
                variant.wire, ty.name, variant.rust_name
            ));
        }
        out.push_str(&format!(
            "                    other => {}::Unknown(other.to_string()),\n                }})\n            }}\n\n            fn visit_unit<E>(self) -> Result<Self::Value, E>\n            where\n                E: serde::de::Error,\n            {{\n                // An explicit `null` is treated like a missing value.\n                Ok({}::Unknown(String::new()))\n            }}\n        }}\n        deserializer.deserialize_any(EnumVisitor)\n    }}\n}}\n",
            ty.name, ty.name
        ));
        items.push(std::mem::take(&mut out));

        // v1 request bodies need the value in their encoded form.
        if ty.needs_form_encode {
            out.push_str(&format!(
                "impl crate::params::FormEncode for {} {{\n    fn encode_form(&self, form: &mut crate::params::FormBuilder) {{\n        form.set_str(self.as_str());\n    }}\n}}\n",
                ty.name
            ));
            items.push(std::mem::take(&mut out));
        }
        items
    }

    fn emit_union(&self, ty: &TypeDef, arms: &[UnionArm]) -> Vec<String> {
        let mut items = Vec::new();
        let mut out = String::new();
        out.push_str("#[derive(Debug, Clone)]\n");
        out.push_str(&format!("pub enum {} {{\n", ty.name));
        for arm in arms {
            out.push_str(&format!("    /// Expanded `{}`.\n", arm.wire));
            out.push_str(&format!("    {}(Box<{}>),\n", arm.rust_name, arm.ty.render()));
        }
        out.push_str("    /// An object type not known to this version of the crate.\n");
        out.push_str("    Other(serde_json::Value),\n");
        out.push_str("}\n");
        items.push(std::mem::take(&mut out));

        out.push_str(&format!("impl<'de> serde::Deserialize<'de> for {} {{\n", ty.name));
        out.push_str("    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>\n    where\n        D: serde::Deserializer<'de>,\n    {\n");
        out.push_str("        let value: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;\n");
        out.push_str("        let tag = value.get(\"object\").and_then(serde_json::Value::as_str).unwrap_or(\"\");\n");
        out.push_str("        Ok(match tag {\n");
        for arm in arms {
            out.push_str(&format!(
                "            \"{}\" => {ty}::{}(Box::new(serde_json::from_value(value).map_err(serde::de::Error::custom)?)),\n",
                arm.wire,
                arm.rust_name,
                ty = ty.name
            ));
        }
        out.push_str(&format!(
            "            _ => {}::Other(value),\n        }})\n    }}\n}}\n",
            ty.name
        ));
        items.push(std::mem::take(&mut out));

        if ty.serialize {
            out.push_str(&format!(
                "impl serde::Serialize for {} {{\n    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>\n    where\n        S: serde::Serializer,\n    {{\n        match self {{\n",
                ty.name
            ));
            for arm in arms {
                out.push_str(&format!(
                    "            {}::{}(value) => serde::Serialize::serialize(&**value, serializer),\n",
                    ty.name, arm.rust_name
                ));
            }
            out.push_str(&format!(
                "            {}::Other(value) => serde::Serialize::serialize(value, serializer),\n        }}\n    }}\n}}\n",
                ty.name
            ));
            items.push(out);
        }
        items
    }

    fn emit_method(&self, method: &Method) -> String {
        let params_type = method.params_type.as_deref().unwrap_or("crate::params::Empty");
        let mut args = vec!["client: &crate::Client".to_string()];
        for (_, rust) in &method.path_args {
            args.push(format!("{rust}: &str"));
        }
        args.push(format!("params: &{params_type}"));
        let mut out = String::new();
        if !method.docs.is_empty() {
            out.push_str(&format!("#[doc = \"{}\"]\n", method.docs));
        }
        if method.multipart {
            out.push_str("///\n/// # Unsupported encoding\n///\n/// This endpoint takes a `multipart/form-data` body, which `stripe` cannot encode\n/// yet, so this method always returns [`Error::UnsupportedEncoding`](crate::Error::UnsupportedEncoding)\n/// without sending a request.\n");
        }
        out.push_str(&format!(
            "pub async fn {function}({args}) -> Result<{response}, crate::Error> {{\n",
            function = method.function,
            args = args.join(", "),
            response = method.response.render(),
        ));
        if method.multipart {
            // The client has no multipart encoder yet; fail loudly instead of
            // sending a body Stripe would reject.
            let mut unused: Vec<String> = vec!["client".to_string(), "params".to_string()];
            unused.extend(method.path_args.iter().map(|(_, rust)| rust.clone()));
            out.push_str(&format!("    let _ = ({});\n", unused.join(", ")));
            out.push_str(&format!(
                "    Err(crate::Error::UnsupportedEncoding {{\n        endpoint: \"{template}\",\n        encoding: \"multipart/form-data\",\n    }})\n}}\n",
                template = method.template,
            ));
            return out;
        }
        let pairs = method
            .path_args
            .iter()
            .map(|(wire, rust)| format!("(\"{wire}\", {rust})"))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "    let path = crate::params::format_path(\"{template}\", &[{pairs}]);\n",
            template = method.template,
        ));
        let call = match (method.verb, method.json) {
            (Verb::Get, _) => "client.get(&path, params)",
            (Verb::Post, false) => "client.post_form(&path, params)",
            (Verb::Post, true) => "client.post_json(&path, params)",
            (Verb::Delete, false) => "client.delete_form(&path, params)",
            (Verb::Delete, true) => "client.delete_json(&path, params)",
        };
        out.push_str(&format!("    {call}.await\n}}\n"));
        out
    }
}
