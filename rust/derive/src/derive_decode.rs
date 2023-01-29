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

use amplify_syn::{DeriveInner, EnumKind, Field, FieldKind, Fields, Items, NamedField, Variant};
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use syn::{Error, LitStr, Result};

use crate::params::{FieldAttr, StrictDerive, VariantAttr};

struct DeriveDecode<'a>(&'a StrictDerive);

impl StrictDerive {
    pub fn derive_decode(&self) -> Result<TokenStream2> {
        let res = self.data.derive(
            &self.conf.strict_crate,
            &ident!(StrictDecode),
            &DeriveDecode(self),
        )?;
        Ok(res)
    }
}

impl DeriveInner for DeriveDecode<'_> {
    fn derive_unit_inner(&self) -> Result<TokenStream2> {
        Err(Error::new(
            Span::call_site(),
            "StrictDecode must not be derived on a unit types. Use just a unit type instead when \
             encoding parent structure.",
        ))
    }

    fn derive_struct_inner(&self, fields: &Items<NamedField>) -> Result<TokenStream2> {
        let crate_name = &self.0.conf.strict_crate;

        let name = fields.iter().map(|f| &f.name);
        let name2 = fields.iter().map(|f| &f.name);

        Ok(quote! {
            fn strict_decode(reader: &mut impl ::#crate_name::TypedRead) -> Result<Self, ::#crate_name::DecodeError> {
                use ::#crate_name::{TypedRead, ReadStruct, fname};
                reader.read_struct(|r| {
                    #( let #name = r.read_field(fname!(stringify!(#name)))?; )*
                    Ok(Self { #( #name2 ),* })
                })
            }
        })
    }

    fn derive_tuple_inner(&self, fields: &Items<Field>) -> Result<TokenStream2> {
        let crate_name = &self.0.conf.strict_crate;

        let no = fields
            .iter()
            .enumerate()
            .map(|(index, _)| Ident::new(&format!("_{index}"), Span::call_site()))
            .collect::<Vec<_>>();
        let no2 = no.clone();

        Ok(quote! {
            fn strict_decode(reader: &mut impl ::#crate_name::TypedRead) -> Result<Self, ::#crate_name::DecodeError> {
                use ::#crate_name::{TypedRead, ReadTuple};
                reader.read_tuple(|r| {
                    #( let #no = r.read_field()?; )*
                    Ok(Self( #( #no2 ),* ))
                })
            }
        })
    }

    fn derive_enum_inner(&self, variants: &Items<Variant>) -> Result<TokenStream2> {
        let crate_name = &self.0.conf.strict_crate;

        let inner = if variants.enum_kind() == EnumKind::Primitive {
            quote! {
                reader.read_enum()
            }
        } else {
            let mut read_variants = Vec::with_capacity(variants.len());
            for var in variants {
                let attr = VariantAttr::try_from(var.attr.clone())?;
                let var_name = &var.name;
                let name = match attr.rename.as_ref() {
                    None => {
                        let s = var_name.to_string();
                        let mut c = s.chars();
                        let s = match c.next() {
                            None => String::new(),
                            Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
                        };
                        Ident::new(&s, Span::call_site())
                    }
                    Some(name) => name.clone(),
                };
                let name = LitStr::new(&name.to_string(), Span::call_site());
                match &var.fields {
                    Fields::Unit => {
                        read_variants.push(quote! {
                            #name => Ok(Self::#var_name),
                        });
                    }
                    Fields::Unnamed(fields) => {
                        let mut field_idx = Vec::with_capacity(fields.len());
                        for index in 0..fields.len() {
                            let index = Ident::new(&format!("_{index}"), Span::call_site());
                            field_idx.push(quote! { #index });
                        }
                        read_variants.push(quote! {
                            #name => r.read_tuple(|t| {
                                #( let #field_idx = t.read_field()?; )*
                                Ok(Self::#var_name( #(#field_idx),* ))
                            }),
                        });
                    }
                    Fields::Named(fields) => {
                        let mut field_name = Vec::with_capacity(fields.len());
                        let mut field_rename = Vec::with_capacity(fields.len());
                        for named_field in fields {
                            let attr =
                                FieldAttr::with(named_field.field.attr.clone(), FieldKind::Named)?;

                            let name = &named_field.name;
                            let rename = match attr.rename {
                                None => named_field.name.clone(),
                                Some(name) => name,
                            };
                            let rename = LitStr::new(&rename.to_string(), Span::call_site());

                            field_name.push(quote! { #name });
                            field_rename.push(quote! { #rename });
                        }

                        read_variants.push(quote! {
                            #name => r.read_struct(|s| {
                                #( let #field_name = s.read_field(fname!(#field_rename))?; )*
                                Ok(Self::#var_name { #(#field_name),* })
                            }),
                        });
                    }
                }
            }

            quote! {
                #[allow(unused_imports)]
                use ::#crate_name::{ReadUnion, ReadTuple, ReadStruct, fname};
                reader.read_union(|field_name, r| {
                    match field_name.as_str() {
                        #( #read_variants )*
                        _ => unreachable!(),
                    }
                })
            }
        };

        Ok(quote! {
            fn strict_decode(reader: &mut impl ::#crate_name::TypedRead) -> Result<Self, ::#crate_name::DecodeError> {
                use ::#crate_name::TypedRead;
                #inner
            }
        })
    }
}