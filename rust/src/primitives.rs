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

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Display, Formatter, Write};
use std::hash::Hash;
use std::io;

use amplify::ascii::AsciiString;
use amplify::confinement::Confined;
#[cfg(feature = "float")]
use amplify::num::apfloat::{ieee, Float};
use amplify::num::{i1024, i256, i512, u1024, u24, u256, u512};

use crate::{
    DecodeError, DefineUnion, ReadTuple, ReadUnion, Sizing, StrictDecode, StrictDumb, StrictEncode,
    StrictProduct, StrictStruct, StrictSum, StrictTuple, StrictType, StrictUnion, TypeName,
    TypedRead, TypedWrite, WriteUnion, STD_LIB, STRICT_TYPES_LIB,
};

pub mod constants {
    use super::Primitive;

    pub const U8: Primitive = Primitive::unsigned(1);
    pub const U16: Primitive = Primitive::unsigned(2);
    pub const U24: Primitive = Primitive::unsigned(3);
    pub const U32: Primitive = Primitive::unsigned(4);
    pub const U48: Primitive = Primitive::unsigned(6);
    pub const U64: Primitive = Primitive::unsigned(8);
    pub const U128: Primitive = Primitive::unsigned(16);
    pub const U160: Primitive = Primitive::unsigned(20);
    pub const U256: Primitive = Primitive::unsigned(32);
    pub const U512: Primitive = Primitive::unsigned(64);
    pub const U1024: Primitive = Primitive::unsigned(128);

    pub const I8: Primitive = Primitive::signed(1);
    pub const I16: Primitive = Primitive::signed(2);
    pub const I24: Primitive = Primitive::signed(3);
    pub const I32: Primitive = Primitive::signed(4);
    pub const I48: Primitive = Primitive::signed(6);
    pub const I64: Primitive = Primitive::signed(8);
    pub const I128: Primitive = Primitive::signed(16);
    pub const I256: Primitive = Primitive::signed(32);
    pub const I512: Primitive = Primitive::signed(64);
    pub const I1024: Primitive = Primitive::signed(128);

    pub const N8: Primitive = Primitive::non_zero(1);
    pub const N16: Primitive = Primitive::non_zero(2);
    pub const N24: Primitive = Primitive::non_zero(3);
    pub const N32: Primitive = Primitive::non_zero(4);
    pub const N48: Primitive = Primitive::non_zero(6);
    pub const N64: Primitive = Primitive::non_zero(8);
    pub const N128: Primitive = Primitive::non_zero(16);

    pub const F16: Primitive = Primitive::float(2);
    pub const F32: Primitive = Primitive::float(4);
    pub const F64: Primitive = Primitive::float(8);
    pub const F80: Primitive = Primitive::float(10);
    pub const F128: Primitive = Primitive::float(16);
    pub const F256: Primitive = Primitive::float(32);

    pub const UNIT: Primitive = Primitive(0x00);
    pub const BYTE: Primitive = Primitive(0x40);
    pub const RESERVED: Primitive = Primitive(0x80);
    pub const F16B: Primitive = Primitive(0xC0);

    pub const FLOAT_RESERVED_1: Primitive = Primitive(0xC1);
    pub const FLOAT_RESERVED_2: Primitive = Primitive(0xC3);
    pub const FLOAT_RESERVED_3: Primitive = Primitive(0xC5);
    pub const FLOAT_RESERVED_4: Primitive = Primitive(0xC6);
    pub const FLOAT_RESERVED_5: Primitive = Primitive(0xC7);
    pub const FLOAT_RESERVED_6: Primitive = Primitive(0xC9);
    pub const FLOAT_RESERVED_7: Primitive = Primitive(0xCB);
    pub const FLOAT_RESERVED_8: Primitive = Primitive(0xCC);
    pub const FLOAT_RESERVED_9: Primitive = Primitive(0xCD);
    pub const FLOAT_RESERVED_10: Primitive = Primitive(0xCE);
    pub const FLOAT_RESERVED_11: Primitive = Primitive(0xCF);
    pub const FLOAT_RESERVED_12: Primitive = Primitive(0xD1);
    pub const FLOAT_RESERVED_13: Primitive = Primitive(0xD2);
    pub const FLOAT_RESERVED_14: Primitive = Primitive(0xD3);
    pub const FLOAT_RESERVED_15: Primitive = Primitive(0xD4);
    pub const FLOAT_RESERVED_16: Primitive = Primitive(0xD5);
    pub const FLOAT_RESERVED_17: Primitive = Primitive(0xD6);
    pub const FLOAT_RESERVED_18: Primitive = Primitive(0xD7);
    pub const FLOAT_RESERVED_19: Primitive = Primitive(0xD8);
    pub const FLOAT_RESERVED_20: Primitive = Primitive(0xD9);
    pub const FLOAT_RESERVED_21: Primitive = Primitive(0xDA);
    pub const FLOAT_RESERVED_22: Primitive = Primitive(0xDB);
    pub const FLOAT_RESERVED_23: Primitive = Primitive(0xDC);
    pub const FLOAT_RESERVED_24: Primitive = Primitive(0xDE);
    pub const FLOAT_RESERVED_25: Primitive = Primitive(0xDF);

