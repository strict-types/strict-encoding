// Derivation macro library for strict encoding.
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

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding_derive;
#[macro_use]
extern crate strict_encoding_test;

mod common;

use common::{compile_test, Error, Result};
use strict_encoding::{StrictDecode, StrictEncode};
use strict_encoding_test::test_encoding_roundtrip;

#[test]
#[should_panic]
fn no_strict_units() { compile_test("basics-failures/no_strict_units"); }

#[test]
#[should_panic]
fn no_unit_types() { compile_test("basics-failures/no_unit_types"); }

#[test]
#[should_panic]
fn no_empty_types() { compile_test("basics-failures/no_empty_types"); }

#[test]
fn unit_struct() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TestLib)]
    struct Strict(u16);
    test_encoding_roundtrip(&Strict(0xcafe), [0xFE, 0xCA])?;

    Ok(())
}

#[test]
fn bytes() -> Result {
    let data = [
        0x10, 0x00, 0xCA, 0xFE, 0xDE, 0xAD, 0xBE, 0xD8, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE,
        0xFF, 0x00, 0x01,
    ];

    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    struct Vect {
        data: Vec<u8>,
    }
    test_encoding_roundtrip(
        &Vect {
            data: data[2..].to_vec(),
        },
        &data,
    )?;

    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictEncode)]
    struct Slice<'a> {
        slice: &'a [u8],
    }
    assert_eq!(&Slice { slice: &data[2..] }.strict_serialize()?, &data);

    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    struct Array {
        bytes: [u8; 16],
    }
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&data[2..]);
    test_encoding_roundtrip(&Array { bytes }, &data[2..])?;

    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    struct Heap(Box<[u8]>);
    test_encoding_roundtrip(&Heap(Box::from(&data[2..])), &data).map_err(Error::from)
}

#[test]
fn skipping() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug, Default)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    struct Skipping {
        pub data: String,

        // This will initialize the field upon decoding with Option::default()
        // value
        #[strict_encoding(skip)]
        pub ephemeral: bool,
    }

    test_encoding_roundtrip(
        &Skipping {
            data: s!("String"),
            ephemeral: false,
        },
        &[0x06, 0x00, b'S', b't', b'r', b'i', b'n', b'g'],
    )
    .map_err(Error::from)
}

#[test]
fn custom_crate() {
    use strict_encoding as custom_crate;

    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(crate = custom_crate)]
    struct One {
        a: Vec<u8>,
    }
}

#[test]
fn generics() {
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    enum CustomErr<Err>
    where Err: std::error::Error + StrictEncode + StrictDecode
    {
        Other(Err),
    }
}
