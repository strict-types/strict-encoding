use amplify_syn::ParametrizedAttr;
use proc_macro2::{Ident, TokenStream};
use syn::{Generics, Path, Type};

#[derive(Clone)]
pub struct DataType {
    pub generics: Generics,
    pub name: Ident,
    pub inner: DataInner,
}

#[derive(Clone)]
pub enum DataInner {
    Struct(Fields),
    Enum(Vec<Variant>),
    Union(Vec<NamedField>),
}

#[derive(Clone)]
pub enum Fields {
    Named(Vec<NamedField>),
    Unnamed(Vec<Field>),
}

#[derive(Clone)]
pub struct NamedField {
    pub name: Ident,
    pub field: Field,
}

#[derive(Clone)]
pub struct Field {
    pub vis: Vis,
    pub attr: ParametrizedAttr,
    pub ty: TokenStream,
}

#[derive(Clone)]
pub struct Variant {
    pub attr: ParametrizedAttr,
    pub name: Ident,
    pub fields: Fields,
}

#[derive(Clone)]
pub enum Vis {
    Public,
    Scoped(Scope),
    Inherit,
}

#[derive(Clone)]
pub enum Scope {
    Crate,
    Super,
    Path(Path),
}