    pub const FLOAT_RESERVED_26: Primitive = Primitive(0xE1);
    pub const FLOAT_RESERVED_27: Primitive = Primitive(0xE2);
    pub const FLOAT_RESERVED_28: Primitive = Primitive(0xE3);
    pub const FLOAT_RESERVED_29: Primitive = Primitive(0xE4);
    pub const FLOAT_RESERVED_30: Primitive = Primitive(0xE5);
    pub const FLOAT_RESERVED_31: Primitive = Primitive(0xE6);
    pub const FLOAT_RESERVED_32: Primitive = Primitive(0xE7);
    pub const FLOAT_RESERVED_33: Primitive = Primitive(0xE8);
    pub const FLOAT_RESERVED_34: Primitive = Primitive(0xE9);
    pub const FLOAT_RESERVED_35: Primitive = Primitive(0xEA);
    pub const FLOAT_RESERVED_36: Primitive = Primitive(0xEB);
    pub const FLOAT_RESERVED_37: Primitive = Primitive(0xEC);
    pub const FLOAT_RESERVED_38: Primitive = Primitive(0xEE);
    pub const FLOAT_RESERVED_39: Primitive = Primitive(0xEF);

    pub const FLOAT_RESERVED_40: Primitive = Primitive(0xF0);
    pub const FLOAT_RESERVED_41: Primitive = Primitive(0xF1);
    pub const FLOAT_RESERVED_42: Primitive = Primitive(0xF2);
    pub const FLOAT_RESERVED_43: Primitive = Primitive(0xF3);
    pub const FLOAT_RESERVED_44: Primitive = Primitive(0xF4);
    pub const FLOAT_RESERVED_45: Primitive = Primitive(0xF5);
    pub const FLOAT_RESERVED_46: Primitive = Primitive(0xF6);
    pub const FLOAT_RESERVED_47: Primitive = Primitive(0xF7);
    pub const FLOAT_RESERVED_48: Primitive = Primitive(0xF8);
    pub const FLOAT_RESERVED_49: Primitive = Primitive(0xF9);
    pub const FLOAT_RESERVED_50: Primitive = Primitive(0xFA);
    pub const FLOAT_RESERVED_51: Primitive = Primitive(0xFB);
    pub const FLOAT_RESERVED_52: Primitive = Primitive(0xFC);
    pub const FLOAT_RESERVED_53: Primitive = Primitive(0xFE);
    pub const FLOAT_RESERVED_54: Primitive = Primitive(0xFF);
}
use self::constants::*;

#[derive(Wrapper, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, From)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
pub struct Primitive(u8);

impl Primitive {
    pub const fn unsigned(bytes: u16) -> Self {
        Primitive(
            NumInfo {
                ty: NumCls::Unsigned,
                size: NumSize::from_bytes(bytes),
            }
            .into_code(),
        )
    }

    pub const fn signed(bytes: u16) -> Self {
        Primitive(
            NumInfo {
                ty: NumCls::Signed,
                size: NumSize::from_bytes(bytes),
            }
            .into_code(),
        )
    }

    pub const fn non_zero(bytes: u16) -> Self {
        Primitive(
            NumInfo {
                ty: NumCls::NonZero,
                size: NumSize::from_bytes(bytes),
            }
            .into_code(),
        )
    }

