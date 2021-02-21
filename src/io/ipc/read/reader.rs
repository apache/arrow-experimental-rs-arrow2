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

//! Arrow IPC File and Stream Readers
//!
//! The `FileReader` and `StreamReader` have similar interfaces,
//! however the `FileReader` expects a reader that supports `Seek`ing

use std::collections::{HashMap, VecDeque};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::sync::Arc;

use crate::array::*;
use crate::datatypes::{DataType, Field, Schema};
use crate::error::{ArrowError, Result};
use crate::record_batch::{RecordBatch, RecordBatchReader};

use super::super::{convert, gen};
use super::super::{ARROW_MAGIC, CONTINUATION_MARKER};
use super::deserialize::read;

type SchemaRef = Arc<Schema>;
type ArrayRef = Arc<dyn Array>;

/// Creates a record batch from binary data using the `ipc::RecordBatch` indexes and the `Schema`
pub fn read_record_batch<R: Read + Seek>(
    batch: gen::Message::RecordBatch,
    schema: Arc<Schema>,
    dictionaries: &[Option<ArrayRef>],
    reader: &mut R,
    block_offset: u64,
) -> Result<RecordBatch> {
    let buffers = batch
        .buffers()
        .ok_or_else(|| ArrowError::IPC("Unable to get buffers from IPC RecordBatch".to_string()))?;
    let mut buffers: VecDeque<&gen::Schema::Buffer> = buffers.into_iter().collect();
    let field_nodes = batch.nodes().ok_or_else(|| {
        ArrowError::IPC("Unable to get field nodes from IPC RecordBatch".to_string())
    })?;

    // This is a bug fix: we should have one dictionary per node, not schema field
    let dictionaries = dictionaries.into_iter().chain(std::iter::repeat(&None));

    let mut field_nodes = field_nodes
        .into_iter()
        .zip(dictionaries)
        .collect::<VecDeque<_>>();

    let arrays = schema
        .fields()
        .iter()
        .map(|field| {
            read(
                &mut field_nodes,
                field.data_type().clone(),
                &mut buffers,
                reader,
                block_offset,
            )
        })
        .collect::<std::io::Result<Vec<_>>>()?;

    RecordBatch::try_new(schema, arrays)
}

/// Read the dictionary from the buffer and provided metadata,
/// updating the `dictionaries_by_field` with the resulting dictionary
fn read_dictionary<R: Read + Seek>(
    batch: gen::Message::DictionaryBatch,
    schema: &Schema,
    dictionaries_by_field: &mut [Option<ArrayRef>],
    reader: &mut R,
    block_offset: u64,
) -> Result<()> {
    if batch.isDelta() {
        return Err(ArrowError::NotYetImplemented(
            "delta dictionary batches not supported".to_string(),
        ));
    }

    let id = batch.id();
    let fields_using_this_dictionary = schema.fields_with_dict_id(id);
    let first_field = fields_using_this_dictionary.first().ok_or_else(|| {
        ArrowError::InvalidArgumentError("dictionary id not found in schema".to_string())
    })?;

    // As the dictionary batch does not contain the type of the
    // values array, we need to retrieve this from the schema.
    // Get an array representing this dictionary's values.
    let dictionary_values: ArrayRef = match first_field.data_type() {
        DataType::Dictionary(_, ref value_type) => {
            // Make a fake schema for the dictionary batch.
            let schema = Schema {
                fields: vec![Field::new("", value_type.as_ref().clone(), false)],
                metadata: HashMap::new(),
            };
            // Read a single column
            let record_batch = read_record_batch(
                batch.data().unwrap(),
                Arc::new(schema),
                dictionaries_by_field,
                reader,
                block_offset,
            )?;
            Some(record_batch.column(0).clone())
        }
        _ => None,
    }
    .ok_or_else(|| {
        ArrowError::InvalidArgumentError("dictionary id not found in schema".to_string())
    })?;

    // for all fields with this dictionary id, update the dictionaries vector
    // in the reader. Note that a dictionary batch may be shared between many fields.
    // We don't currently record the isOrdered field. This could be general
    // attributes of arrays.
    for (i, field) in schema.fields().iter().enumerate() {
        if field.dict_id() == Some(id) {
            // Add (possibly multiple) array refs to the dictionaries array.
            dictionaries_by_field[i] = Some(dictionary_values.clone());
        }
    }

    Ok(())
}

