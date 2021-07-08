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

//! Contains operators over arrays. This module's general design is
//! that each operator has two interfaces, a statically-typed version and a dynamically-typed
//! version.
//! The statically-typed version expects concrete arrays (like `PrimitiveArray`);
//! the dynamically-typed version expects `&dyn Array` and errors if the the type is not
//! supported.
//! Some dynamically-typed operators have an auxiliary function, `can_*`, that returns
//! true if the operator can be applied to the particular `DataType`.

pub mod aggregate;
pub mod arithmetics;
pub mod arity;
pub mod boolean;
pub mod boolean_kleene;
pub mod cast;
pub mod comparison;
pub mod concat;
pub mod contains;
pub mod filter;
pub mod hash;
pub mod if_then_else;
pub mod length;
pub mod limit;
pub mod nullif;
pub mod sort;
pub mod substring;
pub mod take;
pub mod temporal;
mod utils;
pub mod window;

#[cfg(feature = "regex")]
pub mod like;
#[cfg(feature = "regex")]
pub mod regex_match;

#[cfg(feature = "merge_sort")]
pub mod merge_sort;