    pub const fn float(bytes: u16) -> Self {
        Primitive(
            NumInfo {
                ty: NumCls::Float,
                size: NumSize::from_bytes(bytes),
            }
            .into_code(),
        )
    }

    pub fn from_code(code: u8) -> Self { Primitive(code) }
    pub fn into_code(self) -> u8 { self.0 }

    pub fn info(self) -> NumInfo { NumInfo::from_code(self.0) }

    pub fn byte_size(self) -> u16 { self.info().byte_size() }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            UNIT => return f.write_str("()"),
            BYTE => return f.write_str("Byte"),
            F16B => return f.write_str("F16b"),
            RESERVED => unreachable!("reserved primitive value"),
            _ => {}
        }

        let info = self.info();
        f.write_char(match info.ty {
            NumCls::Unsigned => 'U',
            NumCls::Signed => 'I',
            NumCls::NonZero => 'N',
            NumCls::Float => 'F',
        })?;

        write!(f, "{}", info.byte_size() * 8)
    }
}

impl_strict_newtype!(Primitive, STRICT_TYPES_LIB);

/// Information about numeric type
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct NumInfo {
    /// Class of the number
    pub ty: NumCls,
    /// Size of the number, in bytes
    pub size: NumSize,
}

impl NumInfo {
    pub const fn from_code(id: u8) -> Self {
        NumInfo {
            ty: NumCls::from_code(id),
            size: NumSize::from_code(id),
        }
    }

    pub const fn into_code(self) -> u8 { self.ty.into_code() | self.size.into_code() }

    pub const fn byte_size(self) -> u16 { self.size.byte_size() }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct NumSize(NumSizeInner);

impl NumSize {
    pub(super) const fn from_bytes(bytes: u16) -> Self {
        NumSize(if bytes < 0x20 {
            NumSizeInner::Bytes(bytes as u8)
        } else if bytes % 16 != 0 {
            unreachable!()
        } else {
            NumSizeInner::Factored((bytes / 16 - 2) as u8)
        })
    }

    pub(super) const fn from_code(id: u8) -> Self {
        let code = id & 0x1F;
        NumSize(match (id & 0x20) / 0x20 {
            0 => NumSizeInner::Bytes(code),
            1 => NumSizeInner::Factored(code),
            _ => unreachable!(),
        })
    }

    pub(super) const fn into_code(self) -> u8 {
        match self.0 {
            NumSizeInner::Bytes(bytes) => bytes,
            NumSizeInner::Factored(factor) => factor | 0x20,
        }
    }

    pub const fn byte_size(self) -> u16 {
        match self.0 {
            NumSizeInner::Bytes(bytes) => bytes as u16,
            NumSizeInner::Factored(factor) => 2 * (factor as u16 + 1),
        }
    }
}

/// The way how the size is computed and encoded in the type id
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum NumSizeInner {
    /// Lowest 5 bits contain type size in bytes
    Bytes(u8),
    /// Lowest 5 bits contain a factor defining the size according to the
    /// equation `16 * (2 + factor)`
    // TODO: Ensure that U256 doesn't have two encodings with both variants
    Factored(u8),
}

/// Class of the number type
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum NumCls {
    Unsigned = 0x00,
    Signed = 0x40,
    NonZero = 0x80,
    Float = 0xC0,
}

impl NumCls {
    pub const fn from_code(id: u8) -> Self {
        match id & 0xC0 {
            x if x == NumCls::Unsigned as u8 => NumCls::Unsigned,
            x if x == NumCls::Signed as u8 => NumCls::Signed,
            x if x == NumCls::NonZero as u8 => NumCls::NonZero,
            x if x == NumCls::Float as u8 => NumCls::Float,
            _ => unreachable!(),
        }
    }

    pub const fn into_code(self) -> u8 { self as u8 }
}

macro_rules! encode_num {
    ($ty:ty, $id:ident) => {
        impl $crate::StrictType for $ty {
            const STRICT_LIB_NAME: &'static str = $crate::STD_LIB;
        }
        impl $crate::StrictEncode for $ty {
            fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
                unsafe { writer.register_primitive($id)._write_raw_array(self.to_le_bytes()) }
            }
        }
        impl $crate::StrictDecode for $ty {
            fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
                let buf = unsafe { reader._read_raw_array::<{ Self::BITS as usize / 8 }>()? };
                Ok(Self::from_le_bytes(buf))
            }
        }
    };
}

