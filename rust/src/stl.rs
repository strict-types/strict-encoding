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

#![allow(non_camel_case_types)]

use amplify::ascii::AsciiChar;
use amplify::num::u4;

use crate::{
    DecodeError, EncodeError, StrictDecode, StrictDumb, StrictEncode, StrictEnum, StrictSum,
    StrictType, TypeName, TypedRead, TypedWrite, VariantError, LIB_NAME_STD,
};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_STD, tags = repr, into_u8, try_from_u8, crate = crate)]
#[repr(u8)]
pub enum Bool {
    #[default]
    False = 0,
    True = 1,
}

impl From<&bool> for Bool {
    fn from(value: &bool) -> Self { Bool::from(*value) }
}
impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        match value {
            true => Bool::True,
            false => Bool::False,
        }
    }
}
impl From<&Bool> for bool {
    fn from(value: &Bool) -> Self { bool::from(*value) }
}
impl From<Bool> for bool {
    fn from(value: Bool) -> Self {
        match value {
            Bool::False => false,
            Bool::True => true,
        }
    }
}

impl StrictType for bool {
    const STRICT_LIB_NAME: &'static str = LIB_NAME_STD;
}
impl StrictEncode for bool {
    fn strict_encode<W: TypedWrite>(&self, writer: W) -> Result<W, EncodeError> {
        writer.write_enum::<Bool>(Bool::from(self))
    }
}
impl StrictDecode for bool {
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        let v: Bool = reader.read_enum()?;
        Ok(bool::from(v))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_STD, tags = repr, into_u8, try_from_u8, crate = crate)]
#[repr(u8)]
pub enum U4 {
    #[default]
    #[strict_type(rename = "_0")]
    _0 = 0,
    #[strict_type(rename = "_1")]
    _1,
    #[strict_type(rename = "_2")]
    _2,
    #[strict_type(rename = "_3")]
    _3,
    #[strict_type(rename = "_4")]
    _4,
    #[strict_type(rename = "_5")]
    _5,
    #[strict_type(rename = "_6")]
    _6,
    #[strict_type(rename = "_7")]
    _7,
    #[strict_type(rename = "_8")]
    _8,
    #[strict_type(rename = "_9")]
    _9,
    #[strict_type(rename = "_10")]
    _10,
    #[strict_type(rename = "_11")]
    _11,
    #[strict_type(rename = "_12")]
    _12,
    #[strict_type(rename = "_13")]
    _13,
    #[strict_type(rename = "_14")]
    _14,
    #[strict_type(rename = "_15")]
    _15,
}

impl StrictType for u4 {
    const STRICT_LIB_NAME: &'static str = LIB_NAME_STD;
    fn strict_name() -> Option<TypeName> { Some(tn!("U4")) }
}
impl StrictEncode for u4 {
    fn strict_encode<W: TypedWrite>(&self, writer: W) -> Result<W, EncodeError> {
        writer.write_enum::<U4>(U4::try_from(self.to_u8()).expect("broken u4 types guarantees"))
    }
}
impl StrictDecode for u4 {
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        let v: U4 = reader.read_enum()?;
        Ok(u4::with(v as u8))
    }
}

#[derive(Wrapper, WrapperMut, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display, Debug)]
#[wrapper_mut(DerefMut)]
#[derive(StrictDumb)]
#[strict_type(lib = LIB_NAME_STD, dumb = Self(AsciiChar::A), crate = crate)]
pub struct AsciiPrintable(AsciiChar);

impl From<AsciiPrintable> for u8 {
    fn from(value: AsciiPrintable) -> Self { value.0.as_byte() }
}

impl TryFrom<u8> for AsciiPrintable {
    type Error = VariantError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        AsciiChar::from_ascii(value)
            .map_err(|_| VariantError(AsciiPrintable::strict_name(), value))
            .map(Self)
    }
}

impl StrictType for AsciiPrintable {
    const STRICT_LIB_NAME: &'static str = LIB_NAME_STD;
}

