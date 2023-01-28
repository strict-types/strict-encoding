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

mod common;

use common::Result;
use strict_encoding::{StrictDumb, StrictType};

const TEST_LIB: &str = "TestLib";

#[test]
fn wrapper_base() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB)]
    struct ShortLen(u16);

    Ok(())
}

#[test]
fn tuple_base() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB)]
    struct TaggedInfo(u16, u64);

    Ok(())
}

#[test]
fn struct_base() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB)]
    struct Field<V: StrictType + StrictDumb> {
        name: u8,
        value: V,
    }

    Ok(())
}

#[test]
fn struct_base_default() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug, Default)]
    #[derive(StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB)]
    struct Field<V: StrictType>
    where V: Default
    {
        name: u8,
        value: V,
    }

    Ok(())
}

#[test]
fn enum_base() -> Result {
    #[allow(dead_code)]
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB)]
    enum Variants {
        One,
        Two,
        Three,
    }

    Ok(())
}

#[test]
fn dumb_wrapper_container() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb)]
    #[strict_type(lib = TEST_LIB, dumb = ShortLen(u16::MAX))]
    struct ShortLen(u16);

    assert_eq!(ShortLen::strict_dumb(), ShortLen(u16::MAX));
    Ok(())
}

#[test]
fn dumb_wrapper_field() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb)]
    #[strict_type(lib = TEST_LIB)]
    struct ShortLen(#[strict_type(dumb = 1)] u16);

    assert_eq!(ShortLen::strict_dumb(), ShortLen(1));
    Ok(())
}

#[test]
fn dumb_wrapper_precedence() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb)]
    #[strict_type(lib = TEST_LIB, dumb = ShortLen(u16::MAX))]
    struct ShortLen(#[strict_type(dumb = 1)] u16);

    assert_eq!(ShortLen::strict_dumb(), ShortLen(u16::MAX));
    Ok(())
}

#[cfg(feature = "unstable")]
#[test]
fn rename() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB, rename = "ShortLen")]
    struct OtherName(u16);

    assert_eq!(OtherName::NAME, "ShortLen");

    Ok(())
}
