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

use crate::{
    array::{list::ListValuesIter, Array, IterableListArray},
    bitmap::utils::{zip_validity, ZipValidity},
};

use super::FixedSizeListArray;

impl IterableListArray for FixedSizeListArray {
    fn value(&self, i: usize) -> Box<dyn Array> {
        FixedSizeListArray::value(self, i)
    }
}

type ValuesIter<'a> = ListValuesIter<'a, FixedSizeListArray>;
type ZipIter<'a> = ZipValidity<'a, Box<dyn Array>, ValuesIter<'a>>;

impl<'a> IntoIterator for &'a FixedSizeListArray {
    type Item = Option<Box<dyn Array>>;
    type IntoIter = ZipIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> FixedSizeListArray {
    /// Returns an iterator of `Option<Box<dyn Array>>`
    pub fn iter(&'a self) -> ZipIter<'a> {
        zip_validity(ListValuesIter::new(self), &self.validity)
    }

    /// Returns an iterator of `Box<dyn Array>`
    pub fn values_iter(&'a self) -> ValuesIter<'a> {
        ListValuesIter::new(self)
    }
}
