#![allow(dead_code, unused_imports)]
#[cfg(test)]
use std::alloc::{alloc, Layout};
use core::mem::{size_of};

static mut MEMORY_OFFSET: u32 = 0;

#[cfg(test)]
pub fn kalloc<T>() -> *mut T {
    return unsafe { alloc(Layout::new::<T>()) as *mut T };
}

#[cfg(not(test))]
pub fn kalloc<T>() -> *mut T {
    let bytes = size_of::<T>();
    unsafe {    
        let ptr = (0x4030_0000 + MEMORY_OFFSET) as *mut T;
        MEMORY_OFFSET += bytes as u32;
        return ptr;
    }
}