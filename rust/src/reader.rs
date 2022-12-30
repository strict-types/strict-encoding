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

/*
       reader.read_union("Test", Some("Example"), |field, r| match field {
           f!(0u8, "init") => Example::Init(r.read_type()),
           f!(2u8, "connect") => Example::Connect {
               host: r.read_struct().read_field("host").complete(),
           },
       })
*/

use std::io;
use std::io::Read;

use crate::{
    DecodeError, FieldName, ReadStruct, ReadTuple, ReadUnion, StrictDecode, StrictEnum,
    StrictStruct, StrictSum, StrictTuple, StrictUnion, TypedRead,
};

trait TypedParent: Sized {}

// TODO: Move to amplify crate
/// A simple way to count bytes read through [`io::Read`].
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Debug)]
pub struct ReadCounter {
    /// Count of bytes which passed through this reader
    pub count: usize,
}

impl io::Read for ReadCounter {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let count = buf.len();
        self.count += count;
        Ok(count)
    }
}

// TODO: Move to amplify crate
#[derive(Debug)]
pub struct CountingReader<R: io::Read> {
    count: usize,
    limit: usize,
    reader: R,
}

impl<R: io::Read> From<R> for CountingReader<R> {
    fn from(reader: R) -> Self {
        Self {
            count: 0,
            limit: usize::MAX,
            reader,
        }
    }
}

impl<R: io::Read> CountingReader<R> {
    pub fn with(limit: usize, reader: R) -> Self {
        Self {
            count: 0,
            limit,
            reader,
        }
    }

    pub fn unbox(self) -> R { self.reader }
}

impl<R: io::Read> io::Read for CountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { todo!() }
}

#[derive(Debug, From)]
pub struct StrictReader<R: io::Read>(CountingReader<R>);

impl StrictReader<io::Cursor<Vec<u8>>> {
    pub fn in_memory(limit: usize) -> Self {
        StrictReader(CountingReader::with(limit, io::Cursor::new(vec![])))
    }
}

impl StrictReader<ReadCounter> {
    pub fn counter() -> Self { StrictReader(CountingReader::from(ReadCounter::default())) }
}

impl<R: io::Read> StrictReader<R> {
    pub fn with(limit: usize, reader: R) -> Self {
        StrictReader(CountingReader::with(limit, reader))
    }

    pub fn unbox(self) -> R { self.0.unbox() }
}

impl<'read, R: 'read + io::Read> TypedRead<'read> for StrictReader<R> {
    type TupleReader = TupleReader<'read, R>;
    type StructReader = StructReader<'read, R>;
    type UnionReader = Self;

    fn read_union<'me, T: StrictUnion>(
        &'me mut self,
        inner: impl FnOnce(FieldName, &mut Self::UnionReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError>
    where
        'me: 'read,
    {
        let name = T::strict_name().unwrap_or_else(|| tn!("__unnamed"));
        // TODO: read ord
        let ord = 0u8;
        let variant_name = T::variant_name_by_ord(ord)
            .ok_or(DecodeError::UnionValueNotKnown(name.to_string(), ord))?;
        inner(variant_name, self)
    }

    fn read_enum<'me, T: StrictEnum>(
        &'me mut self,
        inner: impl FnOnce(FieldName) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError>
    where
        u8: From<T>,
        'me: 'read,
    {
        let name = T::strict_name().unwrap_or_else(|| tn!("__unnamed"));
        // TODO: read ord
        let ord = 0u8;
        let variant_name = T::variant_name_by_ord(ord)
            .ok_or(DecodeError::EnumValueNotKnown(name.to_string(), ord))?;
        inner(variant_name)
    }

    fn read_tuple<'me, T: StrictTuple>(
        &'me mut self,
        inner: impl FnOnce(&mut Self::TupleReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError>
    where
        'me: 'read,
    {
        let name = T::strict_name().unwrap_or_else(|| tn!("__unnamed"));
        let mut reader = TupleReader {
            read_fields: 0,
            parent: self,
        };
        let res = inner(&mut reader)?;
        assert_ne!(reader.read_fields, 0, "you forget to read fields for a tuple {}", name);
        assert_ne!(
            reader.read_fields,
            T::FIELD_COUNT,
            "the number of fields read for a tuple {} doesn't match type declaration",
            name
        );
        Ok(res)
    }

    fn read_struct<'me, T: StrictStruct>(
        &'me mut self,
        inner: impl FnOnce(&mut Self::StructReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError>
    where
        'me: 'read,
    {
        let name = T::strict_name().unwrap_or_else(|| tn!("__unnamed"));
        let mut reader = StructReader {
            named_fields: empty!(),
            parent: self,
        };
        let res = inner(&mut reader)?;
        assert!(!reader.named_fields.is_empty(), "you forget to read fields for a tuple {}", name);

        for field in T::ALL_FIELDS {
            let pos = reader
                .named_fields
                .iter()
                .position(|f| f.as_str() == *field)
                .unwrap_or_else(|| panic!("field {} is not read for {}", field, name));
            reader.named_fields.remove(pos);
        }
        assert!(reader.named_fields.is_empty(), "excessive fields are read for {}", name);
        Ok(res)
    }

    unsafe fn _read_raw<const MAX_LEN: usize>(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.0.read_exact(&mut buf)?;
        Ok(buf)
    }

    unsafe fn read_raw_array<const LEN: usize>(&mut self) -> io::Result<[u8; LEN]> {
        let mut buf = [0u8; LEN];
        self.0.read_exact(&mut buf)?;
        Ok(buf)
    }
}

pub struct TupleReader<'read, R: io::Read> {
    read_fields: u8,
    parent: &'read mut StrictReader<R>,
}

impl<'read, R: io::Read> ReadTuple for TupleReader<'read, R> {
    fn read_field<T: StrictDecode>(&mut self) -> Result<T, DecodeError> {
        self.read_fields += 1;
        T::strict_decode(self.parent)
    }
}

pub struct StructReader<'read, R: io::Read> {
    named_fields: Vec<FieldName>,
    parent: &'read mut StrictReader<R>,
}

impl<'read, R: io::Read> ReadStruct for StructReader<'read, R> {
    fn read_field<T: StrictDecode>(&mut self, field: FieldName) -> Result<T, DecodeError> {
        self.named_fields.push(field);
        T::strict_decode(self.parent)
    }
}

impl<'read, R: io::Read + 'read> ReadUnion<'read> for StrictReader<R> {
    type TupleReader = TupleReader<'read, R>;
    type StructReader = StructReader<'read, R>;

    fn read_tuple<'me, T: StrictSum>(
        &'me mut self,
        inner: impl FnOnce(&mut Self::TupleReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError>
    where
        'me: 'read,
    {
        let mut reader = TupleReader {
            read_fields: 0,
            parent: self,
        };
        inner(&mut reader)
    }

    fn read_struct<'me, T: StrictSum>(
        &'me mut self,
        inner: impl FnOnce(&mut Self::StructReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError>
    where
        'me: 'read,
    {
        let mut reader = StructReader {
            named_fields: empty!(),
            parent: self,
        };
        inner(&mut reader)
    }
}
