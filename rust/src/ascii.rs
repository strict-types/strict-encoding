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

use crate::{StrictDecode, StrictDumb, StrictEncode, StrictType, STD_LIB};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = STD_LIB, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum AlphaCaps {
    #[strict_type(dumb)]
    A = b'A',
    B = b'B',
    C = b'C',
    D = b'D',
    E = b'E',
    F = b'F',
    G = b'G',
    H = b'H',
    I = b'I',
    J = b'J',
    K = b'K',
    L = b'L',
    M = b'M',
    N = b'N',
    O = b'O',
    P = b'P',
    Q = b'Q',
    R = b'R',
    S = b'S',
    T = b'T',
    U = b'U',
    V = b'V',
    W = b'W',
    X = b'X',
    Y = b'Y',
    Z = b'Z',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = STD_LIB, tags = repr, into_u8, try_from_u8, crate = crate)]
#[repr(u8)]
pub enum AlphaSmall {
    #[strict_type(dumb)]
    #[display("a")]
    A = b'a',
    #[display("b")]
    B = b'b',
    #[display("c")]
    C = b'c',
    #[display("d")]
    D = b'd',
    #[display("e")]
    E = b'e',
    #[display("f")]
    F = b'f',
    #[display("g")]
    G = b'g',
    #[display("h")]
    H = b'h',
    #[display("i")]
    I = b'i',
    #[display("j")]
    J = b'j',
    #[display("k")]
    K = b'k',
    #[display("l")]
    L = b'l',
    #[display("m")]
    M = b'm',
    #[display("n")]
    N = b'n',
    #[display("o")]
    O = b'o',
    #[display("p")]
    P = b'p',
    #[display("q")]
    Q = b'q',
    #[display("r")]
    R = b'r',
    #[display("s")]
    S = b's',
    #[display("t")]
    T = b't',
    #[display("u")]
    U = b'u',
    #[display("v")]
    V = b'v',
    #[display("w")]
    W = b'w',
    #[display("x")]
    X = b'x',
    #[display("y")]
    Y = b'y',
    #[display("z")]
    Z = b'z',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = STD_LIB, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum Alpha {
    #[strict_type(dumb)]
    A = b'A',
    B = b'B',
    C = b'C',
    D = b'D',
    E = b'E',
    F = b'F',
    G = b'G',
    H = b'H',
    I = b'I',
    J = b'J',
    K = b'K',
    L = b'L',
    M = b'M',
    N = b'N',
    O = b'O',
    P = b'P',
    Q = b'Q',
    R = b'R',
    S = b'S',
    T = b'T',
    U = b'U',
    V = b'V',
    W = b'W',
    X = b'X',
    Y = b'Y',
    Z = b'Z',
    #[display("a")]
    SmallA = b'a',
    #[display("b")]
    SmallB = b'b',
    #[display("c")]
    SmallC = b'c',
    #[display("d")]
    SmallD = b'd',
    #[display("e")]
    SmallE = b'e',
    #[display("f")]
    SmallF = b'f',
    #[display("g")]
    SmallG = b'g',
    #[display("h")]
    SmallH = b'h',
    #[display("i")]
    SmallI = b'i',
    #[display("j")]
    SmallJ = b'j',
    #[display("k")]
    SmallK = b'k',
    #[display("l")]
    SmallL = b'l',
    #[display("m")]
    SmallM = b'm',
    #[display("n")]
    SmallN = b'n',
    #[display("o")]
    SmallO = b'o',
    #[display("p")]
    SmallP = b'p',
    #[display("q")]
    SmallQ = b'q',
    #[display("r")]
    SmallR = b'r',
    #[display("s")]
    SmallS = b's',
    #[display("t")]
    SmallT = b't',
    #[display("u")]
    SmallU = b'u',
    #[display("v")]
    SmallV = b'v',
    #[display("w")]
    SmallW = b'w',
    #[display("x")]
    SmallX = b'x',
    #[display("y")]
    SmallY = b'y',
    #[display("z")]
    SmallZ = b'z',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = STD_LIB, tags = repr, into_u8, try_from_u8, crate = crate)]
