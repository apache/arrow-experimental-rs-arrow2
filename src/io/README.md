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

# IO module

This document describes the overall design of this module.

## Rules:

* Each directory in this module corresponds to a specific format such as `csv` and `json`.
* directories that depend on external dependencies MUST be feature gated, with a feature named with a prefix `io_`.
* modules MUST re-export any API of external dependencies they require as part of their public API. E.g.
    * if a module as an API `write(writer: &mut csv:Writer<W>, ...)`, it MUST contain `pub use csv::Writer;`.

    The rational is that adding this crate to `cargo.toml` must be sufficient to use it.
* Each directory SHOULD contain two directories, `read` and `write`, corresponding to functionality about 
reading from the format and writing to the format respectively.
* The base module SHOULD contain `use pub read;` and `use pub write;`.
* Implementations SHOULD separate reading of "data" from reading of "metadata". Examples:
    * schema read or inference SHOULD be a separate function
    * functions that read "data" SHOULD consume a schema typically pre-read.
* Implementations SHOULD separate IO-bounded operations from CPU-bounded operations. I.e. implementations SHOULD:
    * contain functions that consume a `Read` implementor and output a "raw" struct, i.e. a struct that is e.g. compressed and serialized
    * contain functions that consume a "raw" struct and convert it into Arrow.
    * offer each of these functions as independent public APIs, so that consumers can decide how to balance CPU-bounds and IO-bounds.
