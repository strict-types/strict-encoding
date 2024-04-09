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

use amplify::ascii::AsciiString;
use amplify::confinement::Confined;
use amplify::Wrapper;

use crate::stl::{AlphaLodash, AlphaNumLodash};
use crate::{
    impl_strict_newtype, DecodeError, InvalidRString, RString, ReadTuple, StrictDecode, StrictDumb,
    StrictEncode, StrictProduct, StrictTuple, StrictType, TypedRead, TypedWrite, STRICT_TYPES_LIB,
};

/// Identifier (field or type name).
#[derive(Wrapper, WrapperMut, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display)]
#[wrapper_mut(DerefMut)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct Ident(Confined<AsciiString, 1, 100>);

impl FromStr for Ident {
    type Err = InvalidRString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = AsciiString::from_ascii(s.as_bytes())?;
        Ident::try_from(s)
    }
}

impl From<&'static str> for Ident {
    fn from(s: &'static str) -> Self { Self::from_str(s).expect("invalid identifier name") }
}

impl TryFrom<String> for Ident {
    type Error = InvalidRString;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let s = AsciiString::from_ascii(s.as_bytes())?;
        Ident::try_from(s)
    }
}

impl TryFrom<AsciiString> for Ident {
    type Error = InvalidRString;

    fn try_from(ascii: AsciiString) -> Result<Self, InvalidRString> {
        if ascii.is_empty() {
            return Err(InvalidRString::Empty);
        }
        let first = ascii[0];
        if !first.is_alphabetic() && first != '_' {
            return Err(InvalidRString::DisallowedFirst(ascii.clone(), first));
        }
        if let Some(ch) = ascii
            .as_slice()
            .iter()
            .copied()
            .find(|ch| !ch.is_ascii_alphanumeric() && *ch != '_')
        {
            return Err(InvalidRString::InvalidChar(ascii.clone(), ch));
        }
        let s = Confined::try_from(ascii)?;
        Ok(Self(s))
    }
}

impl Debug for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Ident").field(&self.as_str()).finish()
    }
}

impl Ident {
    pub fn from_uint(val: impl Into<u64>) -> Ident {
        Self::try_from(format!("_{}", val.into())).expect("always a valid identifier")
    }
}

impl StrictDumb for Ident {
    fn strict_dumb() -> Self { Self::from("Dumb") }
}
impl StrictType for Ident {
    const STRICT_LIB_NAME: &'static str = STRICT_TYPES_LIB;
}
impl StrictProduct for Ident {}
impl StrictTuple for Ident {
    const FIELD_COUNT: u8 = 1;
}
impl StrictEncode for Ident {
    fn strict_encode<W: TypedWrite>(&self, writer: W) -> std::io::Result<W> {
        let s = RString::<AlphaLodash, AlphaNumLodash, 1, 100>::try_from(self.0.as_bytes())
            .expect("invalid Ident value when invariant is expected");
        writer.write_newtype::<Self>(&s)
    }
}
impl StrictDecode for Ident {
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        reader.read_tuple(|r| {
            let s: RString<AlphaLodash, AlphaNumLodash, 1, 100> = r.read_field()?;
            Ok(Self(
                Confined::try_from(
                    AsciiString::from_ascii(s.as_bytes())
                        .expect("restricted character set mut be a valid ASCII chars"),
                )
                .expect("Ident requirements on sizing differs from generic"),
            ))
        })
    }
}

#[derive(Wrapper, WrapperMut, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
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
    type Error = InvalidRString;

    fn try_from(s: String) -> Result<Self, Self::Error> { Self::from_str(&s) }
}

impl Debug for TypeName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TypeName").field(&self.as_str()).finish()
    }
}

impl TypeName {
    pub fn as_str(&self) -> &str { self.0.as_str() }
    pub fn as_ident(&self) -> &Ident { &self.0 }
    pub fn to_ident(&self) -> Ident { self.clone().into() }
    pub fn into_ident(self) -> Ident { self.into() }
}

impl_strict_newtype!(TypeName, STRICT_TYPES_LIB);

#[derive(Wrapper, WrapperMut, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
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
    type Error = InvalidRString;

    fn try_from(s: String) -> Result<Self, Self::Error> { Self::from_str(&s) }
}

impl Debug for FieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FieldName").field(&self.as_str()).finish()
    }
}

impl FieldName {
    pub fn as_str(&self) -> &str { self.0.as_str() }
    pub fn as_ident(&self) -> &Ident { &self.0 }
    pub fn to_ident(&self) -> Ident { self.clone().into() }
    pub fn into_ident(self) -> Ident { self.into() }
}

impl_strict_newtype!(FieldName, STRICT_TYPES_LIB);

#[derive(Wrapper, WrapperMut, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display, FromStr)]
#[wrapper_mut(DerefMut)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct VariantName(Ident);

impl From<&'static str> for VariantName {
    fn from(ident: &'static str) -> Self { VariantName(Ident::from(ident)) }
}

impl TryFrom<String> for VariantName {
    type Error = InvalidRString;

    fn try_from(s: String) -> Result<Self, Self::Error> { Self::from_str(&s) }
}

impl Debug for VariantName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("VariantName").field(&self.as_str()).finish()
    }
}

impl VariantName {
    pub fn as_str(&self) -> &str { self.0.as_str() }
    pub fn as_ident(&self) -> &Ident { &self.0 }
    pub fn to_ident(&self) -> Ident { self.clone().into() }
    pub fn into_ident(self) -> Ident { self.into() }
}

impl_strict_newtype!(VariantName, STRICT_TYPES_LIB);

#[derive(Wrapper, WrapperMut, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
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
    type Error = InvalidRString;

    fn try_from(s: String) -> Result<Self, Self::Error> { Self::from_str(&s) }
}

impl Debug for LibName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("LibName").field(&self.as_str()).finish()
    }
}

impl LibName {
    pub fn as_str(&self) -> &str { self.0.as_str() }
    pub fn as_ident(&self) -> &Ident { &self.0 }
    pub fn to_ident(&self) -> Ident { self.clone().into() }
    pub fn into_ident(self) -> Ident { self.into() }
}

impl_strict_newtype!(LibName, STRICT_TYPES_LIB);
