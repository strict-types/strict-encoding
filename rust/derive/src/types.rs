use std::ops::{Deref, DerefMut};
use std::slice;

use amplify_syn::ParametrizedAttr;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::{DeriveInput, Generics, Path};

#[derive(Clone)]
pub struct DataType {
    pub generics: Generics,
    pub name: Ident,
    pub attr: ParametrizedAttr,
    pub inner: DataInner,
}

#[derive(Clone)]
pub enum DataInner {
    Uninhabited,
    Struct(Fields),
    Enum(Items<Variant>),
    Union(Items<NamedField>),
}

#[derive(Clone)]
pub enum Fields {
    Unit,
    Named(Items<NamedField>),
    Unnamed(Items<Field>),
}

pub trait Element: Sized {
    type Input: Sized;
    fn with(input: Self::Input, attr_name: &Ident) -> syn::Result<Self>;
}

#[derive(Clone)]
pub struct Items<E: Element>(Vec<E>);

impl<E: Element> Deref for Items<E> {
    type Target = Vec<E>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<E: Element> DerefMut for Items<E> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<'a, E: Element> IntoIterator for &'a Items<E> {
    type Item = &'a E;
    type IntoIter = slice::Iter<'a, E>;

    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
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
    pub ty: syn::Type,
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
    Inherited,
}

#[derive(Clone)]
pub enum Scope {
    Crate,
    Super,
    Path(Path),
}

impl DataType {
    pub fn with(input: DeriveInput, attr_name: Ident) -> syn::Result<Self> {
        let attr = ParametrizedAttr::with(attr_name.to_string(), input.attrs.as_ref())?;
        Ok(DataType {
            generics: input.generics,
            name: input.ident,
            attr,
            inner: DataInner::with(input.data, &attr_name)?,
        })
    }
}

impl DataInner {
    pub fn with(data: syn::Data, attr_name: &Ident) -> syn::Result<Self> {
        match data {
            syn::Data::Struct(inner) => {
                Fields::with(inner.fields, attr_name).map(DataInner::Struct)
            }
            syn::Data::Enum(inner) if inner.variants.is_empty() => Ok(DataInner::Uninhabited),
            syn::Data::Enum(inner) => {
                Items::with(inner.variants.into_iter(), attr_name).map(DataInner::Enum)
            }
            syn::Data::Union(inner) => {
                Items::with(inner.fields.named.into_iter(), attr_name).map(DataInner::Union)
            }
        }
    }
}

impl Fields {
    pub fn with(fields: syn::Fields, attr_name: &Ident) -> syn::Result<Self> {
        match fields {
            syn::Fields::Named(fields) => {
                Items::with(fields.named.into_iter(), attr_name).map(Fields::Named)
            }
            syn::Fields::Unnamed(fields) => {
                Items::with(fields.unnamed.into_iter(), attr_name).map(Fields::Unnamed)
            }
            syn::Fields::Unit => Ok(Fields::Unit),
        }
    }
}

impl<E: Element> Items<E> {
    pub fn with(
        items: impl ExactSizeIterator<Item = E::Input>,
        attr_name: &Ident,
    ) -> syn::Result<Self> {
        let mut list = Vec::with_capacity(items.len());
        for el in items {
            list.push(E::with(el, attr_name)?)
        }
        Ok(Items(list))
    }
}

impl Element for NamedField {
    type Input = syn::Field;

    fn with(input: Self::Input, attr_name: &Ident) -> syn::Result<Self> {
        Ok(NamedField {
            name: input.ident.clone().expect("named field without a name"),
            field: Field::with(input, attr_name)?,
        })
    }
}
impl Element for Field {
    type Input = syn::Field;

    fn with(input: Self::Input, attr_name: &Ident) -> syn::Result<Self> {
        let attr = ParametrizedAttr::with(attr_name.to_string(), input.attrs.as_ref())?;
        Ok(Field {
            vis: input.vis.into(),
            attr,
            ty: input.ty,
        })
    }
}

impl Element for Variant {
    type Input = syn::Variant;

    fn with(input: Self::Input, attr_name: &Ident) -> syn::Result<Self> {
        let attr = ParametrizedAttr::with(attr_name.to_string(), input.attrs.as_ref())?;
        Ok(Variant {
            attr,
            name: input.ident,
            fields: Fields::with(input.fields, attr_name)?,
        })
    }
}

impl From<syn::Visibility> for Vis {
    fn from(vis: syn::Visibility) -> Self {
        match vis {
            syn::Visibility::Public(_) => Vis::Public,
            syn::Visibility::Crate(_) => Vis::Scoped(Scope::Crate),
            syn::Visibility::Restricted(scope) => Vis::Scoped(scope.into()),
            syn::Visibility::Inherited => Vis::Inherited,
        }
    }
}

impl From<syn::VisRestricted> for Scope {
    fn from(scope: syn::VisRestricted) -> Self {
        if scope.in_token.is_none() {
            debug_assert_eq!(scope.path.get_ident().unwrap(), &ident!(super));
            Scope::Super
        } else {
            Scope::Path(*scope.path)
        }
    }
}

impl DataType {
    pub fn derive<D: Derive>(
        &self,
        trait_crate: Path,
        trait_name: Ident,
        attr: &D,
    ) -> syn::Result<TokenStream2> {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let ident_name = &self.name;

        let inner = match &self.inner {
            DataInner::Struct(Fields::Unit) => attr.derive_unit(),
            DataInner::Struct(Fields::Unnamed(fields)) => attr.derive_fields(fields),
            DataInner::Struct(Fields::Named(fields)) => attr.derive_named_fields(fields),
            DataInner::Enum(variants) => attr.derive_variants(variants),
            DataInner::Union(_) => Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "deriving `{}` is not supported in unions",
                    trait_name.to_token_stream().to_string()
                ),
            )),
            DataInner::Uninhabited => Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "deriving `{}` is not supported for uninhabited enums",
                    trait_name.to_token_stream().to_string()
                ),
            )),
        }?;

        let tokens = quote! {
            impl #impl_generics ::#trait_crate::#trait_name for #ident_name #ty_generics #where_clause {
                #inner
            }
        };

        eprintln!("{}", tokens.to_string());

        Ok(tokens)
    }
}

pub trait Derive {
    fn derive_unit(&self) -> syn::Result<TokenStream2>;
    fn derive_named_fields(&self, fields: &Items<NamedField>) -> syn::Result<TokenStream2>;
    fn derive_fields(&self, fields: &Items<Field>) -> syn::Result<TokenStream2>;
    fn derive_variants(&self, fields: &Items<Variant>) -> syn::Result<TokenStream2>;
}
