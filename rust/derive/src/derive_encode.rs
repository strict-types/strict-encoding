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

use amplify_syn::{DeriveInner, Field, Fields, Items, NamedField, Variant};
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use syn::{Error, Index, LitStr, Result};

use crate::params::{FieldAttr, StrictDerive, VariantAttr};

struct DeriveEncode<'a>(&'a StrictDerive);

impl StrictDerive {
    pub fn derive_encode(&self) -> Result<TokenStream2> {
        self.data.derive(&self.conf.strict_crate, &ident!(StrictEncode), &DeriveEncode(self))
    }
    pub fn derive_decode(&self) -> Result<TokenStream2> { Ok(quote! {}) }
}

impl DeriveInner for DeriveEncode<'_> {
    fn derive_unit_inner(&self) -> Result<TokenStream2> {
        Err(Error::new(
            Span::call_site(),
            "StrictEncode must not be derived on a unit types. Use just a unit type instead when \
             encoding parent structure.",
        ))
    }

    fn derive_struct_inner(&self, fields: &Items<NamedField>) -> Result<TokenStream2> {
        let crate_name = &self.0.conf.strict_crate;

        let name = fields.iter().map(|named_field| {
            let attr =
                FieldAttr::try_from(named_field.field.attr.clone()).expect("invalid attribute");
            match attr.rename {
                None => named_field.name.clone(),
                Some(name) => name,
            }
        });

        Ok(quote! {
            fn strict_encode<W: ::#crate_name::TypedWrite>(&self, writer: W) -> ::std::io::Result<W> {
                use ::#crate_name::{TypedWrite, WriteStruct};
                writer.write_struct::<Self>(|w| {
                    Ok(w
                        #( .write_field(::#crate_name::fname!(stringify!(#name)), &self.#name)? )*
                        .complete())
                })
            }
        })
    }

    fn derive_tuple_inner(&self, fields: &Items<Field>) -> Result<TokenStream2> {
        let crate_name = &self.0.conf.strict_crate;

        let no = fields.iter().enumerate().map(|(index, _)| Index::from(index));

        Ok(quote! {
            fn strict_encode<W: ::#crate_name::TypedWrite>(&self, writer: W) -> ::std::io::Result<W> {
                use ::#crate_name::{TypedWrite, WriteTuple};
                writer.write_tuple::<Self>(|w| {
                    Ok(w
                        #( .write_field(&self.#no)? )*
                        .complete())
                })
            }
        })
    }

    fn derive_enum_inner(&self, variants: &Items<Variant>) -> Result<TokenStream2> {
        let crate_name = &self.0.conf.strict_crate;

        let inner = if variants.iter().all(|var| var.fields.is_unit()) {
            quote! {
                writer.write_enum(*self)
            }
        } else {
            let mut define_fields = Vec::with_capacity(variants.len());
            let mut write_fields = Vec::with_capacity(variants.len());
            for var in variants {
                let attr = VariantAttr::try_from(var.attr.clone()).expect("invalid attribute");
                let var_name = &var.name;
                let name = match attr.rename.as_ref() {
                    None => var_name, // TODO: Lowercase the first letter
                    Some(name) => name,
                };
                let name = LitStr::new(&name.to_string(), Span::call_site());
                match &var.fields {
                    Fields::Unit => {
                        define_fields.push(quote! {
                            .define_unit(fname!(#name))
                        });
                        write_fields.push(quote! {
                            Self::#var_name => writer.write_unit(fname!(#name))?,
                        });
                    }
                    Fields::Unnamed(fields) => {
                        let mut field_ty = Vec::with_capacity(fields.len());
                        let mut field_idx = Vec::with_capacity(fields.len());
                        for (index, field) in fields.iter().enumerate() {
                            let ty = &field.ty;
                            let index = Ident::new(&format!("_{index}"), Span::call_site());
                            field_ty.push(quote! { #ty });
                            field_idx.push(quote! { #index });
                        }
                        define_fields.push(quote! {
                            .define_tuple(fname!(#name), |d| {
                                d #( .define_field::<#field_ty>() )* .complete()
                            })
                        });
                        write_fields.push(quote! {
                            Self::#var_name( #( #field_idx ),* ) => writer.write_tuple(fname!(#name), |w| {
                                Ok(w #( .write_field(#field_idx)? )* .complete())
                            })?
                        });
                    }
                    Fields::Named(fields) => {
                        let mut field_ty = Vec::with_capacity(fields.len());
                        let mut field_name = Vec::with_capacity(fields.len());
                        let mut field_rename = Vec::with_capacity(fields.len());
                        for named_field in fields {
                            let attr = FieldAttr::try_from(named_field.field.attr.clone())?;

                            let ty = &named_field.field.ty;
                            let name = &named_field.name;
                            let rename = match attr.rename {
                                None => named_field.name.clone(),
                                Some(name) => name,
                            };
                            field_ty.push(quote! { #ty });
                            field_name.push(quote! { #name });
                            field_rename.push(quote! { #rename });
                        }

                        define_fields.push(quote! {
                            .define_struct(fname!(#name), |d| {
                                d #( .define_field::<#field_ty>() )* .complete()
                            })
                        });
                        write_fields.push(quote! {
                            Self::#var_name { #( #field_name ),* } => writer.write_struct(fname!(#name), |w| {
                                Ok(w #( .write_field(fname!(stringify!(#field_rename)), #field_name)? )* .complete())
                            })?
                        });
                    }
                }
            }

            quote! {
                use ::#crate_name::{DefineUnion, WriteUnion, DefineTuple, DefineStruct};
                writer.write_union::<Self>(|definer| {
                    let writer = definer
                        #( #define_fields )*
                        .complete();

                    Ok(match self {
                        #( #write_fields )*
                    }.complete());
                })
            }
        };

        Ok(quote! {
            fn strict_encode<W: ::#crate_name::TypedWrite>(&self, writer: W) -> ::std::io::Result<W> {
                use ::#crate_name::TypedWrite;
                #inner
            }
        })
    }
}
