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
use std::io;
use std::io::Sink;
use std::marker::PhantomData;

use amplify::WriteCounter;

use crate::{
    DefineEnum, DefineStruct, DefineTuple, DefineUnion, Field, FieldName, LibName, StrictEncode,
    StrictEnum, StrictProduct, StrictStruct, StrictSum, StrictTuple, StrictUnion, TypeName,
    TypedParent, TypedWrite, WriteEnum, WriteStruct, WriteTuple, WriteUnion, NO_LIB,
};

// TODO: Move to amplify crate
#[derive(Debug)]
pub struct CountingWriter<W: io::Write> {
    count: usize,
    limit: usize,
    writer: W,
}

impl<W: io::Write> From<W> for CountingWriter<W> {
    fn from(writer: W) -> Self {
        Self {
            count: 0,
            limit: usize::MAX,
            writer,
        }
    }
}

impl<W: io::Write> CountingWriter<W> {
    pub fn with(limit: usize, writer: W) -> Self {
        Self {
            count: 0,
            limit,
            writer,
        }
    }

    pub fn unbox(self) -> W { self.writer }
}

impl<W: io::Write> io::Write for CountingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.count + buf.len() > self.limit {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }
        let count = self.writer.write(buf)?;
        self.count += count;
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> { self.writer.flush() }
}

#[derive(Debug, From)]
pub struct StrictWriter<W: io::Write>(CountingWriter<W>);

impl StrictWriter<Vec<u8>> {
    pub fn in_memory(limit: usize) -> Self { StrictWriter(CountingWriter::with(limit, vec![])) }
}

impl StrictWriter<WriteCounter> {
    pub fn counter() -> Self { StrictWriter(CountingWriter::from(WriteCounter::default())) }
}

impl StrictWriter<Sink> {
    pub fn sink() -> Self { StrictWriter(CountingWriter::from(Sink::default())) }
}

impl<W: io::Write> StrictWriter<W> {
    pub fn with(limit: usize, writer: W) -> Self {
        StrictWriter(CountingWriter::with(limit, writer))
    }
    pub fn unbox(self) -> W { self.0.unbox() }
}

impl<W: io::Write> TypedWrite for StrictWriter<W> {
    type TupleWriter = StructWriter<W, Self>;
    type StructWriter = StructWriter<W, Self>;
    type UnionDefiner = UnionWriter<W>;
    type EnumDefiner = UnionWriter<W>;

    fn write_union<T: StrictUnion>(
        self,
        inner: impl FnOnce(Self::UnionDefiner) -> io::Result<Self>,
    ) -> io::Result<Self> {
        let writer = UnionWriter::with::<T>(self);
        inner(writer)
    }

    fn write_enum<T: StrictEnum>(
        self,
        inner: impl FnOnce(Self::EnumDefiner) -> io::Result<Self>,
    ) -> io::Result<Self>
    where
        u8: From<T>,
    {
        let writer = UnionWriter::with::<T>(self);
        inner(writer)
    }

    fn write_tuple<T: StrictTuple>(
        self,
        inner: impl FnOnce(Self::TupleWriter) -> io::Result<Self>,
    ) -> io::Result<Self> {
        let writer = StructWriter::with::<T>(self);
        inner(writer)
    }

    fn write_struct<T: StrictStruct>(
        self,
        inner: impl FnOnce(Self::StructWriter) -> io::Result<Self>,
    ) -> io::Result<Self> {
        let writer = StructWriter::with::<T>(self);
        inner(writer)
    }

    unsafe fn _write_raw<const LEN: usize>(mut self, bytes: impl AsRef<[u8]>) -> io::Result<Self> {
        use io::Write;
        self.0.write_all(bytes.as_ref())?;
        Ok(self)
    }
}

pub struct StructWriter<W: io::Write, P: StrictParent<W>> {
    lib: LibName,
    name: Option<TypeName>,
    fields: BTreeSet<Field>,
    ords: BTreeSet<u8>,
    parent: P,
    defined: bool,
    _phantom: PhantomData<W>,
}

impl<W: io::Write, P: StrictParent<W>> StructWriter<W, P> {
    pub fn with<T: StrictProduct>(parent: P) -> Self {
        StructWriter {
            lib: libname!(T::STRICT_LIB_NAME),
            name: T::strict_name(),
            fields: empty!(),
            ords: empty!(),
            parent,
            defined: false,
            _phantom: default!(),
        }
    }