impl StrictSum for AsciiPrintable {
    const ALL_VARIANTS: &'static [(u8, &'static str)] = &[
        (b' ', "space"),
        (b'!', "excl"),
        (b'"', "quotes"),
        (b'#', "hash"),
        (b'$', "dollar"),
        (b'%', "percent"),
        (b'&', "ampersand"),
        (b'\'', "apostrophe"),
        (b'(', "bracketL"),
        (b')', "bracketR"),
        (b'*', "asterisk"),
        (b'+', "plus"),
        (b',', "comma"),
        (b'-', "minus"),
        (b'.', "dot"),
        (b'/', "slash"),
        (b'0', "zero"),
        (b'1', "one"),
        (b'2', "two"),
        (b'3', "three"),
        (b'4', "four"),
        (b'5', "five"),
        (b'6', "six"),
        (b'7', "seven"),
        (b'8', "eight"),
        (b'9', "nine"),
        (b':', "colon"),
        (b';', "semiColon"),
        (b'<', "less"),
        (b'=', "equal"),
        (b'>', "greater"),
        (b'?', "question"),
        (b'@', "at"),
        (b'A', "A"),
        (b'B', "B"),
        (b'C', "C"),
        (b'D', "D"),
        (b'E', "E"),
        (b'F', "F"),
        (b'G', "G"),
        (b'H', "H"),
        (b'I', "I"),
        (b'J', "J"),
        (b'K', "K"),
        (b'L', "L"),
        (b'M', "M"),
        (b'N', "N"),
        (b'O', "O"),
        (b'P', "P"),
        (b'Q', "Q"),
        (b'R', "R"),
        (b'S', "S"),
        (b'T', "T"),
        (b'U', "U"),
        (b'V', "V"),
        (b'W', "W"),
        (b'X', "X"),
        (b'Y', "Y"),
        (b'Z', "Z"),
        (b'[', "sqBracketL"),
        (b'\\', "backSlash"),
        (b']', "sqBracketR"),
        (b'^', "caret"),
        (b'_', "lodash"),
        (b'`', "backtick"),
        (b'a', "a"),
        (b'b', "b"),
        (b'c', "c"),
        (b'd', "d"),
        (b'e', "e"),
        (b'f', "f"),
        (b'g', "g"),
        (b'h', "h"),
        (b'i', "i"),
        (b'j', "j"),
        (b'k', "k"),
        (b'l', "l"),
        (b'm', "m"),
        (b'n', "n"),
        (b'o', "o"),
        (b'p', "p"),
        (b'q', "q"),
        (b'r', "r"),
        (b's', "s"),
        (b't', "t"),
        (b'u', "u"),
        (b'v', "v"),
        (b'w', "w"),
        (b'x', "x"),
        (b'y', "y"),
        (b'z', "z"),
        (b'{', "cBracketL"),
        (b'|', "pipe"),
        (b'}', "cBracketR"),
        (b'~', "tilde"),
    ];
    fn variant_name(&self) -> &'static str {
        Self::ALL_VARIANTS
            .into_iter()
            .find(|(s, _)| *s == self.as_byte())
            .map(|(_, v)| *v)
            .expect("missed ASCII character variant")
    }
}
impl StrictEnum for AsciiPrintable {}
impl StrictEncode for AsciiPrintable {
    fn strict_encode<W: TypedWrite>(&self, writer: W) -> Result<W, EncodeError> {
        writer.write_enum(*self)
    }
}
impl StrictDecode for AsciiPrintable {
    fn strict_decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
        reader.read_enum()
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_STD, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum AlphaCaps {
    #[strict_type(dumb, rename = "A")]
    A = b'A',
    #[strict_type(rename = "B")]
    B = b'B',
    #[strict_type(rename = "C")]
    C = b'C',
    #[strict_type(rename = "D")]
    D = b'D',
    #[strict_type(rename = "E")]
    E = b'E',
    #[strict_type(rename = "F")]
    F = b'F',
    #[strict_type(rename = "G")]
    G = b'G',
    #[strict_type(rename = "H")]
    H = b'H',
    #[strict_type(rename = "I")]
    I = b'I',
    #[strict_type(rename = "J")]
    J = b'J',
    #[strict_type(rename = "K")]
    K = b'K',
    #[strict_type(rename = "L")]
    L = b'L',
    #[strict_type(rename = "M")]
    M = b'M',
    #[strict_type(rename = "N")]
    N = b'N',
    #[strict_type(rename = "O")]
    O = b'O',
    #[strict_type(rename = "P")]
    P = b'P',
    #[strict_type(rename = "Q")]
    Q = b'Q',
    #[strict_type(rename = "R")]
    R = b'R',
    #[strict_type(rename = "S")]
    S = b'S',
    #[strict_type(rename = "T")]
    T = b'T',
    #[strict_type(rename = "U")]
    U = b'U',
    #[strict_type(rename = "V")]
    V = b'V',
    #[strict_type(rename = "W")]
    W = b'W',
    #[strict_type(rename = "X")]
    X = b'X',
    #[strict_type(rename = "Y")]
    Y = b'Y',
    #[strict_type(rename = "Z")]
    Z = b'Z',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_STD, tags = repr, into_u8, try_from_u8, crate = crate)]
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
#[strict_type(lib = LIB_NAME_STD, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum Alpha {
    #[strict_type(dumb, rename = "A")]
    A = b'A',
    #[strict_type(rename = "B")]
    B = b'B',
    #[strict_type(rename = "C")]
    C = b'C',
    #[strict_type(rename = "D")]
    D = b'D',
    #[strict_type(rename = "E")]
    E = b'E',
    #[strict_type(rename = "F")]
    F = b'F',
    #[strict_type(rename = "G")]
    G = b'G',
    #[strict_type(rename = "H")]
    H = b'H',
    #[strict_type(rename = "I")]
    I = b'I',
    #[strict_type(rename = "J")]
    J = b'J',
    #[strict_type(rename = "K")]
    K = b'K',
    #[strict_type(rename = "L")]
    L = b'L',
    #[strict_type(rename = "M")]
    M = b'M',
    #[strict_type(rename = "N")]
    N = b'N',
    #[strict_type(rename = "O")]
    O = b'O',
    #[strict_type(rename = "P")]
    P = b'P',
    #[strict_type(rename = "Q")]
    Q = b'Q',
    #[strict_type(rename = "R")]
    R = b'R',
    #[strict_type(rename = "S")]
    S = b'S',
    #[strict_type(rename = "T")]
    T = b'T',
    #[strict_type(rename = "U")]
    U = b'U',
    #[strict_type(rename = "V")]
    V = b'V',
    #[strict_type(rename = "W")]
    W = b'W',
    #[strict_type(rename = "X")]
    X = b'X',
    #[strict_type(rename = "Y")]
    Y = b'Y',
    #[strict_type(rename = "Z")]
    Z = b'Z',
    #[display("a")]
    a = b'a',
    #[display("b")]
    b = b'b',
    #[display("c")]
    c = b'c',
    #[display("d")]
    d = b'd',
    #[display("e")]
    e = b'e',
    #[display("f")]
    f = b'f',
    #[display("g")]
    g = b'g',
    #[display("h")]
    h = b'h',
    #[display("i")]
    i = b'i',
    #[display("j")]
    j = b'j',
    #[display("k")]
    k = b'k',
    #[display("l")]
    l = b'l',
    #[display("m")]
    m = b'm',
    #[display("n")]
    n = b'n',
    #[display("o")]
    o = b'o',
    #[display("p")]
    p = b'p',
    #[display("q")]
    q = b'q',
    #[display("r")]
    r = b'r',
    #[display("s")]
    s = b's',
    #[display("t")]
    t = b't',
    #[display("u")]
    u = b'u',
    #[display("v")]
    v = b'v',
    #[display("w")]
    w = b'w',
    #[display("x")]
    x = b'x',
    #[display("y")]
    y = b'y',
    #[display("z")]
    z = b'z',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_STD, tags = repr, into_u8, try_from_u8, crate = crate)]
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
#[strict_type(lib = LIB_NAME_STD, tags = repr, into_u8, try_from_u8, crate = crate)]
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
#[strict_type(lib = LIB_NAME_STD, tags = repr, into_u8, try_from_u8, crate = crate)]
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
#[strict_type(lib = LIB_NAME_STD, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum AlphaCapsNum {
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
    #[strict_type(dumb, rename = "A")]
    A = b'A',
    #[strict_type(rename = "B")]
    B = b'B',
    #[strict_type(rename = "C")]
    C = b'C',
    #[strict_type(rename = "D")]
    D = b'D',
    #[strict_type(rename = "E")]
    E = b'E',
    #[strict_type(rename = "F")]
    F = b'F',
    #[strict_type(rename = "G")]
    G = b'G',
    #[strict_type(rename = "H")]
    H = b'H',
    #[strict_type(rename = "I")]
    I = b'I',
    #[strict_type(rename = "J")]
    J = b'J',
    #[strict_type(rename = "K")]
    K = b'K',
    #[strict_type(rename = "L")]
    L = b'L',
    #[strict_type(rename = "M")]
    M = b'M',
    #[strict_type(rename = "N")]
    N = b'N',
    #[strict_type(rename = "O")]
    O = b'O',
    #[strict_type(rename = "P")]
    P = b'P',
    #[strict_type(rename = "Q")]
    Q = b'Q',
    #[strict_type(rename = "R")]
    R = b'R',
    #[strict_type(rename = "S")]
    S = b'S',
    #[strict_type(rename = "T")]
    T = b'T',
    #[strict_type(rename = "U")]
    U = b'U',
    #[strict_type(rename = "V")]
    V = b'V',
    #[strict_type(rename = "W")]
    W = b'W',
    #[strict_type(rename = "X")]
    X = b'X',
    #[strict_type(rename = "Y")]
    Y = b'Y',
    #[strict_type(rename = "Z")]
    Z = b'Z',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_STD, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum AlphaNum {
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
    #[strict_type(dumb, rename = "A")]
    A = b'A',
    #[strict_type(rename = "B")]
    B = b'B',
    #[strict_type(rename = "C")]
    C = b'C',
    #[strict_type(rename = "D")]
    D = b'D',
    #[strict_type(rename = "E")]
    E = b'E',
    #[strict_type(rename = "F")]
    F = b'F',
    #[strict_type(rename = "G")]
    G = b'G',
    #[strict_type(rename = "H")]
    H = b'H',
    #[strict_type(rename = "I")]
    I = b'I',
    #[strict_type(rename = "J")]
    J = b'J',
    #[strict_type(rename = "K")]
    K = b'K',
    #[strict_type(rename = "L")]
    L = b'L',
    #[strict_type(rename = "M")]
    M = b'M',
    #[strict_type(rename = "N")]
    N = b'N',
    #[strict_type(rename = "O")]
    O = b'O',
    #[strict_type(rename = "P")]
    P = b'P',
    #[strict_type(rename = "Q")]
    Q = b'Q',
    #[strict_type(rename = "R")]
    R = b'R',
    #[strict_type(rename = "S")]
    S = b'S',
    #[strict_type(rename = "T")]
    T = b'T',
    #[strict_type(rename = "U")]
    U = b'U',
    #[strict_type(rename = "V")]
    V = b'V',
    #[strict_type(rename = "W")]
    W = b'W',
    #[strict_type(rename = "X")]
    X = b'X',
    #[strict_type(rename = "Y")]
    Y = b'Y',
    #[strict_type(rename = "Z")]
    Z = b'Z',
    #[display("a")]
    a = b'a',
    #[display("b")]
    b = b'b',
    #[display("c")]
    c = b'c',
    #[display("d")]
    d = b'd',
    #[display("e")]
    e = b'e',
    #[display("f")]
    f = b'f',
    #[display("g")]
    g = b'g',
    #[display("h")]
    h = b'h',
    #[display("i")]
    i = b'i',
    #[display("j")]
    j = b'j',
    #[display("k")]
    k = b'k',
    #[display("l")]
    l = b'l',
    #[display("m")]
    m = b'm',
    #[display("n")]
    n = b'n',
    #[display("o")]
    o = b'o',
    #[display("p")]
    p = b'p',
    #[display("q")]
    q = b'q',
    #[display("r")]
    r = b'r',
    #[display("s")]
    s = b's',
    #[display("t")]
    t = b't',
    #[display("u")]
    u = b'u',
    #[display("v")]
    v = b'v',
    #[display("w")]
    w = b'w',
    #[display("x")]
    x = b'x',
    #[display("y")]
    y = b'y',
    #[display("z")]
    z = b'z',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_STD, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum AlphaNumDash {
    #[strict_type(dumb)]
    #[display("-")]
    Dash = b'-',
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
    #[strict_type(dumb, rename = "A")]
    A = b'A',
    #[strict_type(rename = "B")]
    B = b'B',
    #[strict_type(rename = "C")]
    C = b'C',
    #[strict_type(rename = "D")]
    D = b'D',
    #[strict_type(rename = "E")]
    E = b'E',
    #[strict_type(rename = "F")]
    F = b'F',
    #[strict_type(rename = "G")]
    G = b'G',
    #[strict_type(rename = "H")]
    H = b'H',
    #[strict_type(rename = "I")]
    I = b'I',
    #[strict_type(rename = "J")]
    J = b'J',
    #[strict_type(rename = "K")]
    K = b'K',
    #[strict_type(rename = "L")]
    L = b'L',
    #[strict_type(rename = "M")]
    M = b'M',
    #[strict_type(rename = "N")]
    N = b'N',
    #[strict_type(rename = "O")]
    O = b'O',
    #[strict_type(rename = "P")]
    P = b'P',
    #[strict_type(rename = "Q")]
    Q = b'Q',
    #[strict_type(rename = "R")]
    R = b'R',
    #[strict_type(rename = "S")]
    S = b'S',
    #[strict_type(rename = "T")]
    T = b'T',
    #[strict_type(rename = "U")]
    U = b'U',
    #[strict_type(rename = "V")]
    V = b'V',
    #[strict_type(rename = "W")]
    W = b'W',
    #[strict_type(rename = "X")]
    X = b'X',
    #[strict_type(rename = "Y")]
    Y = b'Y',
    #[strict_type(rename = "Z")]
    Z = b'Z',
    #[display("a")]
    a = b'a',
    #[display("b")]
    b = b'b',
    #[display("c")]
    c = b'c',
    #[display("d")]
    d = b'd',
    #[display("e")]
    e = b'e',
    #[display("f")]
    f = b'f',
    #[display("g")]
    g = b'g',
    #[display("h")]
    h = b'h',
    #[display("i")]
    i = b'i',
    #[display("j")]
    j = b'j',
    #[display("k")]
    k = b'k',
    #[display("l")]
    l = b'l',
    #[display("m")]
    m = b'm',
    #[display("n")]
    n = b'n',
    #[display("o")]
    o = b'o',
    #[display("p")]
    p = b'p',
    #[display("q")]
    q = b'q',
    #[display("r")]
    r = b'r',
    #[display("s")]
    s = b's',
    #[display("t")]
    t = b't',
    #[display("u")]
    u = b'u',
    #[display("v")]
    v = b'v',
    #[display("w")]
    w = b'w',
    #[display("x")]
    x = b'x',
    #[display("y")]
    y = b'y',
    #[display("z")]
    z = b'z',
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_STD, tags = repr, into_u8, try_from_u8, crate = crate)]
#[display(inner)]
#[repr(u8)]
pub enum AlphaNumLodash {
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
    #[strict_type(dumb, rename = "A")]
    A = b'A',
    #[strict_type(rename = "B")]
    B = b'B',
    #[strict_type(rename = "C")]
    C = b'C',
    #[strict_type(rename = "D")]
    D = b'D',
    #[strict_type(rename = "E")]
    E = b'E',
    #[strict_type(rename = "F")]
    F = b'F',
    #[strict_type(rename = "G")]
    G = b'G',
    #[strict_type(rename = "H")]
    H = b'H',
    #[strict_type(rename = "I")]
    I = b'I',
    #[strict_type(rename = "J")]
    J = b'J',
    #[strict_type(rename = "K")]
    K = b'K',
    #[strict_type(rename = "L")]
    L = b'L',
    #[strict_type(rename = "M")]
    M = b'M',
    #[strict_type(rename = "N")]
    N = b'N',
    #[strict_type(rename = "O")]
    O = b'O',
    #[strict_type(rename = "P")]
    P = b'P',
    #[strict_type(rename = "Q")]
    Q = b'Q',
    #[strict_type(rename = "R")]
    R = b'R',
    #[strict_type(rename = "S")]
    S = b'S',
    #[strict_type(rename = "T")]
    T = b'T',
    #[strict_type(rename = "U")]
    U = b'U',
    #[strict_type(rename = "V")]
    V = b'V',
    #[strict_type(rename = "W")]
    W = b'W',
    #[strict_type(rename = "X")]
    X = b'X',
    #[strict_type(rename = "Y")]
    Y = b'Y',
    #[strict_type(rename = "Z")]
    Z = b'Z',
    #[strict_type(dumb)]
    #[display("_")]
    Lodash = b'_',
    #[display("a")]
    a = b'a',
    #[display("b")]
    b = b'b',
    #[display("c")]
    c = b'c',
    #[display("d")]
    d = b'd',
    #[display("e")]
    e = b'e',
    #[display("f")]
    f = b'f',
    #[display("g")]
    g = b'g',
    #[display("h")]
    h = b'h',
    #[display("i")]
    i = b'i',
    #[display("j")]
    j = b'j',
    #[display("k")]
    k = b'k',
    #[display("l")]
    l = b'l',
    #[display("m")]
    m = b'm',
    #[display("n")]
    n = b'n',
    #[display("o")]
    o = b'o',
    #[display("p")]
    p = b'p',
    #[display("q")]
    q = b'q',
    #[display("r")]
    r = b'r',
    #[display("s")]
    s = b's',
    #[display("t")]
    t = b't',
    #[display("u")]
    u = b'u',
    #[display("v")]
    v = b'v',
    #[display("w")]
    w = b'w',
    #[display("x")]
    x = b'x',
    #[display("y")]
    y = b'y',
    #[display("z")]
    z = b'z',
}
