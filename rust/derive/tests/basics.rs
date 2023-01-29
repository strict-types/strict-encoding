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

use std::convert::Infallible;

use strict_encoding::{tn, StrictDecode, StrictDumb, StrictEncode, StrictType, VariantError};

const TEST_LIB: &str = "TestLib";

#[test]
fn wrapper_base() -> common::Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB)]
    struct ShortLen(u16);

    Ok(())
}

#[test]
fn tuple_base() -> common::Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB)]
    struct TaggedInfo(u16, u64);

    Ok(())
}

#[test]
fn tuple_generics() -> common::Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB)]
    struct Pair<
        A: StrictDumb + StrictEncode + StrictDecode,
        B: StrictDumb + StrictEncode + StrictDecode,
    >(A, B);

    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB)]
    struct WhereConstraint<A: TryInto<u8>, B: From<String>>(A, B)
    where
        A: StrictDumb + StrictEncode + StrictDecode + From<u8>,
        <A as TryFrom<u8>>::Error: From<Infallible>,
        B: StrictDumb + StrictEncode + StrictDecode;

    Ok(())
}

#[test]
fn struct_generics() -> common::Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB)]
    struct Field<V: StrictEncode + StrictDecode + StrictDumb> {
        tag: u8,
        value: V,
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode)]
    #[strict_type(lib = TEST_LIB)]
    struct ComplexField<'a, V: StrictEncode + StrictDumb>
    where
        for<'b> V: From<&'b str>,
        &'a V: Default,
    {
        tag: u8,
        value: &'a V,
    }

    Ok(())
}

#[test]
fn enum_ord() -> common::Result {
    // TODO: `tags = order` must use into_u8 and try_from_u8 always
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB, tags = order, into_u8, try_from_u8)]
    #[repr(u8)]
    enum Variants {
        #[strict_type(dumb)]
        One = 5,
        Two = 6,
        Three = 7,
    }

    assert_eq!(Variants::Three as u8, 7);
    assert_eq!(u8::from(Variants::Three), 2);
    assert_eq!(Variants::try_from(1), Ok(Variants::Two));
    assert_eq!(Variants::try_from(3), Err(VariantError(Some(tn!("Variants")), 3)));

    Ok(())
}

#[test]
fn enum_repr() -> common::Result {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB, tags = repr, into_u8, try_from_u8)]
    #[repr(u16)]
    enum Cls {
        One = 1,
        #[strict_type(dumb)]
        Two,
        Three,
    }

    assert_eq!(u8::from(Cls::Three), 3);
    assert_eq!(Cls::try_from(2), Ok(Cls::Two));
    assert_eq!(Cls::try_from(4), Err(VariantError(Some(tn!("Cls")), 4)));

    Ok(())
}

#[test]
fn enum_associated() -> common::Result {
    #[allow(dead_code)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB, tags = order)]
    enum Assoc {
        One {
            hash: [u8; 32],
            ord: u8,
        },
        Two(u8, u16, u32),
        #[strict_type(dumb)]
        Three,
        Four(),
        Five {},
    }

    Ok(())
}

#[test]
fn lib_name() -> common::Result {
    #[derive(Clone, PartialEq, Eq, Debug, Default)]
    #[derive(StrictType)]
    #[strict_type(lib = TEST_LIB)]
    struct OtherName(u16);

    assert_eq!(OtherName::STRICT_LIB_NAME, TEST_LIB);

    Ok(())
}

#[test]
fn rename() -> common::Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
    #[strict_type(lib = TEST_LIB, rename = "ShortLen")]
    struct OtherName(u16);

    assert_eq!(OtherName::STRICT_LIB_NAME, TEST_LIB);
    assert_eq!(OtherName::strict_name().unwrap(), tn!("ShortLen"));

    Ok(())
}
