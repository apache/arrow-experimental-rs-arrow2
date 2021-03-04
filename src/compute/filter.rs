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

use crate::array::growable::make_growable;
use crate::array::*;
use crate::record_batch::RecordBatch;
use crate::{bits::BitChunkIterator, error::Result};
use std::iter::Enumerate;

/// Function that can filter arbitrary arrays
pub type Filter<'a> = Box<dyn Fn(&dyn Array) -> Box<dyn Array> + 'a>;

/// Internal state of [SlicesIterator]
#[derive(Debug, PartialEq)]
enum State {
    // it is iterating over bits of a mask (`u64`, steps of size of 1 slot)
    Bits(u64),
    // it is iterating over chunks (steps of size of 64 slots)
    Chunks,
    // it is iterating over the remainding bits (steps of size of 1 slot)
    Remainder,
    // nothing more to iterate.
    Finish,
}

/// An iterator of `(usize, usize)` each representing an interval `[start,start+len[` whose
/// slots of a [BooleanArray] are true. Each interval corresponds to a contiguous region of memory to be
/// "taken" from an array to be filtered.
#[derive(Debug)]
pub(crate) struct SlicesIterator<'a> {
    iter: Enumerate<BitChunkIterator<'a>>,
    state: State,
    filter_count: usize,
    remainder_mask: u64,
    remainder_len: usize,
    chunk_len: usize,
    len: usize,
    start: usize,
    on_region: bool,
    current_chunk: usize,
    current_bit: usize,
}

impl<'a> SlicesIterator<'a> {
    pub(crate) fn new(filter: &'a BooleanArray) -> Self {
        let values = filter.values();

        let chunks = values.chunks();

        Self {
            iter: chunks.iter().enumerate(),
            state: State::Chunks,
            filter_count: values.len() - values.null_count(),
            remainder_len: chunks.remainder_len(),
            chunk_len: chunks.chunk_len(),
            remainder_mask: chunks.remainder(),
            len: 0,
            start: 0,
            on_region: false,
            current_chunk: 0,
            current_bit: 0,
        }
    }

    #[inline]
    fn current_start(&self) -> usize {
        self.current_chunk * 64 + self.current_bit
    }

    #[inline]
    fn iterate_bits(&mut self, mask: u64, max: usize) -> Option<(usize, usize)> {
        while self.current_bit < max {
            if (mask & (1 << self.current_bit)) != 0 {
                if !self.on_region {
                    self.start = self.current_start();
                    self.on_region = true;
                }
                self.len += 1;
            } else if self.on_region {
                let result = (self.start, self.len);
                self.len = 0;
                self.on_region = false;
                self.current_bit += 1;
                return Some(result);
            }
            self.current_bit += 1;
        }
        self.current_bit = 0;
        None
    }

    /// iterates over chunks.
    #[inline]
    fn iterate_chunks(&mut self) -> Option<(usize, usize)> {
        while let Some((i, mask)) = self.iter.next() {
            self.current_chunk = i;
            if mask == 0 {
                if self.on_region {
                    let result = (self.start, self.len);
                    self.len = 0;
                    self.on_region = false;
                    return Some(result);
                }
            } else if mask == 18446744073709551615u64 {
                // = !0u64
                if !self.on_region {
                    self.start = self.current_start();
                    self.on_region = true;
                }
                self.len += 64;
            } else {
                // there is a chunk that has a non-trivial mask => iterate over bits.
                self.state = State::Bits(mask);
                return None;
            }
        }
        // no more chunks => start iterating over the remainder
        self.current_chunk = self.chunk_len;
        self.state = State::Remainder;
        None
    }
}

