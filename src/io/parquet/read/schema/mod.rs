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

use crate::datatypes::Schema;
use crate::error::Result;

mod convert;
mod metadata;

use convert::is_nullable;
pub use convert::parquet_to_arrow_schema;
pub use metadata::read_schema_from_metadata;
pub use parquet2::metadata::{FileMetaData, KeyValue, SchemaDescriptor};
pub use parquet2::schema::types::ParquetType;

pub(crate) use convert::*;

pub fn get_schema(metadata: &FileMetaData) -> Result<Schema> {
    let schema = read_schema_from_metadata(metadata.key_value_metadata())?;
    Ok(schema).transpose().unwrap_or_else(|| {
        parquet_to_arrow_schema(metadata.schema(), metadata.key_value_metadata())
    })
}

pub fn is_type_nullable(type_: &ParquetType) -> bool {
    is_nullable(type_.get_basic_info())
}
