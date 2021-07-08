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

use crate::record_batch::RecordBatch;
use crate::{array::PrimitiveArray, datatypes::DataType};

use super::{ArrowJsonBatch, ArrowJsonColumn};

pub fn from_record_batch(batch: &RecordBatch) -> ArrowJsonBatch {
    let mut json_batch = ArrowJsonBatch {
        count: batch.num_rows(),
        columns: Vec::with_capacity(batch.num_columns()),
    };

    for (col, field) in batch.columns().iter().zip(batch.schema().fields.iter()) {
        let json_col = match field.data_type() {
            DataType::Int8 => {
                let array = col.as_any().downcast_ref::<PrimitiveArray<i8>>().unwrap();

                let (validity, data) = array
                    .iter()
                    .map(|x| (x.is_some() as u8, x.copied().unwrap_or_default().into()))
                    .unzip();

                ArrowJsonColumn {
                    name: field.name().clone(),
                    count: col.len(),
                    validity: Some(validity),
                    data: Some(data),
                    offset: None,
                    children: None,
                }
            }
            _ => ArrowJsonColumn {
                name: field.name().clone(),
                count: col.len(),
                validity: None,
                data: None,
                offset: None,
                children: None,
            },
        };

        json_batch.columns.push(json_col);
    }

    json_batch
}
