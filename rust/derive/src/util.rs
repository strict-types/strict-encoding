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

use amplify_syn::{FieldKind, Items, NamedField};
use heck::ToLowerCamelCase;
use proc_macro2::{Ident, Span};
use syn::Result;

use crate::params::FieldAttr;

pub trait NamedFieldsExt {
    fn field_names(&self) -> Result<Vec<Ident>>;
}

impl NamedFieldsExt for Items<NamedField> {
    fn field_names(&self) -> Result<Vec<Ident>> {
        let mut name = Vec::with_capacity(self.len());
        for named_field in self {
            let attr = FieldAttr::with(named_field.field.attr.clone(), FieldKind::Named)?;
            name.push(match attr.rename {
                None => {
                    let s = named_field.name.to_string().to_lower_camel_case();
                    Ident::new(&s, Span::call_site())
                }
                Some(name) => name,
            });
        }
        Ok(name)
    }
}
