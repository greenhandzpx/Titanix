#![allow(unused)]
use core::{mem::size_of, ptr::copy_nonoverlapping};

use crate::{
    driver::BlockDevice,
    fs::{
        file::{FileMeta, FileMetaInner},
        inode::{DevWrapper, InodeDevice, InodeMeta},
        File, Inode, Mutex, OpenFlags, SeekFrom,
    },
    mm::user_check::UserCheck,
    processor::current_process,
    sync::mutex::SleepLock,
    utils::error::{AsyscallRet, GeneralRet, SyscallRet},
};
use alloc::{boxed::Box, sync::Arc};
use log::debug;

pub struct LoopInode {
    metadata: InodeMeta,
    dev_id: usize,
    lo_meta: Arc<Mutex<LoopMeta>>,
    // dev_fs: SyncUnsafeCell<Option<Arc<DevFs>>>,
}

impl LoopInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str, dev_id: usize) -> Self {
        let metadata = InodeMeta::new(
            Some(parent),
            path,
            crate::fs::InodeMode::FileBLK,
            0,
            Some(InodeDevice::LoopDevice(Arc::new(
                FileBlockDeviceWrapper::new(),
            ))),
        );
        Self {
            metadata,
            dev_id,
            lo_meta: Arc::new(Mutex::new(LoopMeta {
                info: None,
                inner_fd: None,
            })),
        }
    }
}

impl Inode for LoopInode {
    fn open(&self, this: Arc<dyn Inode>) -> GeneralRet<Arc<dyn File>> {
        Ok(Arc::new(LoopFile {
            meta: FileMeta {
                inner: Mutex::new(FileMetaInner {
                    inode: Some(this),
                    mode: self.metadata.mode,
                    pos: 0,
                    dirent_index: 0,
                    file: None,
                }),
                prw_lock: SleepLock::new(()),
                // path: self.metadata().path.clone(),
            },
            lo_meta: self.lo_meta.clone(),
        }))
    }
    fn set_metadata(&mut self, meta: InodeMeta) {
        self.metadata = meta;
    }
    fn metadata(&self) -> &InodeMeta {
        &self.metadata
    }
    fn load_children_from_disk(&self, _this: Arc<dyn Inode>) {
        panic!("Unsupported operation")
    }
    fn delete_child(&self, _child_name: &str) {
        panic!("Unsupported operation delete")
    }
    fn child_removeable(&self) -> GeneralRet<()> {
        Err(crate::utils::error::SyscallErr::EPERM)
    }
}

const LOOP_SET_FD: usize = 0x4C00;
const LOOP_CLR_FD: usize = 0x4C01;
const LOOP_SET_STATUS: usize = 0x4C02;
const LOOP_GET_STATUS: usize = 0x4C03;
const LOOP_SET_STATUS64: usize = 0x4C04;
const LOOP_GET_STATUS64: usize = 0x4C05;
const LOOP_CHANGE_FD: usize = 0x4C06;
const LOOP_SET_CAPACITY: usize = 0x4C07;
const LOOP_SET_DIRECT_IO: usize = 0x4C08;
const LOOP_SET_BLOCK_SIZE: usize = 0x4C09;
const LOOP_CONFIGURE: usize = 0x4C0A;

const LO_NAME_SIZE: usize = 64;
const LO_KEY_SIZE: usize = 32;

const LO_CRYPT_NONE: u32 = 0;

#[repr(C)]
struct LoopInfo64 {
    lo_device: u64,
    lo_inode: u64,
    lo_rdevice: u64,
    lo_offset: u64,
    lo_sizelimit: u64,
    lo_number: u32,
    lo_encrypt_type: u32,
    lo_encrypt_key_size: u32,
    lo_flags: u32,
    lo_file_name: [u8; LO_NAME_SIZE],
    lo_crypt_name: [u8; LO_NAME_SIZE],
    lo_encrypt_key: [u8; LO_KEY_SIZE],
    lo_init: [u64; 2],
}

struct LoopMeta {
    info: Option<LoopInfo64>,
    inner_fd: Option<isize>,
}

