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

use std::fs::File;
use std::iter::once;

use arrow2::io::parquet::write::to_parquet_schema;
use arrow2::{
    array::{Array, Int32Array},
    datatypes::{Field, Schema},
    error::Result,
    io::parquet::write::{array_to_page, write_file, CompressionCodec, DynIter, WriteOptions},
};

fn write_single_array(path: &str, array: &dyn Array, field: Field) -> Result<()> {
    let schema = Schema::new(vec![field]);

    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionCodec::Uncompressed,
    };

    // map arrow fields to parquet fields
    let parquet_schema = to_parquet_schema(&schema)?;

    // Declare the row group iterator. This must be an iterator of iterators of iterators:
    // * first iterator of row groups
    // * second iterator of column chunks
    // * third iterator of pages
    // an array can be divided in multiple pages via `.slice(offset, length)` (`O(1)`).
    // All column chunks within a row group MUST have the same length.
    let row_groups = once(Result::Ok(DynIter::new(once(Ok(DynIter::new(
        once(array)
            .zip(parquet_schema.columns().to_vec().into_iter())
            .map(|(array, descriptor)| array_to_page(array, descriptor, options)),
    ))))));

    // Create a new empty file
    let mut file = File::create(path)?;

    // Write the file. Note that, at present, any error results in a corrupted file.
    write_file(
        &mut file,
        row_groups,
        &schema,
        parquet_schema,
        options,
        None,
    )
}

fn main() -> Result<()> {
    let array = Int32Array::from(&[
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
    ]);
    let field = Field::new("c1", array.data_type().clone(), true);
    write_single_array("test.parquet", &array, field)
}