macro_rules! encode_float {
    ($ty:ty, $len:literal, $id:ident) => {
        #[cfg(feature = "float")]
        impl $crate::StrictType for $ty {
            const STRICT_LIB_NAME: &'static str = $crate::STD_LIB;
        }
        #[cfg(feature = "float")]
        impl $crate::StrictEncode for $ty {
            fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
                let mut be = [0u8; $len];
                be.copy_from_slice(&self.to_bits().to_le_bytes()[..$len]);
                unsafe { writer.register_primitive($id)._write_raw_array(be) }
            }
        }
        #[cfg(feature = "float")]
        impl $crate::StrictDecode for $ty {
            fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
                const BYTES: usize = <$ty>::BITS / 8;
                let mut inner = [0u8; 32];
                let buf = unsafe { reader._read_raw_array::<BYTES>()? };
                inner[..BYTES].copy_from_slice(&buf[..]);
                let bits = u256::from_le_bytes(inner);
                Ok(Self::from_bits(bits))
            }
        }
    };
}

encode_num!(u8, U8);
encode_num!(u16, U16);
encode_num!(u24, U24);
encode_num!(u32, U32);
encode_num!(u64, U64);
encode_num!(u128, U128);
encode_num!(u256, U256);
encode_num!(u512, U512);
encode_num!(u1024, U1024);

encode_num!(i8, I8);
encode_num!(i16, I16);
encode_num!(i32, I32);
encode_num!(i64, I64);
encode_num!(i128, I128);
encode_num!(i256, I256);
encode_num!(i512, I512);
encode_num!(i1024, I1024);

encode_float!(ieee::Half, 2, F16);
encode_float!(ieee::Single, 4, F32);
encode_float!(ieee::Double, 8, F64);
encode_float!(ieee::X87DoubleExtended, 10, F80);
encode_float!(ieee::Quad, 16, F128);
encode_float!(ieee::Oct, 32, F256);

impl<T> StrictType for Box<T>
where T: StrictType
{
    const STRICT_LIB_NAME: &'static str = T::STRICT_LIB_NAME;
}
impl<T> StrictSum for Box<T>
where T: StrictSum
{
    const ALL_VARIANTS: &'static [(u8, &'static str)] = T::ALL_VARIANTS;
    fn variant_name(&self) -> &'static str { self.as_ref().variant_name() }
}
impl<T> StrictProduct for Box<T> where T: Default + StrictProduct {}
impl<T> StrictUnion for Box<T> where T: Default + StrictUnion {}
impl<T> StrictTuple for Box<T>
where T: Default + StrictTuple
{
    const FIELD_COUNT: u8 = T::FIELD_COUNT;
}
impl<T> StrictStruct for Box<T>
where T: Default + StrictStruct
{
    const ALL_FIELDS: &'static [&'static str] = T::ALL_FIELDS;
}
impl<T> StrictEncode for Box<T>
where T: StrictEncode
{
    fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
        self.as_ref().strict_encode(writer)
    }
}
impl<T> StrictDecode for Box<T>
where T: StrictDecode
{
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        T::strict_decode(reader).map(Box::new)
    }
}

impl<T> StrictType for Option<T>
where T: StrictType
{
    const STRICT_LIB_NAME: &'static str = STD_LIB;
}
impl<T> StrictSum for Option<T>
where T: StrictType
{
    const ALL_VARIANTS: &'static [(u8, &'static str)] = &[(0u8, "none"), (1u8, "some")];
    fn variant_name(&self) -> &'static str {
        match self {
            None => "none",
            Some(_) => "some",
        }
    }
}
impl<T> StrictUnion for Option<T> where T: StrictType {}
impl<T: StrictEncode + StrictDumb> StrictEncode for Option<T> {
    fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
        writer.write_union::<Self>(|u| {
            let u = u.define_unit(vname!("none")).define_newtype::<T>(vname!("some")).complete();

            Ok(match self {
                None => u.write_unit(vname!("none")),
                Some(val) => u.write_newtype(vname!("some"), val),
            }?
            .complete())
        })
    }
}
impl<T: StrictDecode> StrictDecode for Option<T> {
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        reader.read_union(|field_name, u| match field_name.as_str() {
            "none" => Ok(None),
            "some" => u.read_tuple(|r| r.read_field().map(Some)),
            _ => unreachable!("unknown option field"),
        })
    }
}

