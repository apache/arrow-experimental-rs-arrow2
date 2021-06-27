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

use criterion::{criterion_group, criterion_main, Criterion};

use arrow2::array::*;
use arrow2::util::bench_util::*;
use arrow2::{compute::aggregate::*, datatypes::DataType};

fn bench_sum(arr_a: &PrimitiveArray<f32>) {
    sum(criterion::black_box(arr_a)).unwrap();
}

fn bench_min(arr_a: &PrimitiveArray<f32>) {
    min_primitive(criterion::black_box(arr_a)).unwrap();
}

fn add_benchmark(c: &mut Criterion) {
    (10..=20).step_by(2).for_each(|log2_size| {
        let size = 2usize.pow(log2_size);
        let arr_a = create_primitive_array::<f32>(size, DataType::Float32, 0.0);

        c.bench_function(&format!("sum 2^{} f32", log2_size), |b| {
            b.iter(|| bench_sum(&arr_a))
        });
        c.bench_function(&format!("min 2^{} f32", log2_size), |b| {
            b.iter(|| bench_min(&arr_a))
        });

        let arr_a = create_primitive_array::<f32>(size, DataType::Float32, 0.1);

        c.bench_function(&format!("sum null 2^{} f32", log2_size), |b| {
            b.iter(|| bench_sum(&arr_a))
        });

        c.bench_function(&format!("min null 2^{} f32", log2_size), |b| {
            b.iter(|| bench_min(&arr_a))
        });
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
