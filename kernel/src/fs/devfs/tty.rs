use alloc::sync::Arc;

use crate::{
    driver::getchar,
    fs::{file::FileMetaInner, inode::InodeMeta, Inode, Mutex, OpenFlags},
    process::thread::spawn_kernel_thread,
    timer::POLL_QUEUE,
    utils::error::GeneralRet,
};
use alloc::boxed::Box;
use core::{
    sync::atomic::{AtomicU8, Ordering},
    task::Waker,
};

use crate::{process, processor::SumGuard, sync::mutex::SleepLock, utils::error::AsyscallRet};

use crate::fs::file::{File, FileMeta};
pub struct TtyInode {
    metadata: InodeMeta,
}

impl TtyInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str) -> Self {
        let metadata = InodeMeta::new(Some(parent), path, crate::fs::InodeMode::FileCHR, 0, None);
        Self { metadata }
    }
}

impl Inode for TtyInode {
    fn open(
        &self,
        this: alloc::sync::Arc<dyn Inode>,
        flags: crate::fs::OpenFlags,
    ) -> GeneralRet<Arc<dyn crate::fs::File>> {
        let file: Arc<dyn File> = Arc::new(TtyFile::new(this, flags));
        file.metadata().inner.lock().file = Some(Arc::downgrade(&file));
        Ok(file)
    }
    fn metadata(&self) -> &crate::fs::inode::InodeMeta {
        &self.metadata
    }

    fn set_metadata(&mut self, meta: crate::fs::inode::InodeMeta) {
        self.metadata = meta;
    }

    fn load_children_from_disk(&self, _this: alloc::sync::Arc<dyn Inode>) {
        panic!()
    }

    fn delete_child(&self, _child_name: &str) {
        panic!()
    }
}

const PRINT_LOCKED: bool = false;

// static PRINT_MUTEX: SleepLock<bool> = SleepLock::new(false);

static PRINT_MUTEX: SleepLock<bool> = SleepLock::new(false);
pub struct TtyFile {
    /// Temporarily save poll in data
    buf: AtomicU8,
    metadata: FileMeta,
}

impl TtyFile {
    pub fn new(this: Arc<dyn Inode>, flags: OpenFlags) -> Self {
        Self {
            buf: AtomicU8::new(255),
            metadata: FileMeta {
                inner: Mutex::new(FileMetaInner {
                    flags,
                    inode: Some(this),
                    mode: crate::fs::InodeMode::FileCHR,
                    pos: 0,
                    dirent_index: 0,
                    file: None,
                }),
                prw_lock: SleepLock::new(()),
            },
        }
    }
}

impl File for TtyFile {
    fn metadata(&self) -> &FileMeta {
        &self.metadata
    }

    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
        // println!("[TtyFile::read] read...");
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            let mut c: u8;
            let mut cnt = 0;
            loop {
                loop {
                    let self_buf = self.buf.load(Ordering::Acquire);
                    if self_buf != 255 {
                        self.buf.store(255, Ordering::Release);
                        c = self_buf;
                        break;
                    }
                    c = getchar();
                    // debug!("stdin read a char {}", c);
                    if c as i8 == -1 {
                        process::yield_now().await;
                    } else {
                        break;
                    }
                }
                let ch = c;
                buf[cnt] = ch;
                cnt += 1;
                if cnt == buf.len() {
                    break;
                }
            }
            // println!("[TtyFile::read] read finished");
            Ok(buf.len())
        })
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        // println!("[TtyFile::write] buf {:?}...", buf);
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            // let buff = unsafe { core::slice::from_raw_parts(buf, len) };
            if PRINT_LOCKED {
                let _locked = PRINT_MUTEX.lock().await;
                print!("{}", unsafe { core::str::from_utf8_unchecked(buf) });
            } else {
                print!("{}", unsafe { core::str::from_utf8_unchecked(buf) });
            }
            // println!("[TtyFile::write] write finished");
            Ok(buf.len())
        })
    }

    fn pollin(&self, waker: Option<Waker>) -> GeneralRet<bool> {
        // Ok(true)
        if self.buf.load(Ordering::Acquire) != 255 {
            return Ok(true);
        }
        let _sum_guard = SumGuard::new();
        let c = getchar();
        if c as i8 == -1 {
            if let Some(waker) = waker {
                POLL_QUEUE.register(
                    self.metadata().inner.lock().file.as_ref().unwrap().clone(),
                    waker,
                    true,
                )
            }
            return Ok(false);
        } else {
            self.buf.store(c as u8, Ordering::Release);
            return Ok(true);
        }
    }

    fn flags(&self) -> OpenFlags {
        self.metadata.inner.lock().flags
    }
}