/// Arrow File reader
pub struct FileReader<R: Read + Seek> {
    /// Buffered file reader that supports reading and seeking
    reader: BufReader<R>,

    /// The schema that is read from the file header
    schema: SchemaRef,

    /// The blocks in the file
    ///
    /// A block indicates the regions in the file to read to get data
    blocks: Vec<gen::File::Block>,

    /// A counter to keep track of the current block that should be read
    current_block: usize,

    /// The total number of blocks, which may contain record batches and other types
    total_blocks: usize,

    /// Optional dictionaries for each schema field.
    ///
    /// Dictionaries may be appended to in the streaming format.
    dictionaries_by_field: Vec<Option<ArrayRef>>,

    /// Metadata version
    metadata_version: gen::Schema::MetadataVersion,
}

impl<R: Read + Seek> FileReader<R> {
    /// Try to create a new file reader
    ///
    /// Returns errors if the file does not meet the Arrow Format header and footer
    /// requirements
    pub fn try_new(reader: R) -> Result<Self> {
        let mut reader = BufReader::new(reader);
        // check if header and footer contain correct magic bytes
        let mut magic_buffer: [u8; 6] = [0; 6];
        reader.read_exact(&mut magic_buffer)?;
        if magic_buffer != ARROW_MAGIC {
            return Err(ArrowError::IPC(
                "Arrow file does not contain correct header".to_string(),
            ));
        }
        reader.seek(SeekFrom::End(-6))?;
        reader.read_exact(&mut magic_buffer)?;
        if magic_buffer != ARROW_MAGIC {
            return Err(ArrowError::IPC(
                "Arrow file does not contain correct footer".to_string(),
            ));
        }
        // read footer length
        let mut footer_size: [u8; 4] = [0; 4];
        reader.seek(SeekFrom::End(-10))?;
        reader.read_exact(&mut footer_size)?;
        let footer_len = i32::from_le_bytes(footer_size);

        // read footer
        let mut footer_data = vec![0; footer_len as usize];
        reader.seek(SeekFrom::End(-10 - footer_len as i64))?;
        reader.read_exact(&mut footer_data)?;

        let footer = gen::File::root_as_footer(&footer_data[..])
            .map_err(|err| ArrowError::IPC(format!("Unable to get root as footer: {:?}", err)))?;

        let blocks = footer.recordBatches().ok_or_else(|| {
            ArrowError::IPC("Unable to get record batches from IPC Footer".to_string())
        })?;

        let total_blocks = blocks.len();

        let ipc_schema = footer.schema().unwrap();
        let schema = convert::fb_to_schema(ipc_schema);

        // Create an array of optional dictionary value arrays, one per field.
        let mut dictionaries_by_field = vec![None; schema.fields().len()];
        for block in footer.dictionaries().unwrap() {
            // read length from end of offset
            let mut message_size: [u8; 4] = [0; 4];
            reader.seek(SeekFrom::Start(block.offset() as u64))?;
            reader.read_exact(&mut message_size)?;
            let footer_len = if message_size == CONTINUATION_MARKER {
                reader.read_exact(&mut message_size)?;
                i32::from_le_bytes(message_size)
            } else {
                i32::from_le_bytes(message_size)
            };

            let mut block_data = vec![0; footer_len as usize];

            reader.read_exact(&mut block_data)?;

            let message = gen::Message::root_as_message(&block_data[..]).map_err(|err| {
                ArrowError::IPC(format!("Unable to get root as message: {:?}", err))
            })?;

            match message.header_type() {
                gen::Message::MessageHeader::DictionaryBatch => {
                    let block_offset = block.offset() as u64 + block.metaDataLength() as u64;
                    let batch = message.header_as_dictionary_batch().unwrap();
                    read_dictionary(
                        batch,
                        &schema,
                        &mut dictionaries_by_field,
                        &mut reader,
                        block_offset,
                    )?;
                }
                t => {
                    return Err(ArrowError::IPC(format!(
                        "Expecting DictionaryBatch in dictionary blocks, found {:?}.",
                        t
                    )));
                }
            };
        }

        Ok(Self {
            reader,
            schema: Arc::new(schema),
            blocks: blocks.to_vec(),
            current_block: 0,
            total_blocks,
            dictionaries_by_field,
            metadata_version: footer.version(),
        })
    }

