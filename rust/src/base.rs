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
use std::hash::Hash;
use std::io;

use amplify::ascii::AsciiString;
use amplify::confinement::Confined;
use amplify::num::apfloat::{ieee, Float};
use amplify::num::{i1024, i256, i512, u1024, u24, u256, u512};

use crate::encoding::{
    DefineUnion, StrictEncode, StrictSum, StrictType, StrictUnion, TypedWrite, WriteUnion,
};
use crate::primitive::constants::*;
use crate::util::Sizing;

const STD_LIB: &'static str = "StdLib";

macro_rules! encode_num {
    ($ty:ty, $id:ident) => {
        impl StrictEncode for $ty {
            unsafe fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
                unsafe { writer.register_primitive($id).write_raw_array(self.to_le_bytes()) }
            }
        }
    };
}

macro_rules! encode_float {
    ($ty:ty, $len:literal, $id:ident) => {
        impl StrictEncode for $ty {
            unsafe fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
                let mut be = [0u8; $len];
                be.copy_from_slice(&self.to_bits().to_le_bytes()[..$len]);
                unsafe { writer.register_primitive($id).write_raw_array(be) }
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
impl<T> StrictUnion for Option<T> where T: StrictUnion {}

impl<T: StrictEncode> StrictEncode for Option<T> {
    unsafe fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
        writer.write_union(libname!(STD_LIB), None, |u| {
            let u = u.define_unit(fname!("none")).define_type::<T>(fname!("some")).complete();

            Ok(match self {
                None => u.write_unit(fname!("none")),
                Some(val) => u.write_type(fname!("some"), val),
            }?
            .complete())
        })
    }
}

impl<T: StrictEncode + Copy, const LEN: usize> StrictEncode for [T; LEN] {
    unsafe fn strict_encode<W: TypedWrite>(&self, mut writer: W) -> io::Result<W> {
        unsafe {
            for item in self {
                writer = item.strict_encode(writer)?;
            }
        }
        Ok(writer.register_array(&T::strict_encode_dumb(), LEN as u16))
    }
}

impl<const MIN_LEN: usize, const MAX_LEN: usize> StrictEncode
    for Confined<String, MIN_LEN, MAX_LEN>
{
    unsafe fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
        unsafe {
            writer
                .register_unicode(Sizing::new(MIN_LEN as u16, MAX_LEN as u16))
                .write_raw_bytes::<MAX_LEN>(self.as_bytes())
        }
    }
}

impl<const MIN_LEN: usize, const MAX_LEN: usize> StrictEncode
    for Confined<AsciiString, MIN_LEN, MAX_LEN>
{
    unsafe fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
        unsafe {
            writer
                .register_ascii(Sizing::new(MIN_LEN as u16, MAX_LEN as u16))
                .write_raw_bytes::<MAX_LEN>(self.as_bytes())
        }
    }
}

impl<T: StrictEncode, const MIN_LEN: usize, const MAX_LEN: usize> StrictEncode
    for Confined<Vec<T>, MIN_LEN, MAX_LEN>
{
    unsafe fn strict_encode<W: TypedWrite>(&self, mut writer: W) -> io::Result<W> {
        unsafe {
            writer = writer.write_raw_collection::<Vec<T>, MIN_LEN, MAX_LEN>(self)?;
        }
        Ok(writer
            .register_list(&T::strict_encode_dumb(), Sizing::new(MIN_LEN as u16, MAX_LEN as u16)))
    }
}

impl<T: StrictEncode + Ord, const MIN_LEN: usize, const MAX_LEN: usize> StrictEncode
    for Confined<BTreeSet<T>, MIN_LEN, MAX_LEN>
{
    unsafe fn strict_encode<W: TypedWrite>(&self, mut writer: W) -> io::Result<W> {
        unsafe {
            writer = writer.write_raw_collection::<BTreeSet<T>, MIN_LEN, MAX_LEN>(self)?;
        }
        Ok(writer
            .register_set(&T::strict_encode_dumb(), Sizing::new(MIN_LEN as u16, MAX_LEN as u16)))
    }
}

impl<K: StrictEncode + Ord + Hash, const MIN_LEN: usize, V: StrictEncode, const MAX_LEN: usize>
    StrictEncode for Confined<BTreeMap<K, V>, MIN_LEN, MAX_LEN>
{
    unsafe fn strict_encode<W: TypedWrite>(&self, mut writer: W) -> io::Result<W> {
        unsafe {
            writer = writer.write_raw_len::<MAX_LEN>(self.len())?;
        }
        for (k, v) in self {
            writer = k.strict_encode(writer)?;
            writer = v.strict_encode(writer)?
        }
        Ok(writer.register_map(
            &K::strict_encode_dumb(),
            &V::strict_encode_dumb(),
            Sizing::new(MIN_LEN as u16, MAX_LEN as u16),
        ))
    }
}
