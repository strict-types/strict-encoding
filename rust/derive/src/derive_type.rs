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

struct DeriveType<'a>(&'a StrictDerive);

impl StrictDerive {
    pub fn derive_type(&self) -> Result<TokenStream2> {
        self.data.derive(self.conf.strict_crate.clone(), ident!(StrictType), &DeriveType(self))
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

    fn derive_struct_inner(&self, fields: &Items<NamedField>) -> Result<TokenStream2> {
        self.derive_type()
    }

    fn derive_tuple_inner(&self, fields: &Items<Field>) -> Result<TokenStream2> {
        self.derive_type()
    }

    fn derive_enum_inner(&self, variants: &Items<Variant>) -> Result<TokenStream2> {
        self.derive_type()
    }
}
