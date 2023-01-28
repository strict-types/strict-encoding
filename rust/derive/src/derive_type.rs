// Derivation macro library for strict encoding.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2023 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright 2022-2023 UBIDECO Institute
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use amplify_syn::{DataInner, DeriveInner, Field, Fields, Items, NamedField, Variant};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::{LitStr, Result};

use crate::params::{ContainerAttr, EnumAttr, FieldAttr, StrictDerive, VariantAttr, VariantTags};

struct DeriveType<'a>(&'a StrictDerive);
struct DeriveProduct<'a>(&'a Fields);
struct DeriveTuple<'a>(&'a Items<Field>);
struct DeriveStruct<'a>(&'a Items<NamedField>);
struct DeriveSum<'a>(&'a Items<Variant>, &'a ContainerAttr, EnumAttr);
struct DeriveEnum<'a>(&'a Items<Variant>);
struct DeriveUnion<'a>(&'a Items<Variant>);

impl StrictDerive {
    pub fn derive_type(&self) -> Result<TokenStream2> {
        let trait_crate = &self.conf.strict_crate;

        let impl_type = self.data.derive(trait_crate, &ident!(StrictType), &DeriveType(self))?;

        let impl_outer = match &self.data.inner {
            DataInner::Struct(fields) => {
                self.data.derive(trait_crate, &ident!(StrictProduct), &DeriveProduct(fields))?
            }
            DataInner::Enum(variants) => {
                let enum_attr = EnumAttr::try_from(self.data.attr.clone())?;
                self.data.derive(
                    trait_crate,
                    &ident!(StrictSum),
                    &DeriveSum(variants, &self.conf, enum_attr),
                )?
            }
            _ => TokenStream2::new(),
        };

        let impl_inner = match &self.data.inner {
            DataInner::Struct(Fields::Named(fields)) => {
                self.data.derive(trait_crate, &ident!(StrictStruct), &DeriveStruct(fields))?
            }
            DataInner::Struct(Fields::Unnamed(fields)) => {
                self.data.derive(trait_crate, &ident!(StrictTuple), &DeriveTuple(fields))?
            }
            DataInner::Enum(variants) if variants.iter().all(|var| var.fields.is_unit()) => {
                self.data.derive(trait_crate, &ident!(StrictEnum), &DeriveEnum(variants))?
            }
            DataInner::Enum(variants) => {
                self.data.derive(trait_crate, &ident!(StrictUnion), &DeriveUnion(variants))?
            }
            _ => TokenStream2::new(),
        };

        Ok(quote! {
            #impl_type
            #impl_outer
            #impl_inner
        })
    }
}

impl DeriveType<'_> {
    pub fn derive_type(&self) -> Result<TokenStream2> {
        let lib_name = &self.0.conf.lib;
        Ok(quote! {
            const STRICT_LIB_NAME: &'static str = #lib_name;
        })
    }
}

impl DeriveInner for DeriveType<'_> {
    fn derive_unit_inner(&self) -> Result<TokenStream2> { self.derive_type() }
    fn derive_struct_inner(&self, _fields: &Items<NamedField>) -> Result<TokenStream2> {
        self.derive_type()
    }
    fn derive_tuple_inner(&self, _fields: &Items<Field>) -> Result<TokenStream2> {
        self.derive_type()
    }
    fn derive_enum_inner(&self, _variants: &Items<Variant>) -> Result<TokenStream2> {
        self.derive_type()
    }
}

impl DeriveInner for DeriveProduct<'_> {
    fn derive_unit_inner(&self) -> Result<TokenStream2> { unreachable!() }
    fn derive_enum_inner(&self, _variants: &Items<Variant>) -> Result<TokenStream2> {
        unreachable!()
    }

    fn derive_struct_inner(&self, _fields: &Items<NamedField>) -> Result<TokenStream2> {
        Ok(TokenStream2::new())
    }
    fn derive_tuple_inner(&self, _fields: &Items<Field>) -> Result<TokenStream2> {
        Ok(TokenStream2::new())
    }
}

