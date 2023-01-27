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

use std::collections::HashMap;

use amplify_syn::{ArgValueReq, AttrReq, ParametrizedAttr, TypeClass, ValueClass};
use proc_macro2::TokenStream as TokenStream2;
use syn::spanned::Spanned;
use syn::{Attribute, DeriveInput, Expr, LitStr, Path, Result};

static ATTR: &str = "strict_type";
static ATTR_CRATE: &str = "crate";
static ATTR_LIB: &str = "lib";
static ATTR_RENAME: &str = "rename";
static ATTR_WITH: &str = "with";
static ATTR_ENCODE_WITH: &str = "encode_with";
static ATTR_DECODE_WITH: &str = "decode_with";

struct ContainerAttr {
    pub strict_crate: Path,
    pub lib: Path,
    pub rename: Option<LitStr>,
    pub encode_with: Option<Path>,
    pub decode_with: Option<Path>,
}

struct FieldAttr {
    pub dumb: Option<Expr>,
}

struct VariantAttr {
    pub dumb: bool,
}

impl TryFrom<&[Attribute]> for ContainerAttr {
    type Error = syn::Error;

    fn try_from(attr: &[Attribute]) -> Result<Self> {
        let map = HashMap::from_iter(vec![
            (ATTR_CRATE, ArgValueReq::optional(TypeClass::Path)),
            (ATTR_LIB, ArgValueReq::required(TypeClass::Path)),
            (ATTR_RENAME, ArgValueReq::optional(ValueClass::str())),
            (ATTR_ENCODE_WITH, ArgValueReq::optional(TypeClass::Path)),
            (ATTR_DECODE_WITH, ArgValueReq::optional(TypeClass::Path)),
        ]);

        let mut params = ParametrizedAttr::with(ATTR, &attr)?;
        params.check(AttrReq::with(map))?;

        Ok(ContainerAttr {
            strict_crate: params.arg_value(ATTR_CRATE).unwrap_or_else(|_| path!(strict_encoding)),
            lib: params.unwrap_arg_value(ATTR_LIB),
            rename: params.arg_value(ATTR_RENAME).ok(),
            encode_with: params
                .arg_value(ATTR_ENCODE_WITH)
                .or_else(|_| params.arg_value(ATTR_WITH))
                .ok(),
            decode_with: params
                .arg_value(ATTR_DECODE_WITH)
                .or_else(|_| params.arg_value(ATTR_WITH))
                .ok(),
        })
    }
}

pub struct StrictDerive {
    input: DeriveInput,
    conf: ContainerAttr,
}

impl TryFrom<DeriveInput> for StrictDerive {
    type Error = syn::Error;

    fn try_from(input: DeriveInput) -> Result<Self> {
        let conf = ContainerAttr::try_from(input.attrs.as_ref())?;
        Ok(Self { input, conf })
    }
}

impl StrictDerive {
    pub fn strict_dumb(&self) -> Result<TokenStream2> {
        let (impl_generics, ty_generics, where_clause) = self.input.generics.split_for_impl();
        Ok(quote_spanned! { self.input.span() =>

        })
    }
}