    pub fn unnamed(parent: P) -> Self {
        StructWriter {
            lib: libname!(NO_LIB),
            name: None,
            fields: empty!(),
            ords: empty!(),
            parent,
            defined: false,
            _phantom: default!(),
        }
    }

    pub fn is_defined(&self) -> bool { self.defined }

    pub fn field_ord(&self, field_name: &FieldName) -> Option<u8> {
        self.fields.iter().find(|f| f.name.as_ref() == Some(field_name)).map(|f| f.ord)
    }

    pub fn fields(&self) -> &BTreeSet<Field> { &self.fields }

    pub fn name(&self) -> &str { self.name.as_ref().map(|n| n.as_str()).unwrap_or("<unnamed>") }

    pub fn next_ord(&self) -> u8 { self.fields.iter().max().map(|f| f.ord + 1).unwrap_or_default() }

    pub fn into_parent(self) -> P { self.parent }

    fn _define_field(mut self, field: Field) -> Self {
        assert!(
            self.fields.insert(field.clone()),
            "field {:#} is already defined as a part of {}",
            &field,
            self.name()
        );
        self.ords.insert(field.ord);
        self
    }

    fn _write_field(mut self, field: Field, value: &impl StrictEncode) -> io::Result<Self> {
        if self.defined {
            assert!(
                !self.fields.contains(&field),
                "field {:#} was not defined in {}",
                &field,
                self.name()
            )
        } else {
            self = self._define_field(field.clone());
        }
        assert!(
            self.ords.remove(&field.ord),
            "field {:#} was already written before in {} struct",
            &field,
            self.name()
        );
        let (mut writer, remnant) = self.parent.into_write_split();
        writer = unsafe { field.ord.strict_encode(writer)? };
        writer = unsafe { value.strict_encode(writer)? };
        self.parent = P::from_write_split(writer, remnant);
        Ok(self)
    }

    fn _complete_definition(mut self) -> P {
        assert!(!self.fields.is_empty(), "struct {} does not have fields defined", self.name());
        self.defined = true;
        self.parent
    }

    fn _complete_write(self) -> P {
        assert!(self.ords.is_empty(), "not all fields were written for {}", self.name());
        self.parent
    }
}

impl<W: io::Write, P: StrictParent<W>> DefineStruct for StructWriter<W, P> {
    type Parent = P;
    fn define_field<T: StrictEncode>(self, name: FieldName) -> Self {
        let ord = self.next_ord();
        DefineStruct::define_field_ord::<T>(self, name, ord)
    }
    fn define_field_ord<T: StrictEncode>(self, name: FieldName, ord: u8) -> Self {
        let field = Field::named(name, ord);
        self._define_field(field)
    }
    fn complete(self) -> P { self._complete_definition() }
}

impl<W: io::Write, P: StrictParent<W>> WriteStruct for StructWriter<W, P> {
    type Parent = P;
    fn write_field(self, name: FieldName, value: &impl StrictEncode) -> io::Result<Self> {
        let ord = self.next_ord();
        WriteStruct::write_field_ord(self, name, ord, value)
    }
    fn write_field_ord(
        self,
        name: FieldName,
        ord: u8,
        value: &impl StrictEncode,
    ) -> io::Result<Self> {
        let field = Field::named(name, ord);
        self._write_field(field, value)
    }
    fn complete(self) -> P { self._complete_write() }
}

impl<W: io::Write, P: StrictParent<W>> DefineTuple for StructWriter<W, P> {
    type Parent = P;
    fn define_field<T: StrictEncode>(self) -> Self {
        let ord = self.next_ord();
        DefineTuple::define_field_ord::<T>(self, ord)
    }
    fn define_field_ord<T: StrictEncode>(self, ord: u8) -> Self {
        let field = Field::unnamed(ord);
        self._define_field(field)
    }
    fn complete(self) -> P { self._complete_definition() }
}

