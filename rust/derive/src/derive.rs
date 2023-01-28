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
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Expr, LitInt, LitStr, Path, Result,
};

const ATTR: &str = "strict_type";
const ATTR_CRATE: &str = "crate";
const ATTR_LIB: &str = "lib";
const ATTR_RENAME: &str = "rename";
const ATTR_WITH: &str = "with";
const ATTR_ENCODE_WITH: &str = "encode_with";
const ATTR_DECODE_WITH: &str = "decode_with";
const ATTR_DUMB: &str = "dumb";
const ATTR_TAGS: &str = "tags";
const ATTR_TAGS_ORDER: &str = "order";
const ATTR_TAGS_REPR: &str = "repr";
const ATTR_TAGS_CUSTOM: &str = "custom";

struct ContainerAttr {
    pub strict_crate: Path,
    pub lib: Path,
    pub rename: Option<LitStr>,
    pub encode_with: Option<Path>,
    pub decode_with: Option<Path>,
}

struct EnumAttr {
    pub tags: VariantTags,
}

struct FieldAttr {
    pub dumb: Option<Expr>,
    pub rename: Option<LitStr>,
}

struct VariantAttr {
    pub dumb: bool,
    pub rename: Option<LitStr>,
    pub value: Option<LitInt>,
}

enum VariantTags {
    Repr,
    Order,
    Custom,
}

impl TryFrom<&[Attribute]> for ContainerAttr {
    type Error = Error;

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

impl TryFrom<&[Attribute]> for EnumAttr {
    type Error = Error;

    fn try_from(attr: &[Attribute]) -> Result<Self> {
        let map = HashMap::from_iter(vec![(ATTR_TAGS, ArgValueReq::required(TypeClass::Path))]);

        let mut params = ParametrizedAttr::with(ATTR, &attr)?;
        params.check(AttrReq::with(map))?;

        let tags = match params
            .arg_value(ATTR_TAGS)
            .unwrap_or_else(|_| path!(custom))
            .to_token_stream()
            .to_string()
            .as_str()
        {
            ATTR_TAGS_REPR => VariantTags::Repr,
            ATTR_TAGS_ORDER => VariantTags::Order,
            ATTR_TAGS_CUSTOM => VariantTags::Custom,
            unknown => {
                return Err(Error::new(
                    Span::call_site(),
                    format!(
                        "invalid enum strict encoding value for `tags` attribute `{unknown}`; \
                         only `repr`, `order` or `custom` are allowed"
                    ),
                ))
            }
        };

        Ok(EnumAttr { tags })
    }
}

pub struct StrictDerive {
    input: DeriveInput,
    conf: ContainerAttr,
}

impl TryFrom<DeriveInput> for StrictDerive {
    type Error = Error;

    fn try_from(input: DeriveInput) -> Result<Self> {
        let conf = ContainerAttr::try_from(input.attrs.as_ref())?;
        Ok(Self { input, conf })
    }
}

impl StrictDerive {
    pub fn strict_dumb(&self) -> Result<TokenStream2> {
        match &self.input.data {
            Data::Struct(data) => self.strict_dumb_struct(data),
            Data::Enum(data) => self.strict_dumb_enum(data),
            Data::Union(_) => Err(Error::new_spanned(
                &self.input,
                "deriving strict type traits is not supported in unions".to_owned(),
            )),
        }
    }
}

impl StrictDerive {
    fn strict_dumb_struct(&self, data: &DataStruct) -> Result<TokenStream2> {
        let (impl_generics, ty_generics, where_clause) = self.input.generics.split_for_impl();

        let import = &self.conf.strict_crate;
        let ident_name = &self.input.ident;

        Ok(quote_spanned! { self.input.span() =>
            impl #impl_generics #import::StrictDumb for #ident_name #ty_generics #where_clause {

            }
        })
    }

    fn strict_dumb_enum(&self, data: &DataEnum) -> Result<TokenStream2> {
        let (impl_generics, ty_generics, where_clause) = self.input.generics.split_for_impl();

        let import = &self.conf.strict_crate;
        let ident_name = &self.input.ident;

        Ok(quote_spanned! { self.input.span() =>
            impl #impl_generics #import::StrictDumb for #ident_name #ty_generics #where_clause {

            }
        })
    }
}
