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

impl<'read, R: io::Read + 'read> TypedRead<'read> for StrictReader<R>
where &'read R: io::Read
{
    type TupleReader = TupleReader<&'read R>;
    type StructReader = StructReader<&'read R>;
    type UnionReader = Self;

    fn read_union<T: StrictUnion>(
        &'read mut self,
        inner: impl FnOnce(FieldName, &'read mut Self::UnionReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError> {
        todo!()
    }

    fn read_enum<T: StrictEnum>(
        &'read mut self,
        inner: impl FnOnce(FieldName) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError>
    where
        u8: From<T>,
    {
        todo!()
    }

    fn read_tuple<T: StrictTuple>(
        &'read mut self,
        inner: impl FnOnce(&'read mut Self::TupleReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError> {
        todo!()
    }

    fn read_struct<T: StrictStruct>(
        &'read mut self,
        inner: impl FnOnce(&'read mut Self::StructReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError> {
        todo!()
    }
}

pub struct TupleReader<R: io::Read> {
    read_fields: u8,
    reader: R,
}

impl<R: io::Read> ReadTuple for TupleReader<R> {
    fn read_field<T: StrictDecode>(&mut self) -> Result<T, DecodeError> {
        self.read_fields += 1;
        T::strict_decode(&self.reader)
    }
}

pub struct StructReader<R: io::Read> {
    named_fields: Vec<FieldName>,
    reader: R,
}

impl<R: io::Read> ReadStruct for StructReader<R> {
    fn read_field<T: StrictDecode>(&mut self, field: FieldName) -> Result<T, DecodeError> {
        self.named_fields.push(field);
        T::strict_decode(&self.reader)
    }
}

impl<'read, R: io::Read + 'read> ReadUnion<'read> for StrictReader<R>
where &'read R: io::Read
{
    type TupleReader = TupleReader<&'read R>;
    type StructReader = StructReader<&'read R>;

    fn read_unit<T: StrictSum>(&'read mut self, name: FieldName) -> Result<T, DecodeError> {
        todo!()
    }

    fn read_tuple<T: StrictSum>(
        &'read mut self,
        name: FieldName,
        inner: impl FnOnce(&'read mut Self::TupleReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError> {
        todo!()
    }

    fn read_struct<T: StrictSum>(
        &'read mut self,
        name: FieldName,
        inner: impl FnOnce(&'read mut Self::StructReader) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError> {
        todo!()
    }
}