impl<W: io::Write, P: StrictParent<W>> WriteTuple for StructWriter<W, P> {
    type Parent = P;
    fn write_field(self, value: &impl StrictEncode) -> io::Result<Self> {
        let ord = self.next_ord();
        WriteTuple::write_field_ord(self, ord, value)
    }
    fn write_field_ord(self, ord: u8, value: &impl StrictEncode) -> io::Result<Self> {
        let field = Field::unnamed(ord);
        self._write_field(field, value)
    }
    fn complete(self) -> P { self._complete_write() }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum FieldType {
    Unit,
    Tuple,
    Struct,
}

pub struct UnionWriter<W: io::Write> {
    lib: LibName,
    name: Option<TypeName>,
    variants: BTreeMap<Field, FieldType>,
    parent: StrictWriter<W>,
    written: bool,
    parent_ident: Option<TypeName>,
}

impl<W: io::Write> UnionWriter<W> {
    pub fn with<T: StrictSum>(parent: StrictWriter<W>) -> Self {
        UnionWriter {
            lib: libname!(T::STRICT_LIB_NAME),
            name: T::strict_name(),
            variants: empty!(),
            parent,
            written: false,
            parent_ident: None,
        }
    }

    pub fn inline<T: StrictSum>(uw: UnionWriter<W>) -> Self {
        UnionWriter {
            lib: libname!(T::STRICT_LIB_NAME),
            name: T::strict_name(),
            variants: empty!(),
            parent: uw.parent,
            written: false,
            parent_ident: uw.name,
        }
    }

    pub fn is_written(&self) -> bool { self.written }

    pub fn variants(&self) -> &BTreeMap<Field, FieldType> { &self.variants }

    pub fn name(&self) -> &str { self.name.as_ref().map(|n| n.as_str()).unwrap_or("<unnamed>") }

    pub fn ord_by_name(&self, name: &FieldName) -> Option<u8> {
        self.variants.keys().find(|f| f.name.as_ref() == Some(name)).map(|f| f.ord)
    }

    pub fn next_ord(&self) -> u8 {
        self.variants.keys().max().map(|f| f.ord + 1).unwrap_or_default()
    }

    fn _define_field(mut self, field: Field, field_type: FieldType) -> Self {
        assert!(
            self.variants.insert(field.clone(), field_type).is_none(),
            "variant {:#} is already defined as a part of {}",
            &field,
            self.name()
        );
        self
    }

    fn _write_field(mut self, name: FieldName, field_type: FieldType) -> io::Result<Self> {
        let (field, t) = self
            .variants
            .iter()
            .find(|(f, _)| f.name.as_ref() == Some(&name))
            .expect(&format!("variant {:#} was not defined in {}", &name, self.name()));
        assert_eq!(
            *t,
            field_type,
            "variant {:#} in {} must be a {:?} while it is written as {:?}",
            &field,
            self.name(),
            t,
            field_type
        );
        assert!(!self.written, "multiple attempts to write variants of {}", self.name());
        self.written = true;
        self.parent = unsafe { field.ord.strict_encode(self.parent)? };
        Ok(self)
    }

    fn _complete_definition(self) -> Self {
        assert!(
            !self.variants.is_empty(),
            "unit or enum {} does not have fields defined",
            self.name()
        );
        self
    }

    fn _complete_write(self) -> StrictWriter<W> {
        assert!(self.written, "not a single variant is written for {}", self.name());
        self.parent
    }
}

impl<W: io::Write> DefineUnion for UnionWriter<W> {
    type Parent = StrictWriter<W>;
    type TupleDefiner = StructWriter<W, Self>;
    type StructDefiner = StructWriter<W, Self>;
    type UnionWriter = UnionWriter<W>;

    fn define_unit(self, name: FieldName) -> Self {
        let field = Field::named(name, self.next_ord());
        self._define_field(field, FieldType::Unit)
    }
    fn define_tuple(mut self, name: FieldName) -> Self::TupleDefiner {
        let field = Field::named(name, self.next_ord());
        self = self._define_field(field, FieldType::Tuple);
        StructWriter::unnamed(self)
    }
    fn define_struct(mut self, name: FieldName) -> Self::StructDefiner {
        let field = Field::named(name, self.next_ord());
        self = self._define_field(field, FieldType::Struct);
        StructWriter::unnamed(self)
    }
    fn complete(self) -> Self::UnionWriter { self._complete_definition() }
}

impl<W: io::Write> WriteUnion for UnionWriter<W> {
    type Parent = StrictWriter<W>;
    type TupleWriter = StructWriter<W, Self>;
    type StructWriter = StructWriter<W, Self>;

