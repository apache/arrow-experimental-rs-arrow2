<!---
  Licensed to the Apache Software Foundation (ASF) under one
  or more contributor license agreements.  See the NOTICE file
  distributed with this work for additional information
  regarding copyright ownership.  The ASF licenses this file
  to you under the Apache License, Version 2.0 (the
  "License"); you may not use this file except in compliance
  with the License.  You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing,
  software distributed under the License is distributed on an
  "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
  KIND, either express or implied.  See the License for the
  specific language governing permissions and limitations
  under the License.
-->

# Arrow2

Arrow2 is a Rust library that implements data structures and functionality enabling
interoperability with the arrow format.

The typical use-case for this library is to perform CPU and memory-intensive analytics in a format that supports heterogeneous data structures, null values, and IPC and FFI interfaces across languages.

Arrow2 is divided into two main parts: a [low-level API](./low_level.md) to efficiently
operate with contiguous memory regions, and a [high-level API](./high_level.md) to operate with
arrow arrays, logical types, schemas, etc.

This repo started as an experiment forked from the Apache arrow project to offer a transmute-free
Rust implementation of that crate.

## Cargo features

This crate has a significant number of cargo features to reduce compilation times and dependencies blowup.
There is also a feature `simd`, that requires the nightly channel, that produces more explicit SIMD instructions via [`packed_simd`](https://github.com/rust-lang/packed_simd).
