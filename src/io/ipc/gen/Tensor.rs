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

#![allow(dead_code)]
#![allow(unused_imports)]

use super::Schema::*;
use flatbuffers::EndianScalar;
use std::{cmp::Ordering, mem};
// automatically generated by the FlatBuffers compiler, do not modify

pub enum TensorDimOffset {}
#[derive(Copy, Clone, PartialEq)]

/// ----------------------------------------------------------------------
/// Data structures for dense tensors
/// Shape data for a single axis in a tensor
pub struct TensorDim<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TensorDim<'a> {
    type Inner = TensorDim<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> TensorDim<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TensorDim { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TensorDimArgs<'args>,
    ) -> flatbuffers::WIPOffset<TensorDim<'bldr>> {
        let mut builder = TensorDimBuilder::new(_fbb);
        builder.add_size_(args.size_);
        if let Some(x) = args.name {
            builder.add_name(x);
        }
        builder.finish()
    }

    pub const VT_SIZE_: flatbuffers::VOffsetT = 4;
    pub const VT_NAME: flatbuffers::VOffsetT = 6;

    /// Length of dimension
    #[inline]
    pub fn size_(&self) -> i64 {
        self._tab.get::<i64>(TensorDim::VT_SIZE_, Some(0)).unwrap()
    }
    /// Name of the dimension, optional
    #[inline]
    pub fn name(&self) -> Option<&'a str> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<&str>>(TensorDim::VT_NAME, None)
    }
}

