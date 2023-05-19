//! an easy FAT32 fs
#![allow(missing_docs)]
#![allow(non_snake_case)]
#![no_std]

extern crate alloc;

mod block_dev;
mod block_cache;
#[macro_use]
mod util;
mod bpb;
mod disk_dentry;
mod fsinfo;
mod fat32_fs;
mod vfs;


pub const BLOCK_SZ: usize = 512;

pub use block_dev::{BlockDevice};