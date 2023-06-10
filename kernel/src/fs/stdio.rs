use core::sync::atomic::{AtomicU8, Ordering};

use alloc::boxed::Box;
use log::{debug, warn};

use crate::{
    process,
    processor::SumGuard,
    sbi::console_getchar,
    sync::mutex::SpinNoIrqLock,
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
            buf: AtomicU8::new(0),
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
        // // TODO: add read buf whose len is longer than 1
        // // Urgent!! Since async trait will allocate heap memory every
        // // time this function is invoked, we should decrease the times
        // // of invocation
        // assert_eq!(buf.len(), 1, "Only support len = 1 in sys_read!");
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            let mut c: u8;
            let mut cnt = 0;
            loop {
                loop {
                    let self_buf = self.buf.load(Ordering::Acquire);
                    if self_buf != 0 {
                        self.buf.store(0, Ordering::Release);
                        c = self_buf;
                        break;
                    }
                    c = console_getchar();
                    // suspend_current_and_run_next();
                    if c as i8 == -1 {
                        process::yield_now().await;
                        continue;
                    } else {
                        break;
                    }
                }
                let ch = c;
                buf[cnt] = ch;
                cnt += 1;
                // debug!("stdin read a char {}, cnt {}, buf len {}", ch, cnt, buf.len());
                if cnt == buf.len() {
                    break;
                }
            }
            Ok(buf.len() as isize)
        })

        // unsafe {

        //     let buf = buf as *mut u8;
        //     buf.write_volatile(ch);
        // }
    }

    fn write(&self, _: &[u8]) -> AsyscallRet {
        // panic!("Cannot write to stdin")
        warn!("Cannot write to stdin");
        Box::pin(async move { Err(SyscallErr::EBADF) })
    }

    fn pollin(&self) -> GeneralRet<bool> {
        if self.buf.load(Ordering::Acquire) != 0 {
            return Ok(true);
        }
        let _sum_guard = SumGuard::new();
        let c = console_getchar();
        if c == 0 {
            return Ok(false);
        } else {
            self.buf.store(c as u8, Ordering::Release);
            return Ok(true);
        }
    }
}

const PRINT_LOCKED: bool = true;

static PRINT_MUTEX: SpinNoIrqLock<bool> = SpinNoIrqLock::new(false);

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
                let _locked = PRINT_MUTEX.lock();
                print!("{}", core::str::from_utf8(buf).unwrap());
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