    /// Return the number of batches in the file
    pub fn num_batches(&self) -> usize {
        self.total_blocks
    }

    /// Return the schema of the file
    pub fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }

    /// Read a specific record batch
    ///
    /// Sets the current block to the index, allowing random reads
    pub fn set_index(&mut self, index: usize) -> Result<()> {
        if index >= self.total_blocks {
            Err(ArrowError::IPC(format!(
                "Cannot set batch to index {} from {} total batches",
                index, self.total_blocks
            )))
        } else {
            self.current_block = index;
            Ok(())
        }
    }

    fn maybe_next(&mut self) -> Result<Option<RecordBatch>> {
        let block = self.blocks[self.current_block];
        self.current_block += 1;

        // read length
        self.reader.seek(SeekFrom::Start(block.offset() as u64))?;
        let mut meta_buf = [0; 4];
        self.reader.read_exact(&mut meta_buf)?;
        if meta_buf == CONTINUATION_MARKER {
            // continuation marker encountered, read message next
            self.reader.read_exact(&mut meta_buf)?;
        }
        let meta_len = i32::from_le_bytes(meta_buf);

        let mut block_data = vec![0; meta_len as usize];
        self.reader.read_exact(&mut block_data)?;

        let message = gen::Message::root_as_message(&block_data[..])
            .map_err(|err| ArrowError::IPC(format!("Unable to get root as footer: {:?}", err)))?;

        // some old test data's footer metadata is not set, so we account for that
        if self.metadata_version != gen::Schema::MetadataVersion::V1
            && message.version() != self.metadata_version
        {
            return Err(ArrowError::IPC(
                "Could not read IPC message as metadata versions mismatch".to_string(),
            ));
        }

        match message.header_type() {
            gen::Message::MessageHeader::Schema => Err(ArrowError::IPC(
                "Not expecting a schema when messages are read".to_string(),
            )),
            gen::Message::MessageHeader::RecordBatch => {
                let batch = message.header_as_record_batch().ok_or_else(|| {
                    ArrowError::IPC("Unable to read IPC message as record batch".to_string())
                })?;
                read_record_batch(
                    batch,
                    self.schema(),
                    &self.dictionaries_by_field,
                    &mut self.reader,
                    block.offset() as u64 + block.metaDataLength() as u64,
                )
                .map(Some)
            }
            gen::Message::MessageHeader::NONE => Ok(None),
            t => Err(ArrowError::IPC(format!(
                "Reading types other than record batches not yet supported, unable to read {:?}",
                t
            ))),
        }
    }
}

impl<R: Read + Seek> Iterator for FileReader<R> {
    type Item = Result<RecordBatch>;

    fn next(&mut self) -> Option<Self::Item> {
        // get current block
        if self.current_block < self.total_blocks {
            self.maybe_next().transpose()
        } else {
            None
        }
    }
}

impl<R: Read + Seek> RecordBatchReader for FileReader<R> {
    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }
}

