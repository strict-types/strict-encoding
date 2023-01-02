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

use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::io;

use crate::{
    DecodeError, FieldName, ReadStruct, StrictDecode, StrictDumb, StrictEncode, StrictProduct,
    StrictStruct, StrictType, TypedRead, TypedWrite, WriteStruct, STEN_LIB,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
pub struct Sizing {
    pub min: u16,
    pub max: u16,
}

impl Sizing {
    pub const ONE: Sizing = Sizing { min: 1, max: 1 };

    pub const U8: Sizing = Sizing {
        min: 0,
        max: u8::MAX as u16,
    };

    pub const U16: Sizing = Sizing {
        min: 0,
        max: u16::MAX,
    };

    pub const U8_NONEMPTY: Sizing = Sizing {
        min: 1,
        max: u8::MAX as u16,
    };

    pub const U16_NONEMPTY: Sizing = Sizing {
        min: 1,
        max: u16::MAX,
    };

    pub const fn new(min: u16, max: u16) -> Self { Sizing { min, max } }

    pub const fn fixed(len: u16) -> Self { Sizing { min: len, max: len } }
}

impl Display for Sizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match (self.min, self.max) {
            (0, u16::MAX) => Ok(()),
            (0, max) => write!(f, " ^ ..{}", max),
            (min, u16::MAX) => write!(f, " ^ {}..", min),
            (min, max) => write!(f, " ^ {}..{:#04x}", min, max),
        }
    }
}

impl StrictDumb for Sizing {
    fn strict_dumb() -> Self { Sizing::U16 }
}
impl StrictType for Sizing {
    const STRICT_LIB_NAME: &'static str = STEN_LIB;
}
impl StrictProduct for Sizing {}
impl StrictStruct for Sizing {
    const ALL_FIELDS: &'static [&'static str] = &["min", "max"];
}
impl StrictEncode for Sizing {
    fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
        writer.write_struct::<Self>(|w| {
            Ok(w.write_field(fname!("min"), &self.min)?
                .write_field(fname!("max"), &self.max)?
                .complete())
        })
    }
}
impl StrictDecode for Sizing {
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        reader.read_struct(|r| {
            let min = r.read_field(fname!("min"))?;
            let max = r.read_field(fname!("max"))?;
            Ok(Sizing { min, max })
        })
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
pub struct Variant {
    pub name: FieldName,
    pub ord: u8,
}

impl Variant {
    pub fn named(name: FieldName, value: u8) -> Variant { Variant { name, ord: value } }

    pub fn none() -> Variant {
        Variant {
            name: fname!("none"),
            ord: 0,
        }
    }
    pub fn some() -> Variant {
        Variant {
            name: fname!("some"),
            ord: 1,
        }
    }
}

impl PartialOrd for Variant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Variant {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        self.ord.cmp(&other.ord)
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if f.alternate() {
            f.write_str(" = ")?;
            Display::fmt(&self.ord, f)?;
        }
        Ok(())
    }
}

impl StrictDumb for Variant {
    fn strict_dumb() -> Self {
        Variant {
            name: fname!("dumb"),
            ord: 0,
        }
    }
}
impl StrictType for Variant {
    const STRICT_LIB_NAME: &'static str = STEN_LIB;
}
impl StrictProduct for Variant {}
impl StrictStruct for Variant {
    const ALL_FIELDS: &'static [&'static str] = &["name", "ord"];
}
impl StrictEncode for Variant {
    fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
        writer.write_struct::<Self>(|w| {
            Ok(w.write_field(fname!("name"), &self.name)?
                .write_field(fname!("ord"), &self.ord)?
                .complete())
        })
    }
}
impl StrictDecode for Variant {
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        reader.read_struct(|r| {
            let name = r.read_field(fname!("name"))?;
            let ord = r.read_field(fname!("ord"))?;
            Ok(Variant { name, ord })
        })
    }
}
