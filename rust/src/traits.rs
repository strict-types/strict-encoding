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

use std::io::{BufRead, Seek};
use std::{fs, io};

use amplify::confinement::{Collection, Confined};
use amplify::num::u24;

use super::DecodeError;
use crate::{
    DeserializeError, FieldName, LibName, SerializeError, Sizing, StrictReader, StrictType,
    StrictWriter, TypeName,
};

pub trait TypedParent: Sized {}

#[allow(unused_variables)]
pub trait TypedWrite: Sized {
    type TupleWriter: WriteTuple<Parent = Self>;
    type StructWriter: WriteStruct<Parent = Self>;
    type UnionDefiner: DefineUnion<Parent = Self>;
    type EnumDefiner: DefineEnum<Parent = Self>;

    fn write_union(
        self,
        lib: LibName,
        name: Option<TypeName>,
        inner: impl FnOnce(Self::UnionDefiner) -> io::Result<Self>,
    ) -> io::Result<Self>;
    fn write_enum(
        self,
        lib: LibName,
        name: Option<TypeName>,
        inner: impl FnOnce(Self::EnumDefiner) -> io::Result<Self>,
    ) -> io::Result<Self>;
    fn write_tuple(
        self,
        lib: LibName,
        name: Option<TypeName>,
        inner: impl FnOnce(Self::TupleWriter) -> io::Result<Self>,
    ) -> io::Result<Self>;
    fn write_struct(
        self,
        lib: LibName,
        name: Option<TypeName>,
        inner: impl FnOnce(Self::StructWriter) -> io::Result<Self>,
    ) -> io::Result<Self>;
    fn write_type(
        self,
        lib: LibName,
        name: Option<TypeName>,
        value: &impl StrictEncode,
    ) -> io::Result<Self> {
        self.write_tuple(lib, name, |writer| Ok(writer.write_field(value)?.complete()))
    }

    #[doc(hidden)]
    unsafe fn register_primitive(self, prim: u8) -> Self { self }
    #[doc(hidden)]
    unsafe fn register_array(self, ty: &impl StrictEncode, len: u16) -> Self { self }
    #[doc(hidden)]
    unsafe fn register_unicode(self, sizing: Sizing) -> Self { self }
    #[doc(hidden)]
    unsafe fn register_ascii(self, sizing: Sizing) -> Self { self }
    #[doc(hidden)]
    unsafe fn register_list(self, ty: &impl StrictEncode, sizing: Sizing) -> Self { self }
    #[doc(hidden)]
    unsafe fn register_set(self, ty: &impl StrictEncode, sizing: Sizing) -> Self { self }
    #[doc(hidden)]
    unsafe fn register_map(
        self,
        ket: &impl StrictEncode,
        ty: &impl StrictEncode,
        sizing: Sizing,
    ) -> Self {
        self
    }

    #[doc(hidden)]
    unsafe fn _write_raw<const MAX_LEN: usize>(self, bytes: impl AsRef<[u8]>) -> io::Result<Self>;
    #[doc(hidden)]
    unsafe fn write_raw_array<const LEN: usize>(self, raw: [u8; LEN]) -> io::Result<Self> {
        self._write_raw::<LEN>(raw)
    }
    #[doc(hidden)]
    unsafe fn write_raw_bytes<const MAX_LEN: usize>(
        self,
        bytes: impl AsRef<[u8]>,
    ) -> io::Result<Self> {
        self.write_raw_len::<MAX_LEN>(bytes.as_ref().len())?._write_raw::<MAX_LEN>(bytes)
    }
    #[doc(hidden)]
    unsafe fn write_raw_len<const MAX_LEN: usize>(self, len: usize) -> io::Result<Self> {
        match MAX_LEN {
            tiny if tiny <= u8::MAX as usize => u8::strict_encode(&(len as u8), self),
            small if small < u16::MAX as usize => u16::strict_encode(&(len as u16), self),
            medium if medium < u24::MAX.into_usize() => {
                u24::strict_encode(&u24::with(len as u32), self)
            }
            large if large < u32::MAX as usize => u32::strict_encode(&(len as u32), self),
            _ => unreachable!("confined collections larger than u32::MAX must not exist"),
        }
    }
    #[doc(hidden)]
    unsafe fn write_raw_collection<C: Collection, const MIN_LEN: usize, const MAX_LEN: usize>(
        mut self,
        col: &Confined<C, MIN_LEN, MAX_LEN>,
    ) -> io::Result<Self>
    where
        for<'a> &'a C: IntoIterator,
        for<'a> <&'a C as IntoIterator>::Item: StrictEncode,
    {
        self = self.write_raw_len::<MAX_LEN>(col.len())?;
        for item in col {
            self = item.strict_encode(self)?;
        }
        Ok(self)
    }
}

