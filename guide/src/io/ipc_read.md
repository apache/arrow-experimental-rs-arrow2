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

# Read Arrow IPC

When compiled with feature `io_ipc`, this crate can be used to read Arrow IPC files.

An Arrow IPC file is composed by a header, a footer, and blocks of `RecordBatch`es.
Reading it generally consists of:

1. read metadata, containing the block positions in the file
2. seek to each block and read it

```rust
{{#include ../../../examples/ipc_file_read.rs}}
```
