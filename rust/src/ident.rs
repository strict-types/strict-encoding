// Strict encoding library for deterministic binary serialization.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2024 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright 2022-2024 UBIDECO Institute
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

use std::fmt::{self, Debug, Formatter};
use std::str::FromStr;

use amplify::Wrapper;

use crate::stl::{AlphaCapsLodash, AlphaLodash, AlphaNumLodash, AlphaSmallLodash};
use crate::{impl_strict_newtype, InvalidRString, RString, STRICT_TYPES_LIB};

#[macro_export]
macro_rules! impl_ident_type {
    ($ty:ty) => {
        impl From<$ty> for String {
            fn from(ident: $ty) -> String { ident.0.into() }
        }

        impl From<&'static str> for $ty {
            fn from(ident: &'static str) -> Self { Self(RString::from(ident)) }
        }

        impl TryFrom<String> for $ty {
            type Error = InvalidRString;

            fn try_from(s: String) -> Result<Self, Self::Error> { Self::from_str(&s) }
        }

        impl Debug for $ty {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.debug_tuple(&$crate::type_name::<Self>())
                    .field(&self.as_str())
                    .finish()
            }
        }

        impl $ty {
            pub fn as_str(&self) -> &str { self.0.as_str() }
        }
    };
}

#[macro_export]
macro_rules! impl_ident_subtype {
    ($ty:ty) => {
        impl From<$ty> for Ident {
            fn from(name: $ty) -> Self {
                Ident::from_str(name.as_str()).expect("ident is a superset")
            }
        }

        impl $ty {
            pub fn to_ident(&self) -> Ident { self.clone().into() }
            pub fn into_ident(self) -> Ident { self.into() }
        }
    };
}

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display, FromStr)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct Ident(RString<AlphaLodash, AlphaNumLodash, 1, 100>);

impl_ident_type!(Ident);
impl_strict_newtype!(Ident, STRICT_TYPES_LIB);

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display, FromStr)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct TypeName(RString<AlphaCapsLodash, AlphaNumLodash, 1, 100>);

impl_ident_type!(TypeName);
impl_ident_subtype!(TypeName);
impl_strict_newtype!(TypeName, STRICT_TYPES_LIB);

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display, FromStr)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct FieldName(RString<AlphaSmallLodash, AlphaNumLodash, 1, 100>);

impl_ident_type!(FieldName);
impl_ident_subtype!(FieldName);
impl_strict_newtype!(FieldName, STRICT_TYPES_LIB);

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display, FromStr)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct VariantName(RString<AlphaSmallLodash, AlphaNumLodash, 1, 100>);

impl_ident_type!(VariantName);
impl_ident_subtype!(VariantName);
impl_strict_newtype!(VariantName, STRICT_TYPES_LIB);

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display, FromStr)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct LibName(RString<AlphaCapsLodash, AlphaNumLodash, 1, 100>);

impl_ident_type!(LibName);
impl_ident_subtype!(LibName);
impl_strict_newtype!(LibName, STRICT_TYPES_LIB);
