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

use std::fs::File;

use arrow2::error::Result;
use arrow2::io::ipc::read::{read_file_metadata, FileReader};
use arrow2::record_batch::RecordBatch;

fn read_batches(path: &str) -> Result<Vec<RecordBatch>> {
    let mut file = File::open(path)?;

    // read the files' metadata. At this point, we can distribute the read whatever we like.
    let metadata = read_file_metadata(&mut file)?;

    // Simplest way: use the reader, an iterator over batches.
    let reader = FileReader::new(&mut file, metadata);

    reader.collect()
}

fn main() -> Result<()> {
    use std::env;
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let batches = read_batches(file_path)?;
    println!("{:?}", batches);
    Ok(())
}
