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

use strict_encoding::{tn, StrictType};

const TEST_LIB: &str = "TestLib";

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
fn rename_tuple() -> common::Result {
    #[derive(Clone, PartialEq, Eq, Debug, Default)]
    #[derive(StrictType)]
    #[strict_type(lib = TEST_LIB, rename = "ShortLen")]
    struct OtherName(u16);

    assert_eq!(OtherName::STRICT_LIB_NAME, TEST_LIB);
    assert_eq!(OtherName::strict_name().unwrap(), tn!("ShortLen"));

    Ok(())
}
