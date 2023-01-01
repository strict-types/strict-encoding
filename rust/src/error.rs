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

use std::io;
use std::ops::Range;

use amplify::{confinement, IoError};

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum DecodeError {
    #[display(inner)]
    #[from(io::Error)]
    Io(IoError),

    /// confinement requirements are not satisfied. Specifically, {0}
    #[from]
    Confinement(confinement::Error),

    /// string data are not in valid UTF-8 encoding.\nDetails: {0}
    #[from]
    Utf8(std::string::FromUtf8Error),

    /// string data are not in valid UTF-8 encoding.\nDetails: {0}
    #[from]
    Ascii(amplify::ascii::AsAsciiStrError),

    /// the value {0} occurs multiple times in a set
    RepeatedSetValue(String),

    /// unsupported value `{1}` for enum `{0}` encountered during decode
    /// operation
    EnumValueNotKnown(String, u8),

    /// unsupported value `{1}` for union `{0}` encountered during decode
    /// operation
    UnionValueNotKnown(String, u8),

    /// decoding resulted in value `{2}` for type `{0}` that exceeds the
    /// supported range {1:#?}
    ValueOutOfRange(String, Range<u128>, u128),

    /// encoded values are not deterministically ordered within a set {name}:
    /// value `{prev}` precedes `{next}`
    BrokenSetOrder {
        name: String,
        prev: String,
        next: String,
    },

    /// encoded map {name} has wrong order of keys: key `{prev}` precedes
    /// `{next}`
    BrokenMapOrder {
        name: String,
        prev: String,
        next: String,
    },

    /// data integrity problem during strict decoding operation.\nDetails: {0}
    DataIntegrityError(String),
}

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum SerializeError {
    #[display(inner)]
    #[from(io::Error)]
    Io(IoError),

    /// confinement requirements are not satisfied. Specifically, {0}
    #[from]
    Confinement(confinement::Error),
}

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum DeserializeError {
    #[display(inner)]
    #[from]
    #[from(io::Error)]
    Decode(DecodeError),

    /// data are not entirely consumed during strict deserialize operation
    DataNotEntirelyConsumed,
}