impl<T: StrictType + Copy + StrictDumb, const LEN: usize> StrictType for [T; LEN] {
    const STRICT_LIB_NAME: &'static str = STD_LIB;
    fn strict_name() -> Option<TypeName> { None }
}
impl<T: StrictEncode + Copy + StrictDumb, const LEN: usize> StrictEncode for [T; LEN] {
    fn strict_encode<W: TypedWrite>(&self, mut writer: W) -> io::Result<W> {
        for item in self {
            writer = item.strict_encode(writer)?;
        }
        Ok(unsafe { writer.register_array(&T::strict_dumb(), LEN as u16) })
    }
}
impl<T: StrictDecode + Copy + StrictDumb, const LEN: usize> StrictDecode for [T; LEN] {
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        let mut ar = [T::strict_dumb(); LEN];
        for c in ar.iter_mut() {
            *c = T::strict_decode(reader)?;
        }
        Ok(ar)
    }
}

impl<const MIN_LEN: usize, const MAX_LEN: usize> StrictType for Confined<String, MIN_LEN, MAX_LEN> {
    const STRICT_LIB_NAME: &'static str = STD_LIB;
    fn strict_name() -> Option<TypeName> { None }
}
impl<const MIN_LEN: usize, const MAX_LEN: usize> StrictEncode
    for Confined<String, MIN_LEN, MAX_LEN>
{
    fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
        unsafe {
            writer
                .register_unicode(Sizing::new(MIN_LEN as u16, MAX_LEN as u16))
                .write_string::<MAX_LEN>(self.as_bytes())
        }
    }
}
impl<const MIN_LEN: usize, const MAX_LEN: usize> StrictDecode
    for Confined<String, MIN_LEN, MAX_LEN>
{
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        let bytes = unsafe { reader.read_string::<MAX_LEN>()? };
        let s = String::from_utf8(bytes)?;
        Confined::try_from(s).map_err(DecodeError::from)
    }
}

impl<const MIN_LEN: usize, const MAX_LEN: usize> StrictType
    for Confined<AsciiString, MIN_LEN, MAX_LEN>
{
    const STRICT_LIB_NAME: &'static str = STD_LIB;
    fn strict_name() -> Option<TypeName> { None }
}
impl<const MIN_LEN: usize, const MAX_LEN: usize> StrictEncode
    for Confined<AsciiString, MIN_LEN, MAX_LEN>
{
    fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
        unsafe {
            writer
                .register_ascii(Sizing::new(MIN_LEN as u16, MAX_LEN as u16))
                .write_string::<MAX_LEN>(self.as_bytes())
        }
    }
}
impl<const MIN_LEN: usize, const MAX_LEN: usize> StrictDecode
    for Confined<AsciiString, MIN_LEN, MAX_LEN>
{
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        let bytes = unsafe { reader.read_string::<MAX_LEN>()? };
        let s = AsciiString::from_ascii(bytes).map_err(|err| err.ascii_error())?;
        Confined::try_from(s).map_err(DecodeError::from)
    }
}

impl<T: StrictType, const MIN_LEN: usize, const MAX_LEN: usize> StrictType
    for Confined<Vec<T>, MIN_LEN, MAX_LEN>
{
    const STRICT_LIB_NAME: &'static str = STD_LIB;
    fn strict_name() -> Option<TypeName> { None }
}
impl<T: StrictEncode + StrictDumb, const MIN_LEN: usize, const MAX_LEN: usize> StrictEncode
    for Confined<Vec<T>, MIN_LEN, MAX_LEN>
{
    fn strict_encode<W: TypedWrite>(&self, mut writer: W) -> io::Result<W> {
        unsafe {
            writer = writer.write_collection::<Vec<T>, MIN_LEN, MAX_LEN>(self)?;
        }
        Ok(unsafe {
            writer.register_list(&T::strict_dumb(), Sizing::new(MIN_LEN as u16, MAX_LEN as u16))
        })
    }
}
impl<T: StrictDecode, const MIN_LEN: usize, const MAX_LEN: usize> StrictDecode
    for Confined<Vec<T>, MIN_LEN, MAX_LEN>
{
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        let len = unsafe { reader._read_raw_len::<MAX_LEN>()? };
        let mut col = Vec::<T>::with_capacity(len);
        for _ in 0..len {
            col.push(StrictDecode::strict_decode(reader)?);
        }
        Confined::try_from(col).map_err(DecodeError::from)
    }
}

