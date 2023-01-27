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

use amplify_syn::{ArgValue, ArgValueReq, AttrReq, ParametrizedAttr, TypeClass, ValueClass};
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{
    Attribute, DeriveInput, Expr, Ident, ImplGenerics, LitStr, Path, Result, Type, TypeGenerics,
    WhereClause,
};

struct ContainerAttr {
    pub strict_crate: Path,
    pub lib: Path,
    pub name: Option<LitStr>,
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
            ("crate", ArgValueReq::with_default(ident!("strict_encoding"))),
            ("lib", ArgValueReq::required(TypeClass::Path.into())),
            ("name", ArgValueReq::Optional(ValueClass::str())),
            ("encode_with", ArgValueReq::Optional(TypeClass::Path.into())),
            ("decode_with", ArgValueReq::Optional(TypeClass::Path.into())),
        ]);

        let mut params = ParametrizedAttr::with("strict_type", &attr)?;
        params.check(AttrReq::with(map))?;

        Ok(ContainerAttr {
            strict_crate: params
                .args
                .get("crate")
                .map(|a| a.clone().try_into())
                .transpose()?
                .expect("amplify_syn AttrReq guarantees"),
            lib: Path {},
            name: None,
            encode_with: None,
            decode_with: None,
        })
    }
}

struct StrictDerive<'a> {
    input: DeriveInput,
    impl_generics: ImplGenerics<'a>,
    ty_generics: TypeGenerics<'a>,
    where_clause: Option<&'a WhereClause>,

    conf: ContainerAttr,
}

impl<'a> TryFrom<DeriveInput> for StrictDerive<'a> {
    type Error = syn::Error;

    fn try_from(input: DeriveInput) -> Result<Self> {
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let conf = ContainerAttr::try_from(&input.attrs)?;
        Ok(Self {
            input,
            impl_generics,
            ty_generics,
            where_clause,
            conf,
        })
    }
}

impl StrictDerive {
    pub fn strict_dumb(&self) -> Result<TokenStream2> {}
}
