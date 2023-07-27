mod qemu;
mod u740;

#[cfg(feature = "board_u740")]
pub use u740::*;

#[cfg(not(feature = "board_u740"))]
pub use qemu::*;
