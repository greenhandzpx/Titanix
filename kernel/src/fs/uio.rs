#[repr(C)]
pub struct Iovec {
    /// user space buf starting address
    pub iov_base: usize,
    /// number of bytes to transfer
    pub iov_len: usize,
}

unsafe impl Send for Iovec {}
unsafe impl Sync for Iovec {}
