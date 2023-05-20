use alloc::boxed::Box;
use async_trait::async_trait;
use log::{debug, warn};

use crate::{
    process,
    processor::SumGuard,
    sbi::console_getchar,
    sync::mutex::SpinNoIrqLock,
    utils::error::{SyscallErr, SyscallRet},
};

use super::file::{File, FileMeta};

pub struct Stdin;

pub struct Stdout;

#[async_trait]
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

    async fn read(&self, buf: &mut [u8]) -> SyscallRet {
        // // TODO: add read buf whose len is longer than 1
        // // Urgent!! Since async trait will allocate heap memory every
        // // time this function is invoked, we should decrease the times
        // // of invocation
        // assert_eq!(buf.len(), 1, "Only support len = 1 in sys_read!");

        let _sum_guard = SumGuard::new();
        let mut c: usize;
        let mut cnt = 0;
        loop {
            loop {
                c = console_getchar();
                // suspend_current_and_run_next();
                if c == 0 {
                    process::yield_now().await;
                    continue;
                } else {
                    break;
                }
            }
            let ch = c as u8;
            buf[cnt] = ch;
            cnt += 1;
            // debug!("stdin read a char {}, cnt {}, buf len {}", ch, cnt, buf.len());
            if cnt == buf.len() {
                break;
            }
        }
        Ok(buf.len() as isize)
        // unsafe {

        //     let buf = buf as *mut u8;
        //     buf.write_volatile(ch);
        // }
    }

    async fn write(&self, _: &[u8]) -> SyscallRet {
        // panic!("Cannot write to stdin")
        warn!("Cannot write to stdin");
        Err(SyscallErr::EBADF)
    }
}

const PRINT_LOCKED: bool = true;

static PRINT_MUTEX: SpinNoIrqLock<bool> = SpinNoIrqLock::new(false);

#[async_trait]
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

    async fn read(&self, _: &mut [u8]) -> SyscallRet {
        warn!("Cannot read stdout");
        Err(SyscallErr::EBADF)
    }

    async fn write(&self, buf: &[u8]) -> SyscallRet {
        let _sum_guard = SumGuard::new();
        // let buff = unsafe { core::slice::from_raw_parts(buf, len) };
        if PRINT_LOCKED {
            let _locked = PRINT_MUTEX.lock();
            print!("{}", core::str::from_utf8(buf).unwrap());
        } else {
            print!("{}", core::str::from_utf8(buf).unwrap());
        }
        Ok(buf.len() as isize)
    }
}