impl DeriveInner for DeriveSum<'_> {
    fn derive_unit_inner(&self) -> Result<TokenStream2> { unreachable!() }
    fn derive_struct_inner(&self, _fields: &Items<NamedField>) -> Result<TokenStream2> {
        unreachable!()
    }
    fn derive_tuple_inner(&self, _fields: &Items<Field>) -> Result<TokenStream2> { unreachable!() }

    fn derive_enum_inner(&self, variants: &Items<Variant>) -> Result<TokenStream2> {
        let items = variants.iter().enumerate().map(|(index, variant)| {
            let attr = VariantAttr::try_from(variant.attr.clone()).expect("invalid attribute");
            let name = match attr.rename {
                None => LitStr::new(&variant.name.to_string(), Span::call_site()),
                Some(name) => name,
            };
            let ord = match (&self.2.tags, &attr.tag) {
                (_, Some(ord)) => ord.to_token_stream(),
                (VariantTags::Repr, None) => quote! { self as u8 },
                (VariantTags::Order, None) => quote! { #index },
                (VariantTags::Custom, None) => {
                    panic!("tag is required for variant `{}`", variant.name)
                }
            };
            quote! { (#ord, #name) }
        });

        let trait_crate = &self.1.strict_crate;

        Ok(quote! {
            const ALL_VARIANTS: &'static [(u8, &'static str)] = &[
                #( #items ),*
            ];

            fn variant_name(&self) -> &'static str {
                use #trait_crate::StrictSum;
                Self::ALL_VARIANTS[self.variant_ord()].1
            }
        })
    }
}

impl DeriveInner for DeriveTuple<'_> {
    fn derive_unit_inner(&self) -> Result<TokenStream2> { unreachable!() }
    fn derive_struct_inner(&self, _fields: &Items<NamedField>) -> Result<TokenStream2> {
        unreachable!()
    }
    fn derive_enum_inner(&self, _variants: &Items<Variant>) -> Result<TokenStream2> {
        unreachable!()
    }

    fn derive_tuple_inner(&self, fields: &Items<Field>) -> Result<TokenStream2> {
        let field_count = fields.len();
        Ok(quote! {
            const FIELD_COUNT: u8 = #field_count as u8;
        })
    }
}

impl DeriveInner for DeriveStruct<'_> {
    fn derive_unit_inner(&self) -> Result<TokenStream2> { unreachable!() }
    fn derive_enum_inner(&self, _variants: &Items<Variant>) -> Result<TokenStream2> {
        unreachable!()
    }
    fn derive_tuple_inner(&self, _fields: &Items<Field>) -> Result<TokenStream2> { unreachable!() }

    fn derive_struct_inner(&self, fields: &Items<NamedField>) -> Result<TokenStream2> {
        let items = fields.iter().map(|named_field| {
            let attr =
                FieldAttr::try_from(named_field.field.attr.clone()).expect("invalid attribute");
            match attr.rename {
                None => LitStr::new(&named_field.name.to_string(), Span::call_site()),
                Some(name) => name,
            }
        });

        Ok(quote! {
            const ALL_FIELDS: &'static [&'static str] = &[
                #( #items ),*
            ];
        })
    }
}

impl DeriveInner for DeriveEnum<'_> {
    fn derive_unit_inner(&self) -> Result<TokenStream2> { unreachable!() }
    fn derive_struct_inner(&self, _fields: &Items<NamedField>) -> Result<TokenStream2> {
        unreachable!()
    }
    fn derive_tuple_inner(&self, _fields: &Items<Field>) -> Result<TokenStream2> { unreachable!() }

    fn derive_enum_inner(&self, _variants: &Items<Variant>) -> Result<TokenStream2> {
        Ok(TokenStream2::new())
    }
}

impl DeriveInner for DeriveUnion<'_> {
    fn derive_unit_inner(&self) -> Result<TokenStream2> { unreachable!() }
    fn derive_struct_inner(&self, _fields: &Items<NamedField>) -> Result<TokenStream2> {
        unreachable!()
    }
    fn derive_tuple_inner(&self, _fields: &Items<Field>) -> Result<TokenStream2> { unreachable!() }

    fn derive_enum_inner(&self, _variants: &Items<Variant>) -> Result<TokenStream2> {
        Ok(TokenStream2::new())
    }
}
