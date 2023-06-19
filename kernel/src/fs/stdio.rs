use alloc::boxed::Box;
use core::sync::atomic::{AtomicU8, Ordering};
use lazy_static::*;
use log::{debug, info, warn};

use crate::{
    process,
    processor::SumGuard,
    sbi::console_getchar,
    sync::mutex::SleepLock,
    utils::error::{AsyscallRet, GeneralRet, SyscallErr},
};

use super::file::{File, FileMeta};

pub struct Stdin {
    /// Temporarily save poll in data
    buf: AtomicU8,
}

impl Stdin {
    pub fn new() -> Self {
        Self {
            buf: AtomicU8::new(255),
        }
    }
}

pub struct Stdout;

// #[async_trait]
impl File for Stdin {
    fn readable(&self) -> bool {
        true
    }

    fn writable(&self) -> bool {
        false
    }

    fn metadata(&self) -> &FileMeta {
        todo!()
    }

    fn read<'a>(&'a self, buf: &'a mut [u8]) -> AsyscallRet {
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
                    c = console_getchar();
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
            Ok(buf.len() as isize)
        })
    }

    fn write(&self, _: &[u8]) -> AsyscallRet {
        // panic!("Cannot write to stdin")
        warn!("Cannot write to stdin");
        Box::pin(async move { Err(SyscallErr::EBADF) })
    }

    fn pollin(&self) -> GeneralRet<bool> {
        if self.buf.load(Ordering::Acquire) != 255 {
            return Ok(true);
        }
        let _sum_guard = SumGuard::new();
        let c = console_getchar();
        if c as i8 == -1 {
            return Ok(false);
        } else {
            self.buf.store(c as u8, Ordering::Release);
            return Ok(true);
        }
    }
}

const PRINT_LOCKED: bool = true;

lazy_static! {
    static ref PRINT_MUTEX: SleepLock<bool> = SleepLock::new(false);
}

// #[async_trait]

impl File for Stdout {
    fn readable(&self) -> bool {
        false
    }

    fn writable(&self) -> bool {
        true
    }

    fn metadata(&self) -> &FileMeta {
        todo!()
    }

    fn read(&self, _: &mut [u8]) -> AsyscallRet {
        warn!("Cannot read stdout");
        Box::pin(async move { Err(SyscallErr::EBADF) })
    }

    fn write<'a>(&'a self, buf: &'a [u8]) -> AsyscallRet {
        Box::pin(async move {
            // TODO: change to sleep lock
            let _sum_guard = SumGuard::new();
            // let buff = unsafe { core::slice::from_raw_parts(buf, len) };
            if PRINT_LOCKED {
                let _locked = PRINT_MUTEX.lock().await;
                // info!("[test]:{:?}", buf);
                if let Some(ch) = core::str::from_utf8(buf).ok() {
                    print!("{}", ch);
                } else {
                    warn!("cannot transfer to utf8: {:?}", buf);
                }
            } else {
                print!("{}", core::str::from_utf8(buf).unwrap());
            }
            Ok(buf.len() as isize)
        })
    }

    fn pollout(&self) -> GeneralRet<bool> {
        // TODO: change to sleep lock
        Ok(true)
    }
}