/// Arrow Stream reader
pub struct StreamReader<R: Read> {
    /// Buffered stream reader
    reader: BufReader<R>,

    /// The schema that is read from the stream's first message
    schema: SchemaRef,

    /// Optional dictionaries for each schema field.
    ///
    /// Dictionaries may be appended to in the streaming format.
    dictionaries_by_field: Vec<Option<ArrayRef>>,

    /// An indicator of whether the stream is complete.
    ///
    /// This value is set to `true` the first time the reader's `next()` returns `None`.
    finished: bool,
}

impl<R: Read> StreamReader<R> {
    /// Try to create a new stream reader
    ///
    /// The first message in the stream is the schema, the reader will fail if it does not
    /// encounter a schema.
    /// To check if the reader is done, use `is_finished(self)`
    pub fn try_new(reader: R) -> Result<Self> {
        let mut reader = BufReader::new(reader);
        // determine metadata length
        let mut meta_size: [u8; 4] = [0; 4];
        reader.read_exact(&mut meta_size)?;
        let meta_len = {
            // If a continuation marker is encountered, skip over it and read
            // the size from the next four bytes.
            if meta_size == CONTINUATION_MARKER {
                reader.read_exact(&mut meta_size)?;
            }
            i32::from_le_bytes(meta_size)
        };

        let mut meta_buffer = vec![0; meta_len as usize];
        reader.read_exact(&mut meta_buffer)?;

        let message = gen::Message::root_as_message(meta_buffer.as_slice())
            .map_err(|err| ArrowError::IPC(format!("Unable to get root as message: {:?}", err)))?;
        // message header is a Schema, so read it
        let ipc_schema: gen::Schema::Schema = message
            .header_as_schema()
            .ok_or_else(|| ArrowError::IPC("Unable to read IPC message as schema".to_string()))?;
        let schema = convert::fb_to_schema(ipc_schema);

        // Create an array of optional dictionary value arrays, one per field.
        // todo: this is wrong for nested types, as there must be one dictionary per node, not per field
        let dictionaries_by_field = vec![None; schema.fields().len()];

        Ok(Self {
            reader,
            schema: Arc::new(schema),
            finished: false,
            dictionaries_by_field,
        })
    }

    /// Return the schema of the stream
    pub fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }

    /// Check if the stream is finished
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    fn maybe_next(&mut self) -> Result<Option<RecordBatch>> {
        if self.finished {
            return Ok(None);
        }
        // determine metadata length
        let mut meta_size: [u8; 4] = [0; 4];

        match self.reader.read_exact(&mut meta_size) {
            Ok(()) => (),
            Err(e) => {
                return if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    // Handle EOF without the "0xFFFFFFFF 0x00000000"
                    // valid according to:
                    // https://arrow.apache.org/docs/format/Columnar.html#ipc-streaming-format
                    self.finished = true;
                    Ok(None)
                } else {
                    Err(ArrowError::from(e))
                };
            }
        }

        let meta_len = {
            // If a continuation marker is encountered, skip over it and read
            // the size from the next four bytes.
            if meta_size == CONTINUATION_MARKER {
                self.reader.read_exact(&mut meta_size)?;
            }
            i32::from_le_bytes(meta_size)
        };

        if meta_len == 0 {
            // the stream has ended, mark the reader as finished
            self.finished = true;
            return Ok(None);
        }

        let mut meta_buffer = vec![0; meta_len as usize];
        self.reader.read_exact(&mut meta_buffer)?;

        let vecs = &meta_buffer.to_vec();
        let message = gen::Message::root_as_message(vecs)
            .map_err(|err| ArrowError::IPC(format!("Unable to get root as message: {:?}", err)))?;

        match message.header_type() {
            gen::Message::MessageHeader::Schema => Err(ArrowError::IPC(
                "Not expecting a schema when messages are read".to_string(),
            )),
            gen::Message::MessageHeader::RecordBatch => {
                let batch = message.header_as_record_batch().ok_or_else(|| {
                    ArrowError::IPC("Unable to read IPC message as record batch".to_string())
                })?;
                // read the block that makes up the record batch into a buffer
                let mut buf = vec![0; message.bodyLength() as usize];
                self.reader.read_exact(&mut buf)?;

                let mut reader = std::io::Cursor::new(buf);

                read_record_batch(
                    batch,
                    self.schema(),
                    &self.dictionaries_by_field,
                    &mut reader,
                    0,
                )
                .map(Some)
            }
            gen::Message::MessageHeader::DictionaryBatch => {
                let batch = message.header_as_dictionary_batch().ok_or_else(|| {
                    ArrowError::IPC("Unable to read IPC message as dictionary batch".to_string())
                })?;
                // read the block that makes up the dictionary batch into a buffer
                let mut buf = vec![0; message.bodyLength() as usize];
                self.reader.read_exact(&mut buf)?;

                let mut reader = std::io::Cursor::new(buf);

                read_dictionary(
                    batch,
                    &self.schema,
                    &mut self.dictionaries_by_field,
                    &mut reader,
                    0,
                )?;

                // read the next message until we encounter a RecordBatch
                self.maybe_next()
            }
            gen::Message::MessageHeader::NONE => Ok(None),
            t => Err(ArrowError::IPC(format!(
                "Reading types other than record batches not yet supported, unable to read {:?} ",
                t
            ))),
        }
    }
}

