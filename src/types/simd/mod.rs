// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use super::{BitChunk, NativeType};

pub trait FromMaskChunk<T> {
    /// Convert itself from a slice.
    fn from_chunk(v: T) -> Self;
}

/// A struct lends itself well to be compiled leveraging SIMD
pub trait NativeSimd: Default {
    const LANES: usize;
    /// The [`NativeType`] of this struct. E.g. `f32` for a `NativeSimd = f32x16`.
    type Native: NativeType;
    /// The type holding bits for masks.
    type Chunk: BitChunk;
    type Mask: FromMaskChunk<Self::Chunk>;

    fn select(self, mask: Self::Mask, default: Self) -> Self;

    /// Convert itself from a slice.
    /// # Panics
    /// * iff `v.len()` != `T::LANES`
    fn from_chunk(v: &[Self::Native]) -> Self;

    /// creates a new Self from `v` by populating items from `v` up to its length.
    /// Items from `v` at positions larger than the number of lanes are ignored;
    /// remaining items are populated with `remaining`.
    fn from_incomplete_chunk(v: &[Self::Native], remaining: Self::Native) -> Self;
}

/// Trait implemented by some [`NativeType`] that have a SIMD representation.
pub trait Simd: NativeType {
    /// The SIMD type associated with this trait.
    /// This type supports SIMD operations
    type Simd: NativeSimd<Native = Self>;
}

#[cfg(not(feature = "simd"))]
mod native;
#[cfg(not(feature = "simd"))]
pub use native::*;
#[cfg(feature = "simd")]
mod packed;
#[cfg(feature = "simd")]
pub use packed::*;

macro_rules! native {
    ($type:ty, $simd:ty) => {
        impl Simd for $type {
            type Simd = $simd;
        }
    };
}

native!(u8, u8x64);
native!(u16, u16x32);
native!(u32, u32x16);
native!(u64, u64x8);
native!(i8, i8x64);
native!(i16, i16x32);
native!(i32, i32x16);
native!(i64, i64x8);
native!(f32, f32x16);
native!(f64, f64x8);
