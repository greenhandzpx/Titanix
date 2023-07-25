use alloc::{
    boxed::Box,
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};
use log::info;

use crate::{
    mm::user_check::UserCheck,
    process::thread,
    processor::SumGuard,
    utils::error::{AsyscallRet, GeneralRet, SyscallErr},
};

use super::{
    file::FileMeta,
    inode::{InodeDevice, InodeMeta},
    File, Inode, Mutex, OpenFlags,
};
/// domain
pub const AF_INET: u32 = 2;

pub const SOCKETADDR_SIZE: usize = core::mem::size_of::<SocketAddr>();

pub const MAX_BUFFER_SIZE: u32 = 1 << 16 - 1;

pub const TCP_MSS: u32 = 32768;

pub struct Socket {
    pub inner: Mutex<SocketInner>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct SocketAddr {
    sa_family: u32,
    sa_data: [u8; 14],
}

impl SocketAddr {
    pub fn new() -> Self {
        Self {
            sa_family: AF_INET,
            sa_data: [0; 14],
        }
    }
}

pub struct SocketInner {
    pub buf: VecDeque<u8>,
    pub sendbuf_size: u32,
    pub recvbuf_size: u32,
    pub addr: SocketAddr,
}

impl SocketInner {
    pub fn new() -> Self {
        Self {
            buf: VecDeque::new(),
            sendbuf_size: MAX_BUFFER_SIZE,
            recvbuf_size: MAX_BUFFER_SIZE,
            addr: SocketAddr::new(),
        }
    }
}

impl Socket {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(SocketInner::new()),
        })
    }
}

pub struct SocketInode {
    metadata: InodeMeta,
}

impl SocketInode {
    pub fn new(socket: Arc<Socket>) -> Self {
        let metadata = InodeMeta::new(
            None,
            "this is a socket",
            super::InodeMode::FileSOCK,
            0,
            Some(InodeDevice::Socket(socket)),
        );
        Self { metadata }
    }
}

impl Inode for SocketInode {
    fn open(&self, this: Arc<dyn Inode>, flags: OpenFlags) -> GeneralRet<Arc<dyn File>> {
        let meta = FileMeta::new(flags);
        meta.inner.lock().inode = Some(this);
        Ok(Arc::new(SocketFile { meta }))
    }
    fn metadata(&self) -> &InodeMeta {
        &self.metadata
    }

    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = meta;
    }

    fn load_children_from_disk(&self, _this: Arc<dyn Inode>) {
        todo!()
    }

    fn delete_child(&self, _child_name: &str) {
        todo!()
    }
}

pub struct SocketFile {
    meta: FileMeta,
}

impl SocketFile {
    pub fn new() -> GeneralRet<Arc<dyn File>> {
        let socket = Socket::new();
        let socket_inode = Arc::new(SocketInode::new(socket));
        socket_inode.open(
            socket_inode.clone(),
            OpenFlags::CLOEXEC | OpenFlags::NONBLOCK,
        )
    }
}

impl File for SocketFile {
    fn readable(&self) -> bool {
        true
    }

    fn writable(&self) -> bool {
        true
    }

    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            self.meta.inner_get(|inner| {
                let inode = inner.inode.as_ref().unwrap();
                let socket = inode.metadata().device.as_ref().unwrap();
                match socket {
                    InodeDevice::Socket(socket) => {
                        let mut inner = socket.inner.lock();
                        let len = inner.buf.len().min(buf.len());
                        UserCheck::new().check_writable_slice(buf.as_ptr() as *mut u8, len)?;
                        inner
                            .buf
                            .drain(..len)
                            .zip(buf.into_iter())
                            .for_each(|(src, dst)| *dst = src);
                        Ok(len)
                    }
                    _ => {
                        info!("[Socket::read] inode device is not Socket");
                        Err(SyscallErr::EBADF)
                    }
                }
            })
        })
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            let socket = self.meta.inner_get(|inner| {
                let inode = inner.inode.as_ref().unwrap();
                let socket = inode.metadata().device.as_ref().unwrap();
                match socket {
                    InodeDevice::Socket(socket) => Ok(socket.clone()),
                    _ => {
                        info!("[Socket::read] inode device is not Socket");
                        Err(SyscallErr::EBADF)
                    }
                }
            })?;
            loop {
                if {
                    let mut inner = socket.inner.lock();
                    if inner.buf.len() + buf.len() > MAX_BUFFER_SIZE as usize {
                        true
                    } else {
                        buf.into_iter().for_each(|ch| inner.buf.push_back(*ch));
                        false
                    }
                } {
                    thread::yield_now().await;
                } else {
                    return Ok(buf.len());
                }
            }
        })
    }

    fn metadata(&self) -> &FileMeta {
        &self.meta
    }

    fn flags(&self) -> OpenFlags {
        self.meta.inner.lock().flags
    }
}

pub struct SocketBufManager {
    pub socketbuf_mgr: Mutex<BTreeMap<SocketAddr, Arc<dyn File>>>,
}

impl SocketBufManager {
    pub const fn new() -> Self {
        Self {
            socketbuf_mgr: Mutex::new(BTreeMap::new()),
        }
    }
}

pub static SOCKETBUF_MANAGER: SocketBufManager = SocketBufManager::new();
