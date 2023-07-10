use alloc::{collections::VecDeque, sync::Arc};

use crate::utils::error::AsyscallRet;

use super::{file::FileMeta, File, Mutex};

pub struct Socket {
    buf: Mutex<VecDeque<u8>>,
}

impl Socket {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            buf: Mutex::new(VecDeque::new()),
        })
    }
}

impl File for Socket {
    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        todo!()
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        todo!()
    }

    fn metadata(&self) -> &FileMeta {
        todo!()
    }
}
