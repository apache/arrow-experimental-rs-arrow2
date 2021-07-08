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

use crate::{
    array::*,
    buffer::Buffer,
    types::{NativeType, NaturalDataType},
};
use crate::{
    array::{Offset, Utf8Array},
    error::Result,
};

pub(super) fn boolean_to_primitive_dyn<T>(array: &dyn Array) -> Result<Box<dyn Array>>
where
    T: NativeType + NaturalDataType + num::One,
{
    let array = array.as_any().downcast_ref().unwrap();
    Ok(Box::new(boolean_to_primitive::<T>(array)))
}

/// Casts the [`BooleanArray`] to a [`PrimitiveArray`].
pub fn boolean_to_primitive<T>(from: &BooleanArray) -> PrimitiveArray<T>
where
    T: NativeType + NaturalDataType + num::One,
{
    let iter = from
        .values()
        .iter()
        .map(|x| if x { T::one() } else { T::default() });
    let values = Buffer::<T>::from_trusted_len_iter(iter);

    PrimitiveArray::<T>::from_data(T::DATA_TYPE, values, from.validity().clone())
}

/// Casts the [`BooleanArray`] to a [`Utf8Array`], casting trues to `"1"` and falses to `"0"`
pub fn boolean_to_utf8<O: Offset>(from: &BooleanArray) -> Utf8Array<O> {
    let iter = from.values().iter().map(|x| if x { "1" } else { "0" });
    Utf8Array::from_trusted_len_values_iter(iter)
}

pub(super) fn boolean_to_utf8_dyn<O: Offset>(array: &dyn Array) -> Result<Box<dyn Array>> {
    let array = array.as_any().downcast_ref().unwrap();
    Ok(Box::new(boolean_to_utf8::<O>(array)))
}
