// Strict encoding library for deterministic binary serialization.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2024 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright 2022-2024 UBIDECO Labs
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

use core::ops::Range;

use amplify::confinement;

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[display(inner)]
// TODO: Replace with no-I/O variant from amplify when ready
pub struct ReadError(String);

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[display(inner)]
// TODO: Replace with no-I/O variant from amplify when ready
pub struct WriteError(String);

#[cfg(feature = "std")]
impl From<std::io::Error> for ReadError {
    fn from(err: std::io::Error) -> Self { Self(err.to_string()) }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for WriteError {
    fn from(err: std::io::Error) -> Self { Self(err.to_string()) }
}

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum DecodeError {
    #[from]
    #[display(inner)]
    Read(ReadError),

    /// confinement requirements are not satisfied. Specifically, {0}
    #[from]
    Confinement(confinement::Error),

    /// non-zero natural number can't have a value equal to zero.
    ZeroNatural,

    /// string data are not in valid UTF-8 encoding.\nDetails: {0}
    #[from]
    Utf8(alloc::string::FromUtf8Error),

    /// string data are not in valid UTF-8 encoding.\nDetails: {0}
    #[from]
    Ascii(amplify::ascii::AsAsciiStrError),

    /// value occurs multiple times in a set
    RepeatedSetValue,

    /// key occurs multiple times in a map
    RepeatedMapValue,

    /// unsupported value `{1}` for enum `{0}` encountered during decode
    /// operation
    EnumTagNotKnown(String, u8),

    /// unsupported value `{1}` for union `{0}` encountered during decode
    /// operation
    UnionTagNotKnown(String, u8),

    /// decoding resulted in value `{2}` for type `{0}` that exceeds the
    /// supported range {1:#?}
    ValueOutOfRange(String, Range<u128>, u128),

    /// encoded values are not deterministically ordered within a set
    BrokenSetOrder,

    /// encoded map has wrong order of keys
    BrokenMapOrder,

    /// data integrity problem during strict decoding operation.\nDetails: {0}
    DataIntegrityError(String),
}

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum SerializeError {
    #[from]
    #[display(inner)]
    Write(WriteError),

    /// confinement requirements are not satisfied. Specifically, {0}
    #[from]
    Confinement(confinement::Error),
}

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum DeserializeError {
    #[from(ReadError)]
    #[display(inner)]
    Decode(DecodeError),

    /// data are not entirely consumed during strict deserialize operation
    DataNotEntirelyConsumed,
}