pub struct LoopFile {
    meta: FileMeta,
    lo_meta: Arc<Mutex<LoopMeta>>,
}

pub struct FileBlockDeviceWrapper {
    file: Mutex<Option<Arc<dyn File>>>,
}

impl FileBlockDeviceWrapper {
    pub fn new() -> Self {
        Self {
            file: Mutex::new(None),
        }
    }
    fn set_file(&self, file: Arc<dyn File>) {
        *self.file.lock() = Some(file.clone());
    }
}

const BLOCK_SIZE: usize = 512;

impl BlockDevice for FileBlockDeviceWrapper {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        let file = self.file.lock().as_ref().unwrap().clone();
        file.seek(SeekFrom::Start(block_id * BLOCK_SIZE)).unwrap();
        file.sync_read(buf).unwrap();
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) {
        let file = self.file.lock().as_ref().unwrap().clone();
        file.seek(SeekFrom::Start(block_id * BLOCK_SIZE)).unwrap();
        file.sync_write(buf).unwrap();
    }
}

// #[async_trait]
impl File for LoopFile {
    fn metadata(&self) -> &FileMeta {
        &self.meta
    }
    fn read<'a>(&'a self, _buf: &'a mut [u8], _flags: OpenFlags) -> AsyscallRet {
        Box::pin(async move { Ok(0) })
    }
    fn write<'a>(&'a self, buf: &'a [u8], _flags: OpenFlags) -> AsyscallRet {
        Box::pin(async move { Ok(buf.len()) })
    }
    fn ioctl(&self, command: usize, value: usize) -> SyscallRet {
        match command {
            LOOP_GET_STATUS64 => {
                if self.lo_meta.lock().info.is_none() {
                    return Err(crate::utils::error::SyscallErr::ENXIO);
                }
                UserCheck::new().check_writable_slice(value as *mut u8, size_of::<LoopInfo64>())?;
                unsafe {
                    copy_nonoverlapping(
                        self.lo_meta.lock().info.as_ref().unwrap(),
                        value as *mut LoopInfo64,
                        1,
                    );
                }
            }
            LOOP_SET_STATUS64 => {
                UserCheck::new()
                    .check_readable_slice(value as *const u8, size_of::<LoopInfo64>())?;
                if self.lo_meta.lock().info.is_none() {
                    (*self.lo_meta.lock()).info = Some(LoopInfo64 {
                        lo_device: 0,
                        lo_inode: 0,
                        lo_rdevice: 0,
                        lo_offset: 0,
                        lo_sizelimit: 0,
                        lo_number: 0,
                        lo_encrypt_type: 0,
                        lo_encrypt_key_size: 0,
                        lo_flags: 0,
                        lo_file_name: [0; LO_NAME_SIZE],
                        lo_crypt_name: [0; LO_NAME_SIZE],
                        lo_encrypt_key: [0; LO_KEY_SIZE],
                        lo_init: [0, 0],
                    });
                }
                unsafe {
                    copy_nonoverlapping(
                        value as *mut LoopInfo64,
                        self.lo_meta.lock().info.as_mut().unwrap(),
                        1,
                    );
                }
            }
            LOOP_SET_FD => {
                (*self.lo_meta.lock()).inner_fd = Some(value as isize);
                let file = current_process()
                    .inner_handler(|proc| proc.fd_table.get(value).unwrap())
                    .file;
                let inner_lock = self.meta.inner.lock();
                let inode_device = inner_lock
                    .inode
                    .as_ref()
                    .unwrap()
                    .metadata()
                    .device
                    .as_ref()
                    .unwrap();
                match inode_device {
                    InodeDevice::LoopDevice(lo_device) => {
                        lo_device.set_file(file);
                    }
                    _ => panic!(),
                }
            }
            LOOP_CLR_FD => {
                (*self.lo_meta.lock()).inner_fd = None;
            }
            _ => {
                log::warn!("unsupported loopfile ioctl {:x}", command);
            }
        }
        Ok(0)
    }
}
