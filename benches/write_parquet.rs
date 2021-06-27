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

use std::io::Cursor;

use criterion::{criterion_group, criterion_main, Criterion};

use arrow2::array::*;
use arrow2::datatypes::{DataType, Field, Schema};
use arrow2::error::Result;
use arrow2::io::parquet::write::*;
use arrow2::util::bench_util::{create_boolean_array, create_primitive_array, create_string_array};

fn write(array: &dyn Array) -> Result<()> {
    let field = Field::new("c1", array.data_type().clone(), true);
    let schema = Schema::new(vec![field]);

    let compression = CompressionCodec::Uncompressed;

    let parquet_type = to_parquet_type(&field)?;

    let row_groups = std::iter::once(Result::Ok(std::iter::once(Ok(std::iter::once(
        array_to_page(array, &parquet_type, compression),
    )))));

    let mut writer = Cursor::new(vec![]);
    write_file(&mut writer, row_groups, schema, compression, None)?;
    Ok(())
}

fn add_benchmark(c: &mut Criterion) {
    (0..=10).step_by(2).for_each(|i| {
        let array = &create_primitive_array::<i64>(1024 * 2usize.pow(i), DataType::Int64, 0.1);
        let a = format!("write i64 2^{}", 10 + i);
        c.bench_function(&a, |b| b.iter(|| write(array).unwrap()));
    });

    (0..=10).step_by(2).for_each(|i| {
        let array = &create_boolean_array(1024 * 2usize.pow(i), 0.1, 0.5);
        let a = format!("write bool 2^{}", 10 + i);
        c.bench_function(&a, |b| b.iter(|| write(array).unwrap()));
    });

    (0..=10).step_by(2).for_each(|i| {
        let array = &create_string_array::<i32>(1024 * 2usize.pow(i), 0.1);
        let a = format!("write utf8 2^{}", 10 + i);
        c.bench_function(&a, |b| b.iter(|| write(array).unwrap()));
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
