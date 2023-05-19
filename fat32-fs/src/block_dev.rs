use core::any::Any;
/// Trait for block devices
/// which reads and writes data in the unit of blocks
pub trait BlockDevice: Send + Sync + Any {
    ///Read data form block to buffer
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    ///Write data from buffer to block
    fn write_block(&self, block_id: usize, buf: &[u8]);
}

/// Trait for buffer cache devices
pub trait BufferCacheOperations {
    /// Do some modifications at the given offset of the given block
    fn write_buffer_at<T>(&self, block_no: usize, offset: usize, f: impl FnOnce(&mut T));
    // fn write_buffer_at(&self, block_no: usize, offset: usize);

    /// Read data at the given offset of the given block
    fn read_buffer_at<T>(&self, block_no: usize, offset: usize, f: impl FnOnce(&T));

    /// Sync the given block
    fn sync_buffer(&self, block_no: usize);

    /// Sync all buffers
    fn sync_all_buffers(&self);
}