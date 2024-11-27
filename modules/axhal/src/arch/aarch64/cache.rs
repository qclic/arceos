#![allow(unused)]

use core::{arch::asm, ptr::NonNull};

fn dcache_line_size() -> usize {
    unsafe {
        let result;
        asm!(
            "mrs    x8, CTR_EL0",
            "ubfm x8, x8, #16, #19",	// cache line size encoding
            "mov		{0}, #4",		// bytes per word
            "lsl		{0}, {0}, x8",	// actual cache line size""",
            out(reg) result);
        result
    }
}

/// Invalidate data cache
pub fn dcache_invalidate_range(addr: NonNull<u8>, size: usize) {
    let addr = addr.as_ptr() as usize;
    unsafe {
        let line_size = dcache_line_size();
        let start = addr & !(line_size - 1);
        let end = (addr + size + line_size - 1) & !(line_size - 1);

        for addr in (start..end).step_by(line_size) {
            asm!("dc ivac, {0}", in(reg) addr);
        }

        asm!("dsb sy; isb");
    }
}

/// Clean data cache
pub fn dcache_clean_range(addr: NonNull<u8>, size: usize) {
    let addr = addr.as_ptr() as usize;
    unsafe {
        let line_size = dcache_line_size();
        let start = addr & !(line_size - 1);
        let end = (addr + size + line_size - 1) & !(line_size - 1);

        for addr in (start..end).step_by(line_size) {
            asm!("dc cvac, {0}", in(reg) addr);
        }

        asm!("dsb sy; isb");
    }
}
