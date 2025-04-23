// Strict encoding library for deterministic binary serialization.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2024 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright 2022-2024 UBIDECO Labs
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

use amplify::Bytes32;

/// File header for strict-serialized content.
///
/// The header has a fixed size of 128 bytes.
pub struct StrictFileHeader {
    // Constant "SE"
    pub magic: [u8; 4],
    pub max_len: u64,
    pub lib_name: [u8; 26],
    pub type_name: [u8; 26],
    pub sem_id: Bytes32,
    pub checksum: Bytes32,
}
