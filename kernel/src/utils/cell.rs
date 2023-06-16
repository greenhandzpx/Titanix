pub struct SyncUnsafeCell<T> (core::cell::SyncUnsafeCell<T>);

impl<T> SyncUnsafeCell<T> {

    #[inline]
    pub const fn new(value: T) -> Self {
        Self(core::cell::SyncUnsafeCell::new(value))
    }

    pub unsafe fn get_unchecked_mut(&self) -> &mut T {
        &mut *self.0.get()
    }
}