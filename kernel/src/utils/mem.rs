#[allow(unused)]
pub unsafe fn uninit_memory<T>() -> T {
    #[allow(clippy::uninit_assumed_init)]
    core::mem::MaybeUninit::uninit().assume_init()
}
