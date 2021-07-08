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

use crate::bitmap::Bitmap;
use parquet2::encoding::hybrid_rle::encode;

use crate::error::Result;

#[inline]
fn encode_iter<I: Iterator<Item = bool>>(iter: I) -> Result<Vec<u8>> {
    let mut buffer = std::io::Cursor::new(vec![0; 4]);
    buffer.set_position(4);
    encode(&mut buffer, iter)?;
    let mut buffer = buffer.into_inner();
    let length = buffer.len() - 4;
    // todo: pay this small debt (loop?)
    let length = length.to_le_bytes();
    buffer[0] = length[0];
    buffer[1] = length[1];
    buffer[2] = length[2];
    buffer[3] = length[3];
    Ok(buffer)
}

/// writes the def levels to a `Vec<u8>` and returns it.
/// Note that this function
#[inline]
pub fn write_def_levels(
    is_optional: bool,
    validity: &Option<Bitmap>,
    len: usize,
) -> Result<Vec<u8>> {
    // encode def levels
    match (is_optional, validity) {
        (true, Some(validity)) => encode_iter(validity.iter()),
        (true, None) => encode_iter(std::iter::repeat(true).take(len)),
        _ => Ok(vec![]), // is required => no def levels
    }
}
