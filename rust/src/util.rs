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

use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::io;

use crate::{ReadStruct, VariantName, WriteStruct, STRICT_TYPES_LIB};

// TODO: Control that min > max!
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
pub struct Sizing {
    pub min: u64,
    pub max: u64,
}
impl_strict_struct!(Sizing, STRICT_TYPES_LIB; min, max);

impl Sizing {
    // TODO: Remove (in strict-types use Array with size 1 for char types instead).
    pub const ONE: Sizing = Sizing { min: 1, max: 1 };

    pub const U8: Sizing = Sizing {
        min: 0,
        max: u8::MAX as u64,
    };

    pub const U16: Sizing = Sizing {
        min: 0,
        max: u16::MAX as u64,
    };

    pub const U8_NONEMPTY: Sizing = Sizing {
        min: 1,
        max: u8::MAX as u64,
    };

    pub const U16_NONEMPTY: Sizing = Sizing {
        min: 1,
        max: u16::MAX as u64,
    };

    pub const fn new(min: u64, max: u64) -> Self { Sizing { min, max } }

    pub const fn fixed(len: u64) -> Self { Sizing { min: len, max: len } }

    pub const fn is_fixed(&self) -> bool { self.min == self.max }

    pub const fn check(&self, len: usize) -> bool {
        let len = len as u64;
        len >= self.min && len <= self.max
    }
}

impl Display for Sizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match (self.min, self.max) {
            (0, 0xFFFF) => Ok(()),
            (min, max) if min == max => write!(f, " ^ {min}"),
            (0, max) => write!(f, " ^ ..{max:#x}"),
            (min, 0xFFFF) => write!(f, " ^ {min}.."),
            (min, max) => write!(f, " ^ {min}..{max:#x}"),
        }
    }
}

#[derive(Clone, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
pub struct Variant {
    pub name: VariantName,
    pub tag: u8,
}
impl_strict_struct!(Variant, STRICT_TYPES_LIB; name, tag);

impl Variant {
    pub fn named(tag: u8, name: VariantName) -> Variant { Variant { name, tag } }

    pub fn none() -> Variant {
        Variant {
            name: vname!("none"),
            tag: 0,
        }
    }
    pub fn some() -> Variant {
        Variant {
            name: vname!("some"),
            tag: 1,
        }
    }
}

impl PartialEq for Variant {
    fn eq(&self, other: &Self) -> bool { self.tag == other.tag || self.name == other.name }
}

impl PartialOrd for Variant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Hash for Variant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
        self.name.hash(state);
    }
}

impl Ord for Variant {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        self.tag.cmp(&other.tag)
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        Ok(())
    }
}
