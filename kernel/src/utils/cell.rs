pub struct SyncUnsafeCell<T>(core::cell::SyncUnsafeCell<T>);

impl<T> SyncUnsafeCell<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(core::cell::SyncUnsafeCell::new(value))
    }

    /// This method is unsafe.
    #[inline]
    pub fn get_unchecked_mut(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }

    #[inline]
    pub const fn get(&self) -> *mut T {
        self.0.get()
    }

    #[inline]
    pub const fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }

    #[inline]
    pub fn lock(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}
