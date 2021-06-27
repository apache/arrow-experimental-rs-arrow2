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

# Crate's design

This document describes the design of this module, and thus the overall crate.
Each module MAY have its own design document, that concerns specifics of that module, and if yes,
it MUST be on each module's `README.md`.

## Equality

Array equality is not defined in the Arrow specification. This crate follows the intent of the specification, but there is no guarantee that this no verification that this equals e.g. C++'s definition.

There is a single source of truth about whether two arrays are equal, and that is via their 
equality operators, defined on the module [`array/equal`](array/equal/mod.rs).

Implementation MUST use these operators for asserting equality, so that all testing follows the same definition of array equality.

## Error handling

* Errors from an external dependency MUST be encapsulated on `External`.
* Errors from IO MUST be encapsulated on `Io`.
* This crate MAY return `NotYetImplemented` when the functionality does not exist, or it MAY panic with `unimplemented!`.

## Logical and physical types

There is a strict separation between physical and logical types:

* physical types MUST be implemented via generics
* logical types MUST be implemented via variables (whose value is e.g. an `enum`)
* logical types MUST be declared and implemented on the `datatypes` module

## Source of undefined behavior

There is one, and only one, acceptable source of undefined behavior: FFI. It is impossible to prove that data passed via pointers are safe for consumption (only a promise from the specification).
