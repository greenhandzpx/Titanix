use alloc::{boxed::Box, sync::Arc, vec::Vec};

use crate::{sync::mutex::SpinNoIrqLock, utils::cell::SyncUnsafeCell};

use super::{file::DefaultFile, inode::InodeMeta, File, FileMeta, Inode, SeekFrom};

pub struct KCoverage {
    inner: SpinNoIrqLock<KCoverageInner>,
}

struct KCoverageInner {
    started: bool,
    pcs: Vec<usize>,
    file: Option<Arc<dyn File>>,
}

pub fn init() {
    let dft_file = DefaultFile::new(FileMeta::new(super::InodeMode::FileCHR));
    let file: Arc<dyn File> = Arc::new(KCovFile::new(dft_file));
    let inode: Arc<dyn Inode> = Arc::new(KCovInode::new());
    inode.create_page_cache_if_needed();
    file.metadata().inner.lock().inode = Some(inode.clone());
    *K_COV_INODE.get_unchecked_mut() = Some(inode);
    K_COVERAGE.inner.lock().file = Some(file);
}

impl KCoverage {
    const fn new() -> Self {
        Self {
            inner: SpinNoIrqLock::new(KCoverageInner {
                started: false,
                pcs: Vec::new(),
                file: None,
            }),
        }
    }

    pub fn open(&self) -> Arc<dyn File> {
        self.inner.lock().file.as_ref().unwrap().clone()
    }

    pub fn start(&self) {
        let mut inner = self.inner.lock();
        inner.started = true;
    }

    pub fn add(&self, pc: usize) {
        let mut inner = self.inner.lock();
        if !inner.started {
            return;
        }

        let file = inner.file.as_ref().unwrap();
        let mut cnt_buf: [u8; 8] = [0; 8];
        file.sync_read(&mut cnt_buf).unwrap();
        let cnt = usize::from_ne_bytes(cnt_buf);
        file.seek(SeekFrom::Start(cnt * core::mem::size_of::<usize>()))
            .unwrap();
        let pc_buf = usize::to_ne_bytes(pc);
        file.sync_write(&pc_buf).unwrap();

        // inner.pcs.push(pc);
    }

    pub fn stop(&self) {
        let mut inner = self.inner.lock();
        inner.started = false;
    }
}

pub static K_COVERAGE: KCoverage = KCoverage::new();

pub static K_COV_INODE: SyncUnsafeCell<Option<Arc<dyn Inode>>> = SyncUnsafeCell::new(None);

pub struct KCovInode {
    metadata: InodeMeta,
}

impl KCovInode {
    fn new() -> Self {
        Self {
            metadata: InodeMeta::new(
                None,
                "/sys/kernel/debug/kcov",
                super::InodeMode::FileCHR,
                8,
                None,
            ),
        }
    }
}

impl Inode for KCovInode {
    fn open(&self, this: Arc<dyn Inode>) -> crate::utils::error::GeneralRet<Arc<dyn File>> {
        Ok(K_COVERAGE.open())
    }

    fn read<'a>(
        &'a self,
        _offset: usize,
        _buf: &'a mut [u8],
    ) -> crate::utils::error::AgeneralRet<usize> {
        Box::pin(async move { Ok(8) })
    }

    fn metadata(&self) -> &super::inode::InodeMeta {
        &self.metadata
    }

    fn set_metadata(&mut self, meta: super::inode::InodeMeta) {
        self.metadata = meta;
    }

    fn load_children_from_disk(&self, this: alloc::sync::Arc<dyn Inode>) {
        todo!()
    }

    fn delete_child(&self, child_name: &str) {
        todo!()
    }

    fn child_removeable(&self) -> crate::utils::error::GeneralRet<()> {
        todo!()
    }
}

struct KCovFile {
    file: DefaultFile,
}

impl KCovFile {
    fn new(file: DefaultFile) -> Self {
        Self { file }
    }
}

const KCOV_ENABLE: usize = 18446744071562617601;
const KCOV_DISABLE: usize = 1;

impl File for KCovFile {
    fn read<'a>(
        &'a self,
        buf: &'a mut [u8],
        flags: super::OpenFlags,
    ) -> crate::utils::error::AsyscallRet {
        self.file.read(buf, flags)
    }

    fn write<'a>(
        &'a self,
        buf: &'a [u8],
        flags: super::OpenFlags,
    ) -> crate::utils::error::AsyscallRet {
        self.file.write(buf, flags)
    }

    fn metadata(&self) -> &FileMeta {
        self.file.metadata()
    }

    fn ioctl(&self, command: usize, _value: usize) -> crate::utils::error::SyscallRet {
        match command {
            KCOV_ENABLE => {
                log::debug!("start kcov..");
                K_COVERAGE.start();
                log::debug!("start kcov finished");
            }
            KCOV_DISABLE => {
                log::debug!("stop kcov..");
                K_COVERAGE.stop();
            }
            _ => {
                panic!()
            }
        }
        Ok(0)
    }
}
