use core::ops::Range;

use crate::{
    mm::user_check::UserCheck,
    processor::SumGuard,
    utils::error::{GeneralRet, SyscallErr},
};

// pub enum IoBuffer<'a> {
//     UserBuffer(UserBuffer),
//     Borrowed(&'a mut )
// }

pub struct UserBuffer {
    start_ptr: usize,
    len: usize,
    _sum_guard: SumGuard,
}

impl UserBuffer {
    pub fn new(start_ptr: usize, len: usize) -> Self {
        Self {
            start_ptr,
            len,
            _sum_guard: SumGuard::new(),
        }
    }

    pub fn get(&self, idx_range: Range<usize>) -> GeneralRet<&[u8]> {
        if idx_range.start > idx_range.end {
            return Err(SyscallErr::EFAULT);
        }
        if idx_range.end > self.len {
            return Err(SyscallErr::EFAULT);
        }
        let start_ptr = (self.start_ptr + idx_range.start) as *const u8;
        let len = idx_range.end - idx_range.start;
        UserCheck::new().check_readable_slice(start_ptr, len)?;
        Ok(unsafe { core::slice::from_raw_parts(start_ptr as *const u8, len) })
    }

    pub fn get_mut(&mut self, range: Range<usize>) -> &mut [u8] {
        todo!()
    }

    pub fn reinterpret<T>(&self) -> &T {
        todo!()
    }

    pub fn reinterpret_mut<T>(&mut self) -> &mut T {
        todo!()
    }
}