impl<'a> Iterator for SlicesIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Chunks => {
                match self.iterate_chunks() {
                    None => {
                        // iterating over chunks does not yield any new slice => continue to the next
                        self.current_bit = 0;
                        self.next()
                    }
                    other => other,
                }
            }
            State::Bits(mask) => {
                match self.iterate_bits(mask, 64) {
                    None => {
                        // iterating over bits does not yield any new slice => change back
                        // to chunks and continue to the next
                        self.state = State::Chunks;
                        self.next()
                    }
                    other => other,
                }
            }
            State::Remainder => match self.iterate_bits(self.remainder_mask, self.remainder_len) {
                None => {
                    self.state = State::Finish;
                    if self.on_region {
                        Some((self.start, self.len))
                    } else {
                        None
                    }
                }
                other => other,
            },
            State::Finish => None,
        }
    }
}

/// Returns a prepared function optimized to filter multiple arrays.
/// Creating this function requires time, but using it is faster than [filter] when the
/// same filter needs to be applied to multiple arrays (e.g. a multi-column `RecordBatch`).
/// WARNING: the nulls of `filter` are ignored and the value on its slot is considered.
/// Therefore, it is considered undefined behavior to pass `filter` with null values.
pub fn build_filter(filter: &BooleanArray) -> Result<Filter> {
    let iter = SlicesIterator::new(filter);
    let filter_count = iter.filter_count;
    let chunks = iter.collect::<Vec<_>>();

    Ok(Box::new(move |array: &dyn Array| {
        let mut mutable = make_growable(&[array], false, filter_count);
        chunks
            .iter()
            .for_each(|(start, len)| mutable.extend(0, *start, *len));
        mutable.to_box()
    }))
}

/// Filters an [Array], returning elements matching the filter (i.e. where the values are true).
/// WARNING: the nulls of `filter` are ignored and the value on its slot is considered.
/// Therefore, it is considered undefined behavior to pass `filter` with null values.
/// # Example
/// ```rust
/// # use arrow2::array::{Int32Array, Primitive, BooleanArray};
/// # use arrow2::datatypes::DataType;
/// # use arrow2::error::Result;
/// # use arrow2::compute::filter::filter;
/// # fn main() -> Result<()> {
/// let array = Primitive::from_slice(&vec![5, 6, 7, 8, 9]).to(DataType::Int32);
/// let filter_array = BooleanArray::from_slice(&vec![true, false, false, true, false]);
/// let c = filter(&array, &filter_array)?;
/// let c = c.as_any().downcast_ref::<Int32Array>().unwrap();
/// assert_eq!(c, &Primitive::from_slice(vec![5, 8]).to(DataType::Int32));
/// # Ok(())
/// # }
/// ```
pub fn filter(array: &dyn Array, filter: &BooleanArray) -> Result<Box<dyn Array>> {
    let iter = SlicesIterator::new(filter);

    let mut mutable = make_growable(&[array], false, iter.filter_count);
    iter.for_each(|(start, len)| mutable.extend(0, start, len));
    Ok(mutable.to_box())
}

