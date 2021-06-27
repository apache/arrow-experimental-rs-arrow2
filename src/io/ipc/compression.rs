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

use std::io::Read;

use crate::error::Result;

#[cfg(feature = "io_ipc_compression")]
pub fn decompress_lz4(input_buf: &[u8], output_buf: &mut [u8]) -> Result<()> {
    let mut decoder = lz4::Decoder::new(input_buf)?;
    decoder.read_exact(output_buf).map_err(|e| e.into())
}

#[cfg(feature = "io_ipc_compression")]
pub fn decompress_zstd(input_buf: &[u8], output_buf: &mut [u8]) -> Result<()> {
    let mut decoder = zstd::Decoder::new(input_buf)?;
    decoder.read_exact(output_buf).map_err(|e| e.into())
}

#[cfg(not(feature = "io_ipc_compression"))]
pub fn decompress_lz4(input_buf: &[u8], output_buf: &mut [u8]) -> Result<()> {
    use crate::error::ArrowError;
    Err(ArrowError::Ipc("The crate was compiled without IPC compression. Use `io_ipc_compression` to read compressed IPC.".to_string()))
}

#[cfg(not(feature = "io_ipc_compression"))]
pub fn decompress_zstd(input_buf: &[u8], output_buf: &mut [u8]) -> Result<()> {
    use crate::error::ArrowError;
    Err(ArrowError::Ipc("The crate was compiled without IPC compression. Use `io_ipc_compression` to read compressed IPC.".to_string()))
}
