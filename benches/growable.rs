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

extern crate arrow2;

use arrow2::{
    array::growable::{Growable, GrowablePrimitive},
    datatypes::DataType,
    util::bench_util::create_primitive_array,
};

use criterion::{criterion_group, criterion_main, Criterion};

fn add_benchmark(c: &mut Criterion) {
    let values = (0..1026).rev();

    let i32_array = create_primitive_array::<i32>(1026 * 10, DataType::Int32, 0.0);
    c.bench_function("growable::primitive::non_null::non_null", |b| {
        b.iter(|| {
            let mut a = GrowablePrimitive::new(&[&i32_array], false, 1026 * 10);
            values
                .clone()
                .into_iter()
                .for_each(|start| a.extend(0, start, 10))
        })
    });

    let i32_array = create_primitive_array::<i32>(1026 * 10, DataType::Int32, 0.0);
    c.bench_function("growable::primitive::non_null::null", |b| {
        b.iter(|| {
            let mut a = GrowablePrimitive::new(&[&i32_array], true, 1026 * 10);
            values.clone().into_iter().for_each(|start| {
                if start % 2 == 0 {
                    a.extend_validity(10);
                } else {
                    a.extend(0, start, 10)
                }
            })
        })
    });

    let i32_array = create_primitive_array::<i32>(1026 * 10, DataType::Int32, 0.1);

    let values = values.collect::<Vec<_>>();
    c.bench_function("growable::primitive::null::non_null", |b| {
        b.iter(|| {
            let mut a = GrowablePrimitive::new(&[&i32_array], false, 1026 * 10);
            values
                .clone()
                .into_iter()
                .for_each(|start| a.extend(0, start, 10))
        })
    });
    c.bench_function("growable::primitive::null::null", |b| {
        b.iter(|| {
            let mut a = GrowablePrimitive::new(&[&i32_array], true, 1026 * 10);
            values.clone().into_iter().for_each(|start| {
                if start % 2 == 0 {
                    a.extend_validity(10);
                } else {
                    a.extend(0, start, 10)
                }
            })
        })
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
