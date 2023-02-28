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

#[macro_export]
macro_rules! strict_dumb {
    () => {
        $crate::StrictDumb::strict_dumb()
    };
}

#[macro_export]
macro_rules! impl_strict_newtype {
    ($ty:ident, $lib:expr) => {
        impl_strict_newtype!($ty, $lib, Self($crate::StrictDumb::strict_dumb()));
    };
    ($ty:ident, $lib:expr, $dumb:expr) => {
        impl $crate::StrictDumb for $ty {
            fn strict_dumb() -> Self { Self::from($dumb) }
        }
        impl $crate::StrictType for $ty {
            const STRICT_LIB_NAME: &'static str = $lib;
        }
        impl $crate::StrictProduct for $ty {}
        impl $crate::StrictTuple for $ty {
            const FIELD_COUNT: u8 = 1;
        }
        impl $crate::StrictEncode for $ty {
            fn strict_encode<W: $crate::TypedWrite>(&self, writer: W) -> ::std::io::Result<W> {
                writer.write_newtype::<Self>(&self.0)
            }
        }
        impl $crate::StrictDecode for $ty {
            fn strict_decode(
                reader: &mut impl $crate::TypedRead,
            ) -> Result<Self, $crate::DecodeError> {
                use $crate::ReadTuple;
                reader.read_tuple(|r| Ok(Self(r.read_field()?)))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_strict_struct {
    ($ty:ty, $lib:expr; $($field:ident),+ $(,)?) => {
        impl_strict_struct!($ty, $lib; $($field => $crate::strict_dumb!()),+);
    };
    ($ty:ty, $lib:expr; $($field:ident => $dumb:expr),+ $(,)?) => {
        impl $crate::StrictDumb for $ty {
            fn strict_dumb() -> Self {
                Self {
                    $($field: $dumb),+
                }
            }
        }
        impl $crate::StrictType for $ty {
            const STRICT_LIB_NAME: &'static str = $lib;
        }
        impl $crate::StrictProduct for $ty {}
        impl $crate::StrictStruct for $ty {
            const ALL_FIELDS: &'static [&'static str] = &[$(stringify!($field)),+];
        }
        impl $crate::StrictEncode for $ty {
            fn strict_encode<W: $crate::TypedWrite>(&self, writer: W) -> io::Result<W> {
                writer.write_struct::<Self>(|w| {
                    Ok(w
                        $(.write_field(fname!(stringify!($field)), &self.$field)?)+
                        .complete())
                })
            }
        }
        impl $crate::StrictDecode for $ty {
            fn strict_decode(reader: &mut impl $crate::TypedRead) -> Result<Self, $crate::DecodeError> {
                reader.read_struct(|r| {
                    $(let $field = r.read_field(fname!(stringify!($field)))?;)+
                    Ok(Self { $($field),+ })
                })
            }
        }
    };
}

#[macro_export]
macro_rules! tn {
    ($name:literal) => {
        $crate::TypeName::from($name).into()
    };
    ($name:expr) => {
        {
            let name_copy = $name.clone();
            $crate::TypeName::try_from($name)
                .unwrap_or_else(|_| panic!("invalid type name `{name_copy}` from formatter"))
                .into()
        }
    };
    ($name:literal, $($arg:expr),+) => {
        tn!(format!($name, $($arg),+))
    };
}

#[macro_export]
macro_rules! vname {
    ($name:literal) => {
        $crate::VariantName::from($name).into()
    };
    ($name:expr) => {
        $crate::VariantName::from($name).into()
    };
}

#[macro_export]
macro_rules! fname {
    ($name:literal) => {
        $crate::FieldName::from($name).into()
    };
    ($name:expr) => {
        $crate::FieldName::from($name).into()
    };
}

#[macro_export]
macro_rules! libname {
    ($name:literal) => {
        $crate::LibName::from($name)
    };
    ($name:expr) => {
        $crate::LibName::from($name)
    };
}
