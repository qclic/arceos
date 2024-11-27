#![allow(unused)]

use core::ptr::NonNull;

/// Invalidate data cache
pub fn dcache_invalidate_range(_addr: NonNull<u8>, _size: usize) {
    unimplemented!();
}

/// Clean data cache
pub fn dcache_clean_range(_addr: NonNull<u8>, _size: usize) {
    unimplemented!();
}
