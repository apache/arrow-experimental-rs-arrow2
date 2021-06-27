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

use parquet2::{
    compression::create_codec,
    encoding::Encoding,
    metadata::ColumnDescriptor,
    read::{CompressedPage, PageHeader},
    schema::DataPageHeader,
    write::WriteOptions,
};

use super::utils;
use crate::{
    array::{Array, FixedSizeBinaryArray},
    error::Result,
    io::parquet::read::is_type_nullable,
};

pub fn array_to_page_v1(
    array: &FixedSizeBinaryArray,
    options: WriteOptions,
    descriptor: ColumnDescriptor,
) -> Result<CompressedPage> {
    let is_optional = is_type_nullable(descriptor.type_());
    let validity = array.validity();

    let mut buffer = utils::write_def_levels(is_optional, validity, array.len())?;

    if is_optional {
        // append the non-null values
        array.iter().for_each(|x| {
            if let Some(x) = x {
                buffer.extend_from_slice(x);
            }
        });
    } else {
        // append all values
        buffer.extend_from_slice(array.values());
    }

    let uncompressed_page_size = buffer.len();

    let codec = create_codec(&options.compression)?;
    let buffer = if let Some(mut codec) = codec {
        // todo: remove this allocation by extending `buffer` directly.
        // needs refactoring `compress`'s API.
        let mut tmp = vec![];
        codec.compress(&buffer, &mut tmp)?;
        tmp
    } else {
        buffer
    };

    let header = PageHeader::V1(DataPageHeader {
        num_values: array.len() as i32,
        encoding: Encoding::Plain,
        definition_level_encoding: Encoding::Rle,
        repetition_level_encoding: Encoding::Rle,
        statistics: None,
    });

    Ok(CompressedPage::new(
        header,
        buffer,
        options.compression,
        uncompressed_page_size,
        None,
        descriptor,
    ))
}