    fn write_unit(self, name: FieldName) -> io::Result<Self> {
        self._write_field(name, FieldType::Unit)
    }
    fn write_tuple(mut self, name: FieldName) -> io::Result<Self::TupleWriter> {
        self = self._write_field(name, FieldType::Tuple)?;
        Ok(StructWriter::unnamed(self))
    }
    fn write_struct(mut self, name: FieldName) -> io::Result<Self::StructWriter> {
        self = self._write_field(name, FieldType::Struct)?;
        Ok(StructWriter::unnamed(self))
    }
    fn complete(self) -> Self::Parent { self._complete_write() }
}

impl<W: io::Write> DefineEnum for UnionWriter<W> {
    type Parent = StrictWriter<W>;
    type EnumWriter = UnionWriter<W>;
    fn define_variant(self, name: FieldName, value: u8) -> Self {
        let field = Field::named(name, value);
        self._define_field(field, FieldType::Unit)
    }
    fn complete(self) -> Self::EnumWriter { self._complete_definition() }
}

impl<W: io::Write> WriteEnum for UnionWriter<W> {
    type Parent = StrictWriter<W>;
    fn write_variant(self, name: FieldName) -> io::Result<Self> {
        self._write_field(name, FieldType::Unit)
    }
    fn complete(self) -> Self::Parent { self._complete_write() }
}

pub trait StrictParent<W: io::Write>: TypedParent {
    type Remnant;
    fn from_write_split(writer: StrictWriter<W>, remnant: Self::Remnant) -> Self;
    fn into_write_split(self) -> (StrictWriter<W>, Self::Remnant);
}
impl<W: io::Write> TypedParent for StrictWriter<W> {}
impl<W: io::Write> TypedParent for UnionWriter<W> {}
impl<W: io::Write> StrictParent<W> for StrictWriter<W> {
    type Remnant = ();
    fn from_write_split(writer: StrictWriter<W>, _: Self::Remnant) -> Self { writer }
    fn into_write_split(self) -> (StrictWriter<W>, Self::Remnant) { (self, ()) }
}
impl<W: io::Write> StrictParent<W> for UnionWriter<W> {
    type Remnant = UnionWriter<Vec<u8>>;
    fn from_write_split(writer: StrictWriter<W>, remnant: Self::Remnant) -> Self {
        Self {
            lib: remnant.lib,
            name: remnant.name,
            variants: remnant.variants,
            parent: writer,
            written: remnant.written,
            parent_ident: remnant.parent_ident,
        }
    }
    fn into_write_split(self) -> (StrictWriter<W>, Self::Remnant) {
        let remnant = UnionWriter {
            lib: self.lib,
            name: self.name,
            variants: self.variants,
            parent: StrictWriter::in_memory(0),
            written: self.written,
            parent_ident: self.parent_ident,
        };
        (self.parent, remnant)
    }
}

pub trait SplitParent {
    type Parent: TypedParent;
    type Remnant;
    fn from_parent_split(parent: Self::Parent, remnant: Self::Remnant) -> Self;
    fn into_parent_split(self) -> (Self::Parent, Self::Remnant);
}
impl<W: io::Write, P: StrictParent<W>> SplitParent for StructWriter<W, P> {
    type Parent = P;
    type Remnant = StructWriter<Vec<u8>, ParentDumb>;
    fn from_parent_split(parent: P, remnant: Self::Remnant) -> Self {
        Self {
            lib: remnant.lib,
            name: remnant.name,
            fields: remnant.fields,
            parent,
            defined: remnant.defined,
            ords: remnant.ords,
            _phantom: none!(),
        }
    }
    fn into_parent_split(self) -> (P, Self::Remnant) {
        let remnant = StructWriter::<Vec<u8>, ParentDumb> {
            lib: self.lib,
            name: self.name,
            fields: self.fields,
            parent: none!(),
            defined: self.defined,
            ords: self.ords,
            _phantom: none!(),
        };
        (self.parent, remnant)
    }
}

#[derive(Default)]
pub struct ParentDumb;
impl TypedParent for ParentDumb {}
impl<W: io::Write> StrictParent<W> for ParentDumb {
    type Remnant = ();
    fn from_write_split(_: StrictWriter<W>, _: Self::Remnant) -> Self { unreachable!() }
    fn into_write_split(self) -> (StrictWriter<W>, Self::Remnant) { unreachable!() }
}