impl flatbuffers::Verifiable for TensorDim<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<i64>(&"size_", Self::VT_SIZE_, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<&str>>(&"name", Self::VT_NAME, false)?
            .finish();
        Ok(())
    }
}
pub struct TensorDimArgs<'a> {
    pub size_: i64,
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for TensorDimArgs<'a> {
    #[inline]
    fn default() -> Self {
        TensorDimArgs {
            size_: 0,
            name: None,
        }
    }
}
pub struct TensorDimBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TensorDimBuilder<'a, 'b> {
    #[inline]
    pub fn add_size_(&mut self, size_: i64) {
        self.fbb_.push_slot::<i64>(TensorDim::VT_SIZE_, size_, 0);
    }
    #[inline]
    pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b str>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(TensorDim::VT_NAME, name);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TensorDimBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TensorDimBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<TensorDim<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for TensorDim<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("TensorDim");
        ds.field("size_", &self.size_());
        ds.field("name", &self.name());
        ds.finish()
    }
}
pub enum TensorOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Tensor<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Tensor<'a> {
    type Inner = Tensor<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> Tensor<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Tensor { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TensorArgs<'args>,
    ) -> flatbuffers::WIPOffset<Tensor<'bldr>> {
        let mut builder = TensorBuilder::new(_fbb);
        if let Some(x) = args.data {
            builder.add_data(x);
        }
        if let Some(x) = args.strides {
            builder.add_strides(x);
        }
        if let Some(x) = args.shape {
            builder.add_shape(x);
        }
        if let Some(x) = args.type_ {
            builder.add_type_(x);
        }
        builder.add_type_type(args.type_type);
        builder.finish()
    }

    pub const VT_TYPE_TYPE: flatbuffers::VOffsetT = 4;
    pub const VT_TYPE_: flatbuffers::VOffsetT = 6;
    pub const VT_SHAPE: flatbuffers::VOffsetT = 8;
    pub const VT_STRIDES: flatbuffers::VOffsetT = 10;
    pub const VT_DATA: flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn type_type(&self) -> Type {
        self._tab
            .get::<Type>(Tensor::VT_TYPE_TYPE, Some(Type::NONE))
            .unwrap()
    }
    /// The type of data contained in a value cell. Currently only fixed-width
    /// value types are supported, no strings or nested types
    #[inline]
    pub fn type_(&self) -> flatbuffers::Table<'a> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Tensor::VT_TYPE_, None)
            .unwrap()
    }
    /// The dimensions of the tensor, optionally named
    #[inline]
    pub fn shape(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<TensorDim<'a>>> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<TensorDim>>,
            >>(Tensor::VT_SHAPE, None)
            .unwrap()
    }
    /// Non-negative byte offsets to advance one value cell along each dimension
    /// If omitted, default to row-major order (C-like).
    #[inline]
    pub fn strides(&self) -> Option<flatbuffers::Vector<'a, i64>> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, i64>>>(
                Tensor::VT_STRIDES,
                None,
            )
    }
    /// The location and size of the tensor's data
    #[inline]
    pub fn data(&self) -> &'a Buffer {
        self._tab.get::<Buffer>(Tensor::VT_DATA, None).unwrap()
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_null(&self) -> Option<Null<'a>> {
        if self.type_type() == Type::Null {
            let u = self.type_();
            Some(Null::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_int(&self) -> Option<Int<'a>> {
        if self.type_type() == Type::Int {
            let u = self.type_();
            Some(Int::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_floating_point(&self) -> Option<FloatingPoint<'a>> {
        if self.type_type() == Type::FloatingPoint {
            let u = self.type_();
            Some(FloatingPoint::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_binary(&self) -> Option<Binary<'a>> {
        if self.type_type() == Type::Binary {
            let u = self.type_();
            Some(Binary::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_utf_8(&self) -> Option<Utf8<'a>> {
        if self.type_type() == Type::Utf8 {
            let u = self.type_();
            Some(Utf8::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_bool(&self) -> Option<Bool<'a>> {
        if self.type_type() == Type::Bool {
            let u = self.type_();
            Some(Bool::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_decimal(&self) -> Option<Decimal<'a>> {
        if self.type_type() == Type::Decimal {
            let u = self.type_();
            Some(Decimal::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_date(&self) -> Option<Date<'a>> {
        if self.type_type() == Type::Date {
            let u = self.type_();
            Some(Date::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_time(&self) -> Option<Time<'a>> {
        if self.type_type() == Type::Time {
            let u = self.type_();
            Some(Time::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_timestamp(&self) -> Option<Timestamp<'a>> {
        if self.type_type() == Type::Timestamp {
            let u = self.type_();
            Some(Timestamp::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_interval(&self) -> Option<Interval<'a>> {
        if self.type_type() == Type::Interval {
            let u = self.type_();
            Some(Interval::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_list(&self) -> Option<List<'a>> {
        if self.type_type() == Type::List {
            let u = self.type_();
            Some(List::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_struct_(&self) -> Option<Struct_<'a>> {
        if self.type_type() == Type::Struct_ {
            let u = self.type_();
            Some(Struct_::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_union(&self) -> Option<Union<'a>> {
        if self.type_type() == Type::Union {
            let u = self.type_();
            Some(Union::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_fixed_size_binary(&self) -> Option<FixedSizeBinary<'a>> {
        if self.type_type() == Type::FixedSizeBinary {
            let u = self.type_();
            Some(FixedSizeBinary::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_fixed_size_list(&self) -> Option<FixedSizeList<'a>> {
        if self.type_type() == Type::FixedSizeList {
            let u = self.type_();
            Some(FixedSizeList::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_map(&self) -> Option<Map<'a>> {
        if self.type_type() == Type::Map {
            let u = self.type_();
            Some(Map::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_duration(&self) -> Option<Duration<'a>> {
        if self.type_type() == Type::Duration {
            let u = self.type_();
            Some(Duration::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_large_binary(&self) -> Option<LargeBinary<'a>> {
        if self.type_type() == Type::LargeBinary {
            let u = self.type_();
            Some(LargeBinary::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_large_utf_8(&self) -> Option<LargeUtf8<'a>> {
        if self.type_type() == Type::LargeUtf8 {
            let u = self.type_();
            Some(LargeUtf8::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type_as_large_list(&self) -> Option<LargeList<'a>> {
        if self.type_type() == Type::LargeList {
            let u = self.type_();
            Some(LargeList::init_from_table(u))
        } else {
            None
        }
    }
}

impl flatbuffers::Verifiable for Tensor<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_union::<Type, _>(
                &"type_type",
                Self::VT_TYPE_TYPE,
                &"type_",
                Self::VT_TYPE_,
                true,
                |key, v, pos| match key {
                    Type::Null => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Null>>(
                        "Type::Null",
                        pos,
                    ),
                    Type::Int => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Int>>(
                        "Type::Int",
                        pos,
                    ),
                    Type::FloatingPoint => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<FloatingPoint>>(
                            "Type::FloatingPoint",
                            pos,
                        ),
                    Type::Binary => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Binary>>(
                        "Type::Binary",
                        pos,
                    ),
                    Type::Utf8 => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Utf8>>(
                        "Type::Utf8",
                        pos,
                    ),
                    Type::Bool => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Bool>>(
                        "Type::Bool",
                        pos,
                    ),
                    Type::Decimal => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<Decimal>>(
                            "Type::Decimal",
                            pos,
                        ),
                    Type::Date => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Date>>(
                        "Type::Date",
                        pos,
                    ),
                    Type::Time => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Time>>(
                        "Type::Time",
                        pos,
                    ),
                    Type::Timestamp => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<Timestamp>>(
                            "Type::Timestamp",
                            pos,
                        ),
                    Type::Interval => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<Interval>>(
                            "Type::Interval",
                            pos,
                        ),
                    Type::List => v.verify_union_variant::<flatbuffers::ForwardsUOffset<List>>(
                        "Type::List",
                        pos,
                    ),
                    Type::Struct_ => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<Struct_>>(
                            "Type::Struct_",
                            pos,
                        ),
                    Type::Union => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Union>>(
                        "Type::Union",
                        pos,
                    ),
                    Type::FixedSizeBinary => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<FixedSizeBinary>>(
                            "Type::FixedSizeBinary",
                            pos,
                        ),
                    Type::FixedSizeList => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<FixedSizeList>>(
                            "Type::FixedSizeList",
                            pos,
                        ),
                    Type::Map => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Map>>(
                        "Type::Map",
                        pos,
                    ),
                    Type::Duration => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<Duration>>(
                            "Type::Duration",
                            pos,
                        ),
                    Type::LargeBinary => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<LargeBinary>>(
                            "Type::LargeBinary",
                            pos,
                        ),
                    Type::LargeUtf8 => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<LargeUtf8>>(
                            "Type::LargeUtf8",
                            pos,
                        ),
                    Type::LargeList => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<LargeList>>(
                            "Type::LargeList",
                            pos,
                        ),
                    _ => Ok(()),
                },
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<TensorDim>>,
            >>(&"shape", Self::VT_SHAPE, true)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, i64>>>(
                &"strides",
                Self::VT_STRIDES,
                false,
            )?
            .visit_field::<Buffer>(&"data", Self::VT_DATA, true)?
            .finish();
        Ok(())
    }
}
pub struct TensorArgs<'a> {
    pub type_type: Type,
    pub type_: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
    pub shape: Option<
        flatbuffers::WIPOffset<
            flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<TensorDim<'a>>>,
        >,
    >,
    pub strides: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, i64>>>,
    pub data: Option<&'a Buffer>,
}
impl<'a> Default for TensorArgs<'a> {
    #[inline]
    fn default() -> Self {
        TensorArgs {
            type_type: Type::NONE,
            type_: None, // required field
            shape: None, // required field
            strides: None,
            data: None, // required field
        }
    }
}
pub struct TensorBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TensorBuilder<'a, 'b> {
    #[inline]
    pub fn add_type_type(&mut self, type_type: Type) {
        self.fbb_
            .push_slot::<Type>(Tensor::VT_TYPE_TYPE, type_type, Type::NONE);
    }
    #[inline]
    pub fn add_type_(&mut self, type_: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Tensor::VT_TYPE_, type_);
    }
    #[inline]
    pub fn add_shape(
        &mut self,
        shape: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<TensorDim<'b>>>,
        >,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Tensor::VT_SHAPE, shape);
    }
    #[inline]
    pub fn add_strides(&mut self, strides: flatbuffers::WIPOffset<flatbuffers::Vector<'b, i64>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Tensor::VT_STRIDES, strides);
    }
    #[inline]
    pub fn add_data(&mut self, data: &Buffer) {
        self.fbb_.push_slot_always::<&Buffer>(Tensor::VT_DATA, data);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TensorBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TensorBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Tensor<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_.required(o, Tensor::VT_TYPE_, "type_");
        self.fbb_.required(o, Tensor::VT_SHAPE, "shape");
        self.fbb_.required(o, Tensor::VT_DATA, "data");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for Tensor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Tensor");
        ds.field("type_type", &self.type_type());
        match self.type_type() {
            Type::Null => {
                if let Some(x) = self.type_as_null() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Int => {
                if let Some(x) = self.type_as_int() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::FloatingPoint => {
                if let Some(x) = self.type_as_floating_point() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Binary => {
                if let Some(x) = self.type_as_binary() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Utf8 => {
                if let Some(x) = self.type_as_utf_8() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Bool => {
                if let Some(x) = self.type_as_bool() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Decimal => {
                if let Some(x) = self.type_as_decimal() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Date => {
                if let Some(x) = self.type_as_date() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Time => {
                if let Some(x) = self.type_as_time() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Timestamp => {
                if let Some(x) = self.type_as_timestamp() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Interval => {
                if let Some(x) = self.type_as_interval() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::List => {
                if let Some(x) = self.type_as_list() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Struct_ => {
                if let Some(x) = self.type_as_struct_() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Union => {
                if let Some(x) = self.type_as_union() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::FixedSizeBinary => {
                if let Some(x) = self.type_as_fixed_size_binary() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::FixedSizeList => {
                if let Some(x) = self.type_as_fixed_size_list() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Map => {
                if let Some(x) = self.type_as_map() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::Duration => {
                if let Some(x) = self.type_as_duration() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::LargeBinary => {
                if let Some(x) = self.type_as_large_binary() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::LargeUtf8 => {
                if let Some(x) = self.type_as_large_utf_8() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            Type::LargeList => {
                if let Some(x) = self.type_as_large_list() {
                    ds.field("type_", &x)
                } else {
                    ds.field(
                        "type_",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            _ => {
                let x: Option<()> = None;
                ds.field("type_", &x)
            }
        };
        ds.field("shape", &self.shape());
        ds.field("strides", &self.strides());
        ds.field("data", &self.data());
        ds.finish()
    }
}
#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_tensor<'a>(buf: &'a [u8]) -> Tensor<'a> {
    unsafe { flatbuffers::root_unchecked::<Tensor<'a>>(buf) }
}

#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_tensor<'a>(buf: &'a [u8]) -> Tensor<'a> {
    unsafe { flatbuffers::size_prefixed_root_unchecked::<Tensor<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `Tensor`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_tensor_unchecked`.
pub fn root_as_tensor(buf: &[u8]) -> Result<Tensor, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<Tensor>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Tensor` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_tensor_unchecked`.
pub fn size_prefixed_root_as_tensor(buf: &[u8]) -> Result<Tensor, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<Tensor>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Tensor` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_tensor_unchecked`.
pub fn root_as_tensor_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<Tensor<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<Tensor<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Tensor` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_tensor_unchecked`.
pub fn size_prefixed_root_as_tensor_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<Tensor<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<Tensor<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Tensor and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Tensor`.
pub unsafe fn root_as_tensor_unchecked(buf: &[u8]) -> Tensor {
    flatbuffers::root_unchecked::<Tensor>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Tensor and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Tensor`.
pub unsafe fn size_prefixed_root_as_tensor_unchecked(buf: &[u8]) -> Tensor {
    flatbuffers::size_prefixed_root_unchecked::<Tensor>(buf)
}
#[inline]
pub fn finish_tensor_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Tensor<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_tensor_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Tensor<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