impl<R: Read> Iterator for StreamReader<R> {
    type Item = Result<RecordBatch>;

    fn next(&mut self) -> Option<Self::Item> {
        self.maybe_next().transpose()
    }
}

impl<R: Read> RecordBatchReader for StreamReader<R> {
    fn schema(&self) -> Arc<Schema> {
        self.schema.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::io::json_integration::{to_record_batch, ArrowJson};

    use super::*;

    use std::{convert::TryFrom, fs::File};

    use flate2::read::GzDecoder;

    fn test_file(version: &str, file_name: &str) {
        let testdata = crate::util::test_util::arrow_test_data();
        let file = File::open(format!(
            "{}/arrow-ipc-stream/integration/{}/{}.arrow_file",
            testdata, version, file_name
        ))
        .unwrap();

        let reader = FileReader::try_new(file).unwrap();

        // read expected JSON output
        let (schema, batches) = read_gzip_json(version, file_name);

        assert_eq!(&schema, reader.schema().as_ref());

        batches
            .iter()
            .zip(reader.map(|x| x.unwrap()))
            .for_each(|(lhs, rhs)| {
                assert_eq!(lhs, &rhs);
            });
    }

    #[test]
    fn read_generated_100_primitive() {
        test_file("1.0.0-littleendian", "generated_primitive");
    }

    #[test]
    fn read_generated_100_datetime() {
        test_file("1.0.0-littleendian", "generated_datetime");
    }

    #[test]
    fn read_generated_100_null_trivial() {
        test_file("1.0.0-littleendian", "generated_null_trivial");
    }

    #[test]
    fn read_generated_100_null() {
        test_file("1.0.0-littleendian", "generated_null");
    }

    #[test]
    fn read_generated_100_primitive_zerolength() {
        test_file("1.0.0-littleendian", "generated_primitive_zerolength");
    }

    #[test]
    fn read_generated_100_primitive_primitive_no_batches() {
        test_file("1.0.0-littleendian", "generated_primitive_no_batches");
    }

    #[test]
    fn read_generated_100_dictionary() {
        test_file("1.0.0-littleendian", "generated_dictionary");
    }

    #[test]
    fn read_generated_100_nested() {
        test_file("1.0.0-littleendian", "generated_nested");
    }

    #[test]
    fn read_generated_100_interval() {
        test_file("1.0.0-littleendian", "generated_interval");
    }

    /*
    #[test]
    fn read_generated_streams_100() {
        let testdata = crate::util::test_util::arrow_test_data();
        let version = "1.0.0-littleendian";
        // the test is repetitive, thus we can read all supported files at once
        let paths = vec![
            "generated_interval",
            "generated_datetime",
            "generated_dictionary",
            "generated_nested",
            "generated_null_trivial",
            "generated_null",
            "generated_primitive_no_batches",
            "generated_primitive_zerolength",
            "generated_primitive",
        ];
        paths.iter().for_each(|path| {
            let file = File::open(format!(
                "{}/arrow-ipc-stream/integration/{}/{}.stream",
                testdata, version, path
            ))
            .unwrap();

            let mut reader = StreamReader::try_new(file).unwrap();

            // read expected JSON output
            let arrow_json = read_gzip_json(version, path);
            assert!(arrow_json.equals_reader(&mut reader));
            // the next batch must be empty
            assert!(reader.next().is_none());
            // the stream must indicate that it's finished
            assert!(reader.is_finished());
        });
    }

    /*
    #[test]
    fn test_arrow_single_float_row() {
        let schema = Schema::new(vec![
            Field::new("a", DataType::Float32, false),
            Field::new("b", DataType::Float32, false),
            Field::new("c", DataType::Int32, false),
            Field::new("d", DataType::Int32, false),
        ]);
        let arrays = vec![
            Arc::new(Float32Array::from(vec![1.23])) as ArrayRef,
            Arc::new(Float32Array::from(vec![-6.50])) as ArrayRef,
            Arc::new(Int32Array::from(vec![2])) as ArrayRef,
            Arc::new(Int32Array::from(vec![1])) as ArrayRef,
        ];
        let batch = RecordBatch::try_new(Arc::new(schema.clone()), arrays).unwrap();
        // create stream writer
        let file = File::create("target/debug/testdata/float.stream").unwrap();
        let mut stream_writer =
            crate::ipc::writer::StreamWriter::try_new(file, &schema).unwrap();
        stream_writer.write(&batch).unwrap();
        stream_writer.finish().unwrap();

        // read stream back
        let file = File::open("target/debug/testdata/float.stream").unwrap();
        let reader = StreamReader::try_new(file).unwrap();

        reader.for_each(|batch| {
            let batch = batch.unwrap();
            assert!(
                batch
                    .column(0)
                    .as_any()
                    .downcast_ref::<Float32Array>()
                    .unwrap()
                    .value(0)
                    != 0.0
            );
            assert!(
                batch
                    .column(1)
                    .as_any()
                    .downcast_ref::<Float32Array>()
                    .unwrap()
                    .value(0)
                    != 0.0
            );
        })
    }
    */
    */

    /// Read gzipped JSON file
    fn read_gzip_json(version: &str, file_name: &str) -> (Schema, Vec<RecordBatch>) {
        let testdata = crate::util::test_util::arrow_test_data();
        let file = File::open(format!(
            "{}/arrow-ipc-stream/integration/{}/{}.json.gz",
            testdata, version, file_name
        ))
        .unwrap();
        let mut gz = GzDecoder::new(&file);
        let mut s = String::new();
        gz.read_to_string(&mut s).unwrap();
        // convert to Arrow JSON
        let arrow_json: ArrowJson = serde_json::from_str(&s).unwrap();

        let schema = serde_json::to_value(arrow_json.schema).unwrap();
        let schema = Schema::try_from(&schema).unwrap();

        // read dictionaries
        let mut dictionaries = HashMap::new();
        if let Some(dicts) = &arrow_json.dictionaries {
            for json_dict in dicts {
                // TODO: convert to a concrete Arrow type
                dictionaries.insert(json_dict.id, json_dict);
            }
        }

        let batches = arrow_json
            .batches
            .iter()
            .map(|batch| to_record_batch(&schema, batch, &dictionaries))
            .collect::<Result<Vec<_>>>()
            .unwrap();

        (schema, batches)
    }
}
