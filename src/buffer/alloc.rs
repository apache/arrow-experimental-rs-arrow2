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

//! Defines memory-related functions, such as allocate/deallocate/reallocate memory
//! regions, cache and allocation alignments.

use crate::types::NativeType;

use super::alignment::ALIGNMENT;

use std::mem::size_of;
use std::ptr::NonNull;
use std::{
    alloc::{handle_alloc_error, Layout},
    sync::atomic::AtomicIsize,
};

// If this number is not zero after all objects have been `drop`, there is a memory leak
pub static mut ALLOCATIONS: AtomicIsize = AtomicIsize::new(0);

#[inline]
unsafe fn dangling<T: NativeType>() -> NonNull<T> {
    NonNull::new_unchecked(ALIGNMENT as *mut T)
}

/// Allocates a cache-aligned memory region of `size` bytes with uninitialized values.
/// This is more performant than using [allocate_aligned_zeroed] when all bytes will have
/// an unknown or non-zero value and is semantically similar to `malloc`.
pub fn allocate_aligned<T: NativeType>(size: usize) -> NonNull<T> {
    unsafe {
        if size == 0 {
            dangling()
        } else {
            let size = size * size_of::<T>();
            ALLOCATIONS.fetch_add(size as isize, std::sync::atomic::Ordering::SeqCst);

            let layout = Layout::from_size_align_unchecked(size, ALIGNMENT);
            let raw_ptr = std::alloc::alloc(layout) as *mut T;
            NonNull::new(raw_ptr).unwrap_or_else(|| handle_alloc_error(layout))
        }
    }
}

/// Allocates a cache-aligned memory region of `size` bytes with `0` on all of them.
/// This is more performant than using [allocate_aligned] and setting all bytes to zero
/// and is semantically similar to `calloc`.
pub fn allocate_aligned_zeroed<T: NativeType>(size: usize) -> NonNull<T> {
    unsafe {
        if size == 0 {
            dangling()
        } else {
            let size = size * size_of::<T>();
            ALLOCATIONS.fetch_add(size as isize, std::sync::atomic::Ordering::SeqCst);

            let layout = Layout::from_size_align_unchecked(size, ALIGNMENT);
            let raw_ptr = std::alloc::alloc_zeroed(layout) as *mut T;
            NonNull::new(raw_ptr).unwrap_or_else(|| handle_alloc_error(layout))
        }
    }
}

/// Frees memory previously allocated by [`allocate_aligned_zeroed`] or [`allocate_aligned`].
/// # Safety
/// This function is sound iff:
///
/// * `ptr` was allocated by [`allocate_aligned_zeroed`] or [`allocate_aligned`]
/// * `size` must be the same size that was used to allocate that block of memory.
pub unsafe fn free_aligned<T: NativeType>(ptr: NonNull<T>, size: usize) {
    if size != 0 {
        let size = size * size_of::<T>();
        ALLOCATIONS.fetch_sub(size as isize, std::sync::atomic::Ordering::SeqCst);
        std::alloc::dealloc(
            ptr.as_ptr() as *mut u8,
            Layout::from_size_align_unchecked(size, ALIGNMENT),
        );
    }
}

/// # Safety
///
/// This function is unsafe because undefined behavior can result if the caller does not ensure all
/// of the following:
///
/// * ptr must be currently allocated via this allocator,
///
/// * new_size must be greater than zero.
///
/// * new_size, when rounded up to the nearest multiple of [ALIGNMENT], must not overflow (i.e.,
/// the rounded value must be less than usize::MAX).
pub unsafe fn reallocate<T: NativeType>(
    ptr: NonNull<T>,
    old_size: usize,
    new_size: usize,
) -> NonNull<T> {
    if old_size == 0 {
        return allocate_aligned(new_size);
    }

    if new_size == 0 {
        free_aligned(ptr, old_size);
        return dangling();
    }
    let old_size = old_size * size_of::<T>();
    let new_size = new_size * size_of::<T>();

    ALLOCATIONS.fetch_add(
        new_size as isize - old_size as isize,
        std::sync::atomic::Ordering::SeqCst,
    );
    let raw_ptr = std::alloc::realloc(
        ptr.as_ptr() as *mut u8,
        Layout::from_size_align_unchecked(old_size, ALIGNMENT),
        new_size,
    ) as *mut T;
    NonNull::new(raw_ptr).unwrap_or_else(|| {
        handle_alloc_error(Layout::from_size_align_unchecked(new_size, ALIGNMENT))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        for _ in 0..10 {
            let p = allocate_aligned::<u32>(1024);
            // make sure this is 64-byte aligned
            assert_eq!(0, (p.as_ptr() as usize) % 64);
            unsafe { free_aligned(p, 1024) };
        }
    }

    #[test]
    fn test_reallocate() {
        let size = 16;
        let ptr = allocate_aligned::<i32>(size);
        let new_size = 32;
        let ptr = unsafe { reallocate(ptr, size, 32) };
        unsafe { free_aligned(ptr, new_size) };
        assert_eq!(
            unsafe { ALLOCATIONS.load(std::sync::atomic::Ordering::SeqCst) },
            0
        );
    }
}
