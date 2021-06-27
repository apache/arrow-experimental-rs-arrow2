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

use flatbuffers::FlatBufferBuilder;

use crate::datatypes::*;

use super::super::{convert, gen};
use super::MetadataVersion;

/// Converts
pub fn schema_to_bytes(schema: &Schema, version: MetadataVersion) -> Vec<u8> {
    let mut fbb = FlatBufferBuilder::new();
    let schema = {
        let fb = convert::schema_to_fb_offset(&mut fbb, schema);
        fb.as_union_value()
    };

    let mut message = gen::Message::MessageBuilder::new(&mut fbb);
    message.add_version(version);
    message.add_header_type(gen::Message::MessageHeader::Schema);
    message.add_bodyLength(0);
    message.add_header(schema);
    // TODO: custom metadata
    let data = message.finish();
    fbb.finish(data, None);

    fbb.finished_data().to_vec()
}
