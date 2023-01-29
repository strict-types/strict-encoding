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

use amplify_syn::{DeriveInner, Field, Items, NamedField, Variant};
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{Error, Result};

use crate::params::{FieldAttr, StrictDerive, VariantAttr};

struct DeriveDumb<'a>(&'a StrictDerive);

impl StrictDerive {
    pub fn derive_dumb(&self) -> Result<TokenStream2> {
        self.data.derive(&self.conf.strict_crate, &ident!(StrictDumb), &DeriveDumb(self))
    }
}

impl DeriveInner for DeriveDumb<'_> {
    fn derive_unit_inner(&self) -> Result<TokenStream2> {
        Ok(quote! {
            fn strict_dumb() -> Self {
                Self()
            }
        })
    }

    fn derive_struct_inner(&self, fields: &Items<NamedField>) -> Result<TokenStream2> {
        if let Some(ref dumb_expr) = self.0.conf.dumb {
            return Ok(quote! {
                fn strict_dumb() -> Self {
                    #dumb_expr
                }
            });
        }

        let crate_name = &self.0.conf.strict_crate;
        let trait_name = quote!(::#crate_name::StrictDumb);
        let items = fields.iter().map(|named| {
            let attr = FieldAttr::try_from(named.field.attr.clone()).expect("invalid attribute");
            let name = &named.name;
            match attr.dumb {
                None => quote! { #name: StrictDumb::strict_dumb() },
                Some(dumb_value) => quote! { #name: #dumb_value },
            }
        });

        Ok(quote! {
            fn strict_dumb() -> Self {
                use #trait_name;
                Self {
                    #( #items ),*
                }
            }
        })
    }

    fn derive_tuple_inner(&self, fields: &Items<Field>) -> Result<TokenStream2> {
        if let Some(ref dumb_expr) = self.0.conf.dumb {
            return Ok(quote! {
                fn strict_dumb() -> Self {
                    #dumb_expr
                }
            });
        }

        let crate_name = &self.0.conf.strict_crate;
        let trait_name = quote!(::#crate_name::StrictDumb);
        let items = fields.iter().map(|field| {
            let attr = FieldAttr::try_from(field.attr.clone()).expect("invalid attribute");
            match attr.dumb {
                None => quote! { StrictDumb::strict_dumb() },
                Some(dumb_value) => quote! { #dumb_value },
            }
        });

        Ok(quote! {
            fn strict_dumb() -> Self {
                use #trait_name;
                Self(#( #items ),*)
            }
        })
    }

    fn derive_enum_inner(&self, variants: &Items<Variant>) -> Result<TokenStream2> {
        if let Some(ref dumb_expr) = self.0.conf.dumb {
            return Ok(quote! {
                fn strict_dumb() -> Self {
                    #dumb_expr
                }
            });
        }

        let dumb_variant = variants
            .iter()
            .find_map(|variant| {
                let attr = VariantAttr::try_from(variant.attr.clone()).expect("invalid attribute");
                let name = &variant.name;
                match attr.dumb {
                    false => Some(quote! { Self::#name }),
                    true => None,
                }
            })
            .ok_or_else(|| {
                Error::new(
                    Span::call_site(),
                    "enum must mark one of its variants with `#[strict_type(dumb)]` attribute, or \
                     provide a dumb value in eponym attribute at container level",
                )
            })?;

        Ok(quote! {
            fn strict_dumb() -> Self {
                #dumb_variant
            }
        })
    }
}