impl<T: StrictType + Ord, const MIN_LEN: usize, const MAX_LEN: usize> StrictType
    for Confined<BTreeSet<T>, MIN_LEN, MAX_LEN>
{
    const STRICT_LIB_NAME: &'static str = STD_LIB;
    fn strict_name() -> Option<TypeName> { None }
}
impl<T: StrictEncode + Ord + StrictDumb, const MIN_LEN: usize, const MAX_LEN: usize> StrictEncode
    for Confined<BTreeSet<T>, MIN_LEN, MAX_LEN>
{
    fn strict_encode<W: TypedWrite>(&self, mut writer: W) -> io::Result<W> {
        unsafe {
            writer = writer.write_collection::<BTreeSet<T>, MIN_LEN, MAX_LEN>(self)?;
        }
        Ok(unsafe {
            writer.register_set(&T::strict_dumb(), Sizing::new(MIN_LEN as u16, MAX_LEN as u16))
        })
    }
}
impl<T: StrictDecode + Ord, const MIN_LEN: usize, const MAX_LEN: usize> StrictDecode
    for Confined<BTreeSet<T>, MIN_LEN, MAX_LEN>
{
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        let len = unsafe { reader._read_raw_len::<MAX_LEN>()? };
        let mut col = BTreeSet::<T>::new();
        for _ in 0..len {
            let item = StrictDecode::strict_decode(reader)?;
            if matches!(col.last(), Some(last) if last > &item) {
                return Err(DecodeError::BrokenSetOrder);
            }
            if !col.insert(item) {
                return Err(DecodeError::RepeatedSetValue);
            }
        }
        Confined::try_from(col).map_err(DecodeError::from)
    }
}

impl<K: StrictType + Ord + Hash, V: StrictType, const MIN_LEN: usize, const MAX_LEN: usize>
    StrictType for Confined<BTreeMap<K, V>, MIN_LEN, MAX_LEN>
{
    const STRICT_LIB_NAME: &'static str = STD_LIB;
    fn strict_name() -> Option<TypeName> { None }
}
impl<
        K: StrictEncode + Ord + Hash + StrictDumb,
        V: StrictEncode + StrictDumb,
        const MIN_LEN: usize,
        const MAX_LEN: usize,
    > StrictEncode for Confined<BTreeMap<K, V>, MIN_LEN, MAX_LEN>
{
    fn strict_encode<W: TypedWrite>(&self, mut writer: W) -> io::Result<W> {
        unsafe {
            writer = writer._write_raw_len::<MAX_LEN>(self.len())?;
        }
        for (k, v) in self {
            writer = k.strict_encode(writer)?;
            writer = v.strict_encode(writer)?
        }
        Ok(unsafe {
            writer.register_map(
                &K::strict_dumb(),
                &V::strict_dumb(),
                Sizing::new(MIN_LEN as u16, MAX_LEN as u16),
            )
        })
    }
}
impl<
        K: StrictDecode + Ord + Hash + StrictDumb,
        V: StrictDecode + StrictDumb,
        const MIN_LEN: usize,
        const MAX_LEN: usize,
    > StrictDecode for Confined<BTreeMap<K, V>, MIN_LEN, MAX_LEN>
{
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        let len = unsafe { reader._read_raw_len::<MAX_LEN>()? };
        let mut col = BTreeMap::new();
        for _ in 0..len {
            let key = StrictDecode::strict_decode(reader)?;
            let val = StrictDecode::strict_decode(reader)?;
            if matches!(col.last_key_value(), Some((last, _)) if last > &key) {
                return Err(DecodeError::BrokenMapOrder);
            }
            if col.insert(key, val).is_some() {
                return Err(DecodeError::RepeatedMapValue);
            }
        }
        Confined::try_from(col).map_err(DecodeError::from)
    }
}

#[cfg(test)]
mod test {
    use super::constants::U8;

    #[test]
    fn u8() {
        let prim = U8;
        assert_eq!(prim.byte_size(), 1);
    }
}