/// Returns a new [RecordBatch] with arrays containing only values matching the filter.
/// WARNING: the nulls of `filter` are ignored and the value on its slot is considered.
/// Therefore, it is considered undefined behavior to pass `filter` with null values.
pub fn filter_record_batch(
    record_batch: &RecordBatch,
    filter: &BooleanArray,
) -> Result<RecordBatch> {
    let filter = build_filter(filter)?;
    let filtered_arrays = record_batch
        .columns()
        .iter()
        .map(|a| filter(a.as_ref()).into())
        .collect();
    RecordBatch::try_new(record_batch.schema().clone(), filtered_arrays)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datatypes::DataType;

    #[test]
    fn test_filter_array_slice() {
        let a = Primitive::<i32>::from_slice(&[5, 6, 7, 8, 9])
            .to(DataType::Int32)
            .slice(1, 4);
        let b = BooleanArray::from_slice(vec![true, true, false, false, true]).slice(1, 4);
        let c = filter(&a, &b).unwrap();
        let d = c.as_ref().as_any().downcast_ref::<Int32Array>().unwrap();
        assert_eq!(2, d.len());
        assert_eq!(6, d.value(0));
        assert_eq!(9, d.value(1));
    }

    #[test]
    fn test_filter_array_low_density() {
        // this test exercises the all 0's branch of the filter algorithm
        let mut data_values = (1..=65).collect::<Vec<i32>>();
        let mut filter_values = (1..=65).map(|i| matches!(i % 65, 0)).collect::<Vec<bool>>();
        // set up two more values after the batch
        data_values.extend_from_slice(&[66, 67]);
        filter_values.extend_from_slice(&[false, true]);
        let a = Primitive::<i32>::from_slice(data_values).to(DataType::Int32);
        let b = BooleanArray::from_slice(filter_values);
        let c = filter(&a, &b).unwrap();
        let d = c.as_ref().as_any().downcast_ref::<Int32Array>().unwrap();
        assert_eq!(2, d.len());
        assert_eq!(65, d.value(0));
        assert_eq!(67, d.value(1));
    }

    #[test]
    fn test_filter_array_high_density() {
        // this test exercises the all 1's branch of the filter algorithm
        let mut data_values = (1..=65).map(Some).collect::<Vec<_>>();
        let mut filter_values = (1..=65)
            .map(|i| !matches!(i % 65, 0))
            .collect::<Vec<bool>>();
        // set second data value to null
        data_values[1] = None;
        // set up two more values after the batch
        data_values.extend_from_slice(&[Some(66), None, Some(67), None]);
        filter_values.extend_from_slice(&[false, true, true, true]);
        let a = Primitive::<i32>::from(data_values).to(DataType::Int32);
        let b = BooleanArray::from_slice(filter_values);
        let c = filter(&a, &b).unwrap();
        let d = c.as_ref().as_any().downcast_ref::<Int32Array>().unwrap();
        assert_eq!(67, d.len());
        assert_eq!(3, d.null_count());
        assert_eq!(1, d.value(0));
        assert_eq!(true, d.is_null(1));
        assert_eq!(64, d.value(63));
        assert_eq!(true, d.is_null(64));
        assert_eq!(67, d.value(65));
    }

    #[test]
    fn test_filter_string_array_simple() {
        let a = Utf8Array::<i32>::from_slice(vec!["hello", " ", "world", "!"]);
        let b = BooleanArray::from_slice(vec![true, false, true, false]);
        let c = filter(&a, &b).unwrap();
        let d = c
            .as_ref()
            .as_any()
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap();
        assert_eq!(2, d.len());
        assert_eq!("hello", d.value(0));
        assert_eq!("world", d.value(1));
    }

    #[test]
    fn test_filter_primative_array_with_null() {
        let a = Primitive::<i32>::from(vec![Some(5), None]).to(DataType::Int32);
        let b = BooleanArray::from_slice(vec![false, true]);
        let c = filter(&a, &b).unwrap();
        let d = c.as_ref().as_any().downcast_ref::<Int32Array>().unwrap();
        assert_eq!(1, d.len());
        assert_eq!(true, d.is_null(0));
    }

    #[test]
    fn test_filter_string_array_with_null() {
        let a = Utf8Array::<i32>::from(&vec![Some("hello"), None, Some("world"), None]);
        let b = BooleanArray::from_slice(vec![true, false, false, true]);
        let c = filter(&a, &b).unwrap();
        let d = c
            .as_ref()
            .as_any()
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap();
        assert_eq!(2, d.len());
        assert_eq!("hello", d.value(0));
        assert_eq!(false, d.is_null(0));
        assert_eq!(true, d.is_null(1));
    }

    #[test]
    fn test_filter_binary_array_with_null() {
        let data: Vec<Option<&[u8]>> = vec![Some(b"hello"), None, Some(b"world"), None];
        let a = BinaryArray::<i32>::from(&data);
        let b = BooleanArray::from_slice(vec![true, false, false, true]);
        let c = filter(&a, &b).unwrap();
        let d = c
            .as_ref()
            .as_any()
            .downcast_ref::<BinaryArray<i32>>()
            .unwrap();
        assert_eq!(2, d.len());
        assert_eq!(b"hello", d.value(0));
        assert_eq!(false, d.is_null(0));
        assert_eq!(true, d.is_null(1));
    }

    /*
    #[test]
    fn test_filter_dictionary_array() {
        let values = vec![Some("hello"), None, Some("world"), Some("!")];
        let a: Int8DictionaryArray = values.iter().copied().collect();
        let b = BooleanArray::from(vec![false, true, true, false]);
        let c = filter(&a, &b).unwrap();
        let d = c
            .as_ref()
            .as_any()
            .downcast_ref::<Int8DictionaryArray>()
            .unwrap();
        let value_array = d.values();
        let values = value_array.as_any().downcast_ref::<StringArray>().unwrap();
        // values are cloned in the filtered dictionary array
        assert_eq!(3, values.len());
        // but keys are filtered
        assert_eq!(2, d.len());
        assert_eq!(true, d.is_null(0));
        assert_eq!("world", values.value(d.keys().value(1) as usize));
    }

    #[test]
    fn test_filter_list_array() {
        let value_data = ArrayData::builder(DataType::Int32)
            .len(8)
            .add_buffer(Buffer::from_slice_ref(&[0, 1, 2, 3, 4, 5, 6, 7]))
            .build();

        let value_offsets = Buffer::from_slice_ref(&[0i64, 3, 6, 8, 8]);

        let list_data_type =
            DataType::LargeList(Box::new(Field::new("item", DataType::Int32, false)));
        let list_data = ArrayData::builder(list_data_type)
            .len(4)
            .add_buffer(value_offsets)
            .add_child_data(value_data)
            .null_bit_buffer(Buffer::from([0b00000111]))
            .build();

        //  a = [[0, 1, 2], [3, 4, 5], [6, 7], null]
        let a = LargeListArray::from(list_data);
        let b = BooleanArray::from(vec![false, true, false, true]);
        let result = filter(&a, &b).unwrap();

        // expected: [[3, 4, 5], null]
        let value_data = ArrayData::builder(DataType::Int32)
            .len(3)
            .add_buffer(Buffer::from_slice_ref(&[3, 4, 5]))
            .build();

        let value_offsets = Buffer::from_slice_ref(&[0i64, 3, 3]);

        let list_data_type =
            DataType::LargeList(Box::new(Field::new("item", DataType::Int32, false)));
        let expected = ArrayData::builder(list_data_type)
            .len(2)
            .add_buffer(value_offsets)
            .add_child_data(value_data)
            .null_bit_buffer(Buffer::from([0b00000001]))
            .build();

        assert_eq!(&make_array(expected), &result);
    }
    */

    #[test]
    fn test_slice_iterator_bits() {
        let filter_values = (0..64).map(|i| i == 1).collect::<Vec<bool>>();
        let filter = BooleanArray::from_slice(filter_values);

        let iter = SlicesIterator::new(&filter);
        let filter_count = iter.filter_count;
        let chunks = iter.collect::<Vec<_>>();

        assert_eq!(chunks, vec![(1, 1)]);
        assert_eq!(filter_count, 1);
    }

    #[test]
    fn test_slice_iterator_bits1() {
        let filter_values = (0..64).map(|i| i != 1).collect::<Vec<bool>>();
        let filter = BooleanArray::from_slice(filter_values);

        let iter = SlicesIterator::new(&filter);
        let filter_count = iter.filter_count;
        let chunks = iter.collect::<Vec<_>>();

        assert_eq!(chunks, vec![(0, 1), (2, 62)]);
        assert_eq!(filter_count, 64 - 1);
    }

    #[test]
    fn test_slice_iterator_chunk_and_bits() {
        let filter_values = (0..130).map(|i| i % 62 != 0).collect::<Vec<bool>>();
        let filter = BooleanArray::from_slice(filter_values);

        let iter = SlicesIterator::new(&filter);
        let filter_count = iter.filter_count;
        let chunks = iter.collect::<Vec<_>>();

        assert_eq!(chunks, vec![(1, 61), (63, 61), (125, 5)]);
        assert_eq!(filter_count, 61 + 61 + 5);
    }
}
