use core::mem::*;
use core::ptr::*;
use core::time;
use alloc::vec;
use alloc::vec::Vec;

pub fn load_fn<T: Copy>(dst: &mut T, src: &[u8], offset: &mut usize) {
    unsafe {
        let sz = core::mem::size_of::<T>();
        core::ptr::copy_nonoverlapping(&src[*offset], dst as *mut _ as *mut u8, sz);
        *offset += sz;
    }
}

pub fn store_fn<T: Copy>(src: &T, dst: &mut [u8], offset: &mut usize) {
    unsafe {
        let sz = core::mem::size_of::<T>();
        core::ptr::copy_nonoverlapping(src as *const _ as *const u8, &mut dst[*offset], sz);
        *offset += sz;
    }
}
