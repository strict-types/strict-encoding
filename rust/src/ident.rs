// Strict encoding library for deterministic binary serialization.
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

use std::str::FromStr;

use amplify::ascii::{AsAsciiStrError, AsciiChar, AsciiString, FromAsciiError};
use amplify::confinement::Confined;
use amplify::{confinement, Wrapper};

use crate::{impl_strict_newtype, STRICT_TYPES_LIB};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum InvalidIdent {
    /// ident must contain at least one character
    Empty,

    /// identifier name must start with alphabetic character and not `{0}`
    NonAlphabetic(AsciiChar),

    /// identifier name contains invalid character `{0}`
    InvalidChar(AsciiChar),

    #[from(AsAsciiStrError)]
    /// identifier name contains non-ASCII character(s)
    NonAsciiChar,

    /// identifier name has invalid length
    #[from]
    Confinement(confinement::Error),
}

impl<O> From<FromAsciiError<O>> for InvalidIdent {
    fn from(_: FromAsciiError<O>) -> Self { InvalidIdent::NonAsciiChar }
}

/// Identifier (field or type name).
#[derive(Wrapper, WrapperMut, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, From)]
#[wrapper(Deref, Display)]
#[wrapper_mut(DerefMut)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct Ident(Confined<AsciiString, 1, 32>);

impl FromStr for Ident {
    type Err = InvalidIdent;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = AsciiString::from_ascii(s.as_bytes())?;
        Ident::try_from(s)
    }
}

impl From<&'static str> for Ident {
    fn from(s: &'static str) -> Self { Self::from_str(s).expect("invalid identifier name") }
}

impl TryFrom<String> for Ident {
    type Error = InvalidIdent;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let s = AsciiString::from_ascii(s.as_bytes())?;
        Ident::try_from(s)
    }
}

impl TryFrom<AsciiString> for Ident {
    type Error = InvalidIdent;

    fn try_from(ascii: AsciiString) -> Result<Self, InvalidIdent> {
        if ascii.is_empty() {
            return Err(InvalidIdent::Empty);
        }
        let first = ascii[0];
        if !first.is_alphabetic() && first != '_' {
            return Err(InvalidIdent::NonAlphabetic(first));
        }
        if let Some(ch) =
            ascii.as_slice().iter().copied().find(|ch| !ch.is_ascii_alphanumeric() && *ch != b'_')
        {
            return Err(InvalidIdent::InvalidChar(ch));
        }
        let s = Confined::try_from(ascii)?;
        Ok(Self(s))
    }
}

impl_strict_newtype!(Ident, STRICT_TYPES_LIB, "Dumb");

#[derive(Wrapper, WrapperMut, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, From)]
#[wrapper(Deref, Display, FromStr)]
#[wrapper_mut(DerefMut)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct TypeName(Ident);

impl From<&'static str> for TypeName {
    fn from(ident: &'static str) -> Self { TypeName(Ident::from(ident)) }
}

impl TryFrom<String> for TypeName {
    type Error = InvalidIdent;

    fn try_from(s: String) -> Result<Self, Self::Error> { Ident::try_from(s).map(Self) }
}

impl_strict_newtype!(TypeName, STRICT_TYPES_LIB);

#[derive(Wrapper, WrapperMut, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, From)]
#[wrapper(Deref, Display, FromStr)]
#[wrapper_mut(DerefMut)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct FieldName(Ident);

impl From<&'static str> for FieldName {
    fn from(ident: &'static str) -> Self { FieldName(Ident::from(ident)) }
}

impl TryFrom<String> for FieldName {
    type Error = InvalidIdent;

    fn try_from(s: String) -> Result<Self, Self::Error> { Ident::try_from(s).map(Self) }
}

impl_strict_newtype!(FieldName, STRICT_TYPES_LIB);

pub type VariantName = FieldName;

#[derive(Wrapper, WrapperMut, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, From)]
#[wrapper(Deref, Display, FromStr)]
#[wrapper_mut(DerefMut)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct LibName(Ident);

impl From<&'static str> for LibName {
    fn from(ident: &'static str) -> Self { LibName(Ident::from(ident)) }
}

impl TryFrom<String> for LibName {
    type Error = InvalidIdent;

    fn try_from(s: String) -> Result<Self, Self::Error> { Ident::try_from(s).map(Self) }
}

impl_strict_newtype!(LibName, STRICT_TYPES_LIB);
