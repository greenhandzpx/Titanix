use alloc::boxed::Box;
use async_trait::async_trait;
use log::warn;

use crate::{
    process,
    processor::SumGuard,
    sbi::console_getchar,
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
        // TODO: add read buf whose len is longer than 1
        // Urgent!! Since async trait will allocate heap memory every
        // time this function is invoked, we should decrease the times
        // of invocation
        assert_eq!(buf.len(), 1, "Only support len = 1 in sys_read!");
        let mut c: usize;
        loop {
            c = console_getchar();
            process::yield_now().await;
            // suspend_current_and_run_next();
            if c == 0 {
                continue;
            } else {
                break;
            }
        }
        let ch = c as u8;
        let _sum_guard = SumGuard::new();
        buf[0] = ch;
        Ok(1)
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
        print!("{}", core::str::from_utf8(buf).unwrap());
        Ok(buf.len() as isize)
    }
}