#[repr(u8)]
pub enum Dec {
    #[strict_type(dumb)]
    #[display("0")]
    Zero = b'0',
    #[display("1")]
    One = b'1',
    #[display("2")]
    Two = b'2',
    #[display("3")]
    Three = b'3',
    #[display("4")]
    Four = b'4',
    #[display("5")]
    Five = b'5',
    #[display("6")]
    Six = b'6',
    #[display("7")]
    Seven = b'7',
    #[display("8")]
    Eight = b'8',
    #[display("9")]
    Nine = b'9',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = STD_LIB, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum HexDecCaps {
    #[strict_type(dumb)]
    #[display("0")]
    Zero = b'0',
    #[display("1")]
    One = b'1',
    #[display("2")]
    Two = b'2',
    #[display("3")]
    Three = b'3',
    #[display("4")]
    Four = b'4',
    #[display("5")]
    Five = b'5',
    #[display("6")]
    Six = b'6',
    #[display("7")]
    Seven = b'7',
    #[display("8")]
    Eight = b'8',
    #[display("9")]
    Nine = b'9',
    #[display("A")]
    Ten = b'A',
    #[display("B")]
    Eleven = b'B',
    #[display("C")]
    Twelve = b'C',
    #[display("D")]
    Thirteen = b'D',
    #[display("E")]
    Fourteen = b'E',
    #[display("F")]
    Fifteen = b'F',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = STD_LIB, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum HexDecSmall {
    #[strict_type(dumb)]
    #[display("0")]
    Zero = b'0',
    #[display("1")]
    One = b'1',
    #[display("2")]
    Two = b'2',
    #[display("3")]
    Three = b'3',
    #[display("4")]
    Four = b'4',
    #[display("5")]
    Five = b'5',
    #[display("6")]
    Six = b'6',
    #[display("7")]
    Seven = b'7',
    #[display("8")]
    Eight = b'8',
    #[display("9")]
    Nine = b'9',
    #[display("a")]
    Ten = b'a',
    #[display("b")]
    Eleven = b'b',
    #[display("c")]
    Twelve = b'c',
    #[display("d")]
    Thirteen = b'd',
    #[display("e")]
    Fourteen = b'e',
    #[display("f")]
    Fifteen = b'f',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = STD_LIB, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum AlphaCapsNum {
    #[strict_type(dumb)]
    A = b'A',
    B = b'B',
    C = b'C',
    D = b'D',
    E = b'E',
    F = b'F',
    G = b'G',
    H = b'H',
    I = b'I',
    J = b'J',
    K = b'K',
    L = b'L',
    M = b'M',
    N = b'N',
    O = b'O',
    P = b'P',
    Q = b'Q',
    R = b'R',
    S = b'S',
    T = b'T',
    U = b'U',
    V = b'V',
    W = b'W',
    X = b'X',
    Y = b'Y',
    Z = b'Z',
    #[display("0")]
    Zero = b'0',
    #[display("1")]
    One = b'1',
    #[display("2")]
    Two = b'2',
    #[display("3")]
    Three = b'3',
    #[display("4")]
    Four = b'4',
    #[display("5")]
    Five = b'5',
    #[display("6")]
    Six = b'6',
    #[display("7")]
    Seven = b'7',
    #[display("8")]
    Eight = b'8',
    #[display("9")]
    Nine = b'9',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = STD_LIB, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum AlphaNum {
    #[strict_type(dumb)]
    A = b'A',
    B = b'B',
    C = b'C',
    D = b'D',
    E = b'E',
    F = b'F',
    G = b'G',
    H = b'H',
    I = b'I',
    J = b'J',
    K = b'K',
    L = b'L',
    M = b'M',
    N = b'N',
    O = b'O',
    P = b'P',
    Q = b'Q',
    R = b'R',
    S = b'S',
    T = b'T',
    U = b'U',
    V = b'V',
    W = b'W',
    X = b'X',
    Y = b'Y',
    Z = b'Z',
    #[display("a")]
    SmallA = b'a',
    #[display("b")]
    SmallB = b'b',
    #[display("c")]
    SmallC = b'c',
    #[display("d")]
    SmallD = b'd',
    #[display("e")]
    SmallE = b'e',
    #[display("f")]
    SmallF = b'f',
    #[display("g")]
    SmallG = b'g',
    #[display("h")]
    SmallH = b'h',
    #[display("i")]
    SmallI = b'i',
    #[display("j")]
    SmallJ = b'j',
    #[display("k")]
    SmallK = b'k',
    #[display("l")]
    SmallL = b'l',
    #[display("m")]
    SmallM = b'm',
    #[display("n")]
    SmallN = b'n',
    #[display("o")]
    SmallO = b'o',
    #[display("p")]
    SmallP = b'p',
    #[display("q")]
    SmallQ = b'q',
    #[display("r")]
    SmallR = b'r',
    #[display("s")]
    SmallS = b's',
    #[display("t")]
    SmallT = b't',
    #[display("u")]
    SmallU = b'u',
    #[display("v")]
    SmallV = b'v',
    #[display("w")]
    SmallW = b'w',
    #[display("x")]
    SmallX = b'x',
    #[display("y")]
    SmallY = b'y',
    #[display("z")]
    SmallZ = b'z',
    #[display("0")]
    Zero = b'0',
    #[display("1")]
    One = b'1',
    #[display("2")]
    Two = b'2',
    #[display("3")]
    Three = b'3',
    #[display("4")]
    Four = b'4',
    #[display("5")]
    Five = b'5',
    #[display("6")]
    Six = b'6',
    #[display("7")]
    Seven = b'7',
    #[display("8")]
    Eight = b'8',
    #[display("9")]
    Nine = b'9',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = STD_LIB, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum AlphaNumLodash {
    #[strict_type(dumb)]
    Lodash = b'_',
    A = b'A',
    B = b'B',
    C = b'C',
    D = b'D',
    E = b'E',
    F = b'F',
    G = b'G',
    H = b'H',
    I = b'I',
    J = b'J',
    K = b'K',
    L = b'L',
    M = b'M',
    N = b'N',
    O = b'O',
    P = b'P',
    Q = b'Q',
    R = b'R',
    S = b'S',
    T = b'T',
    U = b'U',
    V = b'V',
    W = b'W',
    X = b'X',
    Y = b'Y',
    Z = b'Z',
    #[display("a")]
    SmallA = b'a',
    #[display("b")]
    SmallB = b'b',
    #[display("c")]
    SmallC = b'c',
    #[display("d")]
    SmallD = b'd',
    #[display("e")]
    SmallE = b'e',
    #[display("f")]
    SmallF = b'f',
    #[display("g")]
    SmallG = b'g',
    #[display("h")]
    SmallH = b'h',
    #[display("i")]
    SmallI = b'i',
    #[display("j")]
    SmallJ = b'j',
    #[display("k")]
    SmallK = b'k',
    #[display("l")]
    SmallL = b'l',
    #[display("m")]
    SmallM = b'm',
    #[display("n")]
    SmallN = b'n',
    #[display("o")]
    SmallO = b'o',
    #[display("p")]
    SmallP = b'p',
    #[display("q")]
    SmallQ = b'q',
    #[display("r")]
    SmallR = b'r',
    #[display("s")]
    SmallS = b's',
    #[display("t")]
    SmallT = b't',
    #[display("u")]
    SmallU = b'u',
    #[display("v")]
    SmallV = b'v',
    #[display("w")]
    SmallW = b'w',
    #[display("x")]
    SmallX = b'x',
    #[display("y")]
    SmallY = b'y',
    #[display("z")]
    SmallZ = b'z',
    #[display("0")]
    Zero = b'0',
    #[display("1")]
    One = b'1',
    #[display("2")]
    Two = b'2',
    #[display("3")]
    Three = b'3',
    #[display("4")]
    Four = b'4',
    #[display("5")]
    Five = b'5',
    #[display("6")]
    Six = b'6',
    #[display("7")]
    Seven = b'7',
    #[display("8")]
    Eight = b'8',
    #[display("9")]
    Nine = b'9',
}