pub trait TypedRead: Sized {
    type TupleReader: ReadTuple<Parent = Self>;
    type StructReader: ReadStruct<Parent = Self>;
    type UnionReader: ReadUnion<Parent = Self>;

    fn read_union<T: StrictDecode>(
        &mut self,
        lib: LibName,
        name: Option<TypeName>,
        inner: impl FnOnce(FieldName, &mut Self::UnionReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError>;
    fn read_enum<T: StrictDecode>(
        &mut self,
        lib: LibName,
        name: Option<TypeName>,
        inner: impl FnOnce(FieldName) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError>;

    fn read_tuple<T: StrictDecode>(
        &mut self,
        lib: LibName,
        name: Option<TypeName>,
        inner: impl FnOnce(FieldName, &mut Self::TupleReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError>;
    fn read_struct<T: StrictDecode>(
        &mut self,
        lib: LibName,
        name: Option<TypeName>,
        inner: impl FnOnce(FieldName, &mut Self::StructReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError>;

    fn read_type<T: StrictDecode>(
        &mut self,
        lib: LibName,
        name: Option<TypeName>,
        value: &impl StrictEncode,
    ) -> Result<T, DecodeError> {
        self.read_tuple(lib, name, |writer| Ok(writer.write_field(value)?.complete()))
    }
}

pub trait DefineTuple: Sized {
    type Parent: TypedParent;
    fn define_field<T: StrictEncode>(self) -> Self;
    fn define_field_ord<T: StrictEncode>(self, ord: u8) -> Self;
    fn complete(self) -> Self::Parent;
}

pub trait WriteTuple: Sized {
    type Parent: TypedParent;
    fn write_field(self, value: &impl StrictEncode) -> io::Result<Self>;
    fn write_field_ord(self, ord: u8, value: &impl StrictEncode) -> io::Result<Self>;
    fn complete(self) -> Self::Parent;
}

pub trait ReadTuple {
    type Parent: TypedRead;
}

pub trait DefineStruct: Sized {
    type Parent: TypedParent;
    fn define_field<T: StrictEncode>(self, name: FieldName) -> Self;
    fn define_field_ord<T: StrictEncode>(self, name: FieldName, ord: u8) -> Self;
    fn complete(self) -> Self::Parent;
}

pub trait WriteStruct: Sized {
    type Parent: TypedParent;
    fn write_field(self, name: FieldName, value: &impl StrictEncode) -> io::Result<Self>;
    fn write_field_ord(
        self,
        name: FieldName,
        ord: u8,
        value: &impl StrictEncode,
    ) -> io::Result<Self>;
    fn complete(self) -> Self::Parent;
}

pub trait ReadStruct {
    type Parent: TypedRead;
}

pub trait DefineEnum: Sized {
    type Parent: TypedWrite;
    type EnumWriter: WriteEnum<Parent = Self::Parent>;
    fn define_variant(self, name: FieldName, value: u8) -> Self;
    fn complete(self) -> Self::EnumWriter;
}

pub trait WriteEnum: Sized {
    type Parent: TypedWrite;
    fn write_variant(self, name: FieldName) -> io::Result<Self>;
    fn complete(self) -> Self::Parent;
}

pub trait DefineUnion: Sized {
    type Parent: TypedWrite;
    type TupleDefiner: DefineTuple<Parent = Self>;
    type StructDefiner: DefineStruct<Parent = Self>;
    type UnionWriter: WriteUnion<Parent = Self::Parent>;

    fn define_unit(self, name: FieldName) -> Self;
    fn define_type<T: StrictEncode>(self, name: FieldName) -> Self {
        self.define_tuple(name).define_field::<T>().complete()
    }
    fn define_tuple(self, name: FieldName) -> Self::TupleDefiner;
    fn define_struct(self, name: FieldName) -> Self::StructDefiner;

    fn complete(self) -> Self::UnionWriter;
}

pub trait WriteUnion: Sized {
    type Parent: TypedWrite;
    type TupleWriter: WriteTuple<Parent = Self>;
    type StructWriter: WriteStruct<Parent = Self>;

    fn write_unit(self, name: FieldName) -> io::Result<Self>;
    fn write_type(self, name: FieldName, value: &impl StrictEncode) -> io::Result<Self> {
        Ok(self.write_tuple(name)?.write_field(value)?.complete())
    }
    fn write_tuple(self, name: FieldName) -> io::Result<Self::TupleWriter>;
    fn write_struct(self, name: FieldName) -> io::Result<Self::StructWriter>;

    fn complete(self) -> Self::Parent;
}

pub trait ReadUnion: Sized {
    type Parent: TypedRead;
    type TupleReader: ReadTuple<Parent = Self>;
    type StructReader: ReadStruct<Parent = Self>;

    fn read_unit<T: StrictDecode>(&mut self, name: FieldName) -> Result<T, DecodeError>;
    fn read_type(self, name: FieldName, value: &impl StrictEncode) -> io::Result<Self> {
        Ok(self.write_tuple(name)?.write_field(value)?.complete())
    }
    fn read_tuple(self, name: FieldName) -> io::Result<Self::TupleReader>;
    fn read_struct(self, name: FieldName) -> io::Result<Self::StructReader>;

    fn complete(self) -> Self::Parent;
}

pub trait StrictEncode: StrictType {
    unsafe fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W>;
}

pub trait StrictDecode: Sized {
    fn strict_decode(reader: &impl TypedRead) -> Result<Self, DecodeError>;
}

impl<T: StrictEncode> StrictEncode for &T {
    unsafe fn strict_encode<W: TypedWrite>(&self, writer: W) -> io::Result<W> {
        unsafe { (*self).strict_encode(writer) }
    }
}

pub trait Serialize: StrictEncode {
    fn strict_serialized_len(&self) -> io::Result<usize> {
        let counter = StrictWriter::counter();
        Ok(unsafe { self.strict_encode(counter)? }.unbox().count)
    }

    fn to_strict_serialized<const MAX: usize>(
        &self,
    ) -> Result<Confined<Vec<u8>, 0, MAX>, SerializeError> {
        let ast_data = StrictWriter::in_memory(MAX);
        let data = unsafe { self.strict_encode(ast_data)? }.unbox();
        Confined::<Vec<u8>, 0, MAX>::try_from(data).map_err(SerializeError::from)
    }

    fn strict_serialize_to_file<const MAX: usize>(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), SerializeError> {
        let file = StrictWriter::with(MAX, fs::File::create(path)?);
        unsafe {
            self.strict_encode(file)?;
        }
        Ok(())
    }
}

pub trait Deserialize: StrictDecode {
    fn from_strict_serialized<const MAX: usize>(
        ast_data: Confined<Vec<u8>, 0, MAX>,
    ) -> Result<Self, DeserializeError> {
        let cursor = io::Cursor::new(ast_data.into_inner());
        let mut reader = StrictReader::with(MAX, cursor);
        let me = Self::strict_decode(&mut reader)?;
        let mut cursor = reader.unbox();
        if !cursor.fill_buf()?.is_empty() {
            return Err(DeserializeError::DataNotEntirelyConsumed);
        }
        Ok(me)
    }

    fn strict_deserialize_from_file<const MAX: usize>(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Self, DeserializeError> {
        let file = fs::File::open(path)?;
        let mut reader = StrictReader::with(MAX, file);
        let me = Self::strict_decode(&mut reader)?;
        let mut file = reader.unbox();
        if file.stream_position()? != file.seek(io::SeekFrom::End(0))? {
            return Err(DeserializeError::DataNotEntirelyConsumed);
        }
        Ok(me)
    }
}
