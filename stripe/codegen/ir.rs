//! Intermediate representation for generated types.

/// A Rust type reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeRef {
    /// `()`
    Unit,
    /// `bool`
    Bool,
    /// `i64`
    I64,
    /// `u64`
    #[allow(dead_code)]
    U64,
    /// `f64`
    F64,
    /// `String`
    Str,
    /// `serde_json::Value`
    Json,
    /// A generated type, stored as a fully-qualified path.
    Named(String),
    /// `Vec<T>`
    Vec(Box<TypeRef>),
    /// `crate::List<T>`
    List(Box<TypeRef>),
    /// `std::collections::HashMap<String, T>`
    Map(Box<TypeRef>),
    /// `Option<T>`
    Opt(Box<TypeRef>),
    /// `crate::Expandable<T>`
    Expandable(Box<TypeRef>),
}

impl TypeRef {
    /// Renders this type as Rust source.
    pub fn render(&self) -> String {
        match self {
            TypeRef::Unit => "()".to_string(),
            TypeRef::Bool => "bool".to_string(),
            TypeRef::I64 => "i64".to_string(),
            TypeRef::U64 => "u64".to_string(),
            TypeRef::F64 => "f64".to_string(),
            TypeRef::Str => "String".to_string(),
            TypeRef::Json => "serde_json::Value".to_string(),
            TypeRef::Named(path) => path.clone(),
            TypeRef::Vec(inner) => format!("Vec<{}>", inner.render()),
            TypeRef::List(inner) => format!("crate::List<{}>", inner.render()),
            TypeRef::Map(inner) => format!("std::collections::HashMap<String, {}>", inner.render()),
            TypeRef::Opt(inner) => format!("Option<{}>", inner.render()),
            TypeRef::Expandable(inner) => format!("crate::Expandable<{}>", inner.render()),
        }
    }

    /// Renders this type inside a `Box`, used to break recursive type cycles.
    pub fn render_boxed(&self) -> String {
        match self {
            TypeRef::Opt(inner) => format!("Option<Box<{}>>", inner.render()),
            other => format!("Box<{}>", other.render()),
        }
    }
}

/// A struct field.
#[derive(Debug, Clone)]
pub struct Field {
    /// Rust field name.
    pub rust_name: String,
    /// Original JSON name, when it differs from the Rust name.
    pub json_name: Option<String>,
    /// Field type.
    pub ty: TypeRef,
    /// Documentation.
    pub docs: String,
    /// Whether the field must be boxed to break a recursive type cycle.
    pub boxed: bool,
}

impl Field {
    /// Renders the field's Rust type.
    pub fn rendered_type(&self) -> String {
        if self.boxed {
            self.ty.render_boxed()
        } else {
            self.ty.render()
        }
    }
}

/// A string enum variant.
#[derive(Debug, Clone)]
pub struct Variant {
    /// Rust variant name.
    pub rust_name: String,
    /// Wire value.
    pub wire: String,
    /// Documentation.
    pub docs: String,
}

/// An arm of a discriminated union.
#[derive(Debug, Clone)]
pub struct UnionArm {
    /// Rust variant name.
    pub rust_name: String,
    /// Value of the `object` discriminator.
    pub wire: String,
    /// The object type.
    pub ty: TypeRef,
}

/// The shape of a generated type.
#[derive(Debug, Clone)]
pub enum TypeKind {
    /// A struct.
    Struct(Vec<Field>),
    /// A string enum.
    Enum(Vec<Variant>),
    /// A type alias.
    Alias(TypeRef),
    /// A map type alias (`HashMap<String, T>`).
    Map(TypeRef),
    /// A discriminated union of object types with a raw fallback.
    Union(Vec<UnionArm>),
}

/// How a generated type is encoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// A response type: `Deserialize` + `Serialize`.
    Response,
    /// A v1 request type: `Serialize` + `FormEncode`.
    Form,
    /// A v2 request type: `Serialize` only.
    Json,
}

/// A generated type definition.
#[derive(Debug, Clone)]
pub struct TypeDef {
    /// Simple type name.
    pub name: String,
    /// Fully-qualified Rust path.
    pub fqn: String,
    /// Owning module (used for feature gating and layout).
    pub module: String,
    /// Documentation.
    pub docs: String,
    /// The type's shape.
    pub kind: TypeKind,
    /// Encoding mode (drives which impls are generated).
    pub mode: Mode,
    /// Response types derive `Serialize` only when a JSON (v2) request can
    /// reference them; v1 form requests never serialize a response type.
    pub serialize: bool,
    /// Enums implement `Default` only when a response field without an `Option`
    /// falls back to one.
    pub needs_default: bool,
    /// Enums implement `FormEncode` only when a v1 request body can reference
    /// them.
    pub needs_form_encode: bool,
}
