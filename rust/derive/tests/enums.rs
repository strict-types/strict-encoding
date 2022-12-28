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
extern crate strict_encoding_test;

mod common;

use common::Result;
use strict_encoding_test::test_encoding_roundtrip;

#[test]
fn enum_associated_types() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictEncode, StrictDecode)]
    struct Heap(Box<[u8]>);

    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictEncode, StrictDecode)]
    enum Hi {
        /// Docstring
        First(u8),
        Second(Heap),
        Third,
        Fourth {
            heap: Heap,
        },
        #[strict_encoding(value = 7)]
        Seventh,
    }

    let heap = Heap(Box::from([0xA1, 0xA2]));
    test_encoding_roundtrip(&Hi::First(0xC8), [0x00, 0xC8])?;
    test_encoding_roundtrip(&Hi::Second(heap.clone()), [
        0x01, 0x02, 0x00, 0xA1, 0xA2,
    ])?;
    test_encoding_roundtrip(&Hi::Third, [0x02])?;
    test_encoding_roundtrip(&Hi::Fourth { heap }, [
        0x03, 0x02, 0x00, 0xA1, 0xA2,
    ])?;
    test_encoding_roundtrip(&Hi::Seventh, [0x07])?;

    Ok(())
}

#[test]
fn enum_default_values() -> Result {
    #[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
    #[derive(StrictEncode, StrictDecode)]
    #[repr(u16)]
    #[display(Debug)]
    enum ContractType {
        Bit8 = 1,
        Bit16 = 2,
        Bit32 = 4,
        Bit64 = 8,
    }

    test_encoding_roundtrip(&ContractType::Bit8, [0x00])?;
    test_encoding_roundtrip(&ContractType::Bit16, [0x01])?;
    test_encoding_roundtrip(&ContractType::Bit32, [0x02])?;
    test_encoding_roundtrip(&ContractType::Bit64, [0x03])?;

    Ok(())
}

#[test]
fn enum_repr() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictEncode, StrictDecode)]
    #[strict_encoding(by_order, repr = u16)]
    #[repr(u16)]
    enum U16 {
        Bit8 = 1,
        Bit16 = 2,
        Bit32 = 4,
        Bit64 = 8,
    }

    test_encoding_roundtrip(&U16::Bit8, [0x00, 0x00])?;
    test_encoding_roundtrip(&U16::Bit16, [0x01, 0x00])?;
    test_encoding_roundtrip(&U16::Bit32, [0x02, 0x00])?;
    test_encoding_roundtrip(&U16::Bit64, [0x03, 0x00])?;

    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictEncode, StrictDecode)]
    #[strict_encoding(by_order, repr = u8)]
    #[repr(u16)]
    enum ByOrder {
        Bit8 = 1,
        Bit16 = 2,
        Bit32 = 4,
        Bit64 = 8,
    }

    test_encoding_roundtrip(&ByOrder::Bit8, [0x00])?;
    test_encoding_roundtrip(&ByOrder::Bit16, [0x01])?;
    test_encoding_roundtrip(&ByOrder::Bit32, [0x02])?;
    test_encoding_roundtrip(&ByOrder::Bit64, [0x03])?;

    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictEncode, StrictDecode)]
    #[strict_encoding(by_value)]
    #[repr(u8)]
    enum ByValue {
        Bit8 = 1,
        Bit16 = 2,
        Bit32 = 4,
        Bit64 = 8,
    }

    test_encoding_roundtrip(&ByValue::Bit8, [0x01])?;
    test_encoding_roundtrip(&ByValue::Bit16, [0x02])?;
    test_encoding_roundtrip(&ByValue::Bit32, [0x04])?;
    test_encoding_roundtrip(&ByValue::Bit64, [0x08])?;

    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictEncode, StrictDecode)]
    #[strict_encoding(by_value)]
    #[repr(u16)]
    enum ByValue16 {
        Bit8 = 1,
        Bit16 = 2,
        Bit32 = 4,
        Bit64 = 8,
    }

    test_encoding_roundtrip(&ByValue16::Bit8, [0x01])?;
    test_encoding_roundtrip(&ByValue16::Bit16, [0x02])?;
    test_encoding_roundtrip(&ByValue16::Bit32, [0x04])?;
    test_encoding_roundtrip(&ByValue16::Bit64, [0x08])?;

    Ok(())
}

#[test]
fn enum_custom_values() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictEncode, StrictDecode)]
    #[strict_encoding(by_value)]
    #[repr(u8)]
    enum CustomValues {
        Bit8 = 1,

        #[strict_encoding(value = 11)]
        Bit16 = 2,

        #[strict_encoding(value = 12)]
        Bit32 = 4,

        #[strict_encoding(value = 13)]
        Bit64 = 8,
    }

    test_encoding_roundtrip(&CustomValues::Bit8, [1])?;
    test_encoding_roundtrip(&CustomValues::Bit16, [11])?;
    test_encoding_roundtrip(&CustomValues::Bit32, [12])?;
    test_encoding_roundtrip(&CustomValues::Bit64, [13])?;

    Ok(())
}
