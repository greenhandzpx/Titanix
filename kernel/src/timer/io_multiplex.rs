use core::{future::Future, task::Poll};

use alloc::vec::Vec;
use log::{debug, warn};

use crate::{
    fs::ffi::FdSet,
    processor::{current_process, SumGuard},
    syscall::{PollEvents, PollFd},
    utils::error::{SyscallErr, SyscallRet},
};

pub struct IOMultiplexFuture {
    fds: Vec<PollFd>,
    user_format: IOMultiplexFormat,
}

pub enum IOMultiplexFormat {
    /// User addr, like `&mut [PollFd]`,
    /// used for `ppoll`
    PollFds(usize),
    /// User addr, like `&mut FdSet`,
    /// used for `pselect`
    FdSets(RawFdSetRWE),
}

pub struct RawFdSetRWE {
    /// User addr, like `&mut FdSet`
    pub read_fd_set_ptr: Option<usize>,
    /// User addr, like `&mut FdSet`
    pub write_fd_set_ptr: Option<usize>,
    /// User addr, like `&mut FdSet`
    pub except_fd_set_ptr: Option<usize>,
}

impl RawFdSetRWE {
    pub fn new(read_ptr: usize, write_ptr: usize, except_ptr: usize) -> Self {
        Self {
            read_fd_set_ptr: match read_ptr {
                0 => None,
                _ => Some(read_ptr),
            },
            write_fd_set_ptr: match write_ptr {
                0 => None,
                _ => Some(write_ptr),
            },
            except_fd_set_ptr: match except_ptr {
                0 => None,
                _ => Some(except_ptr),
            },
        }
    }
    pub fn update_by_fds_vec(&self, fds: &Vec<PollFd>) {
        for fd in fds.iter() {
            if let Some(fd_set_ptr) = self.read_fd_set_ptr {
                let fd_set = unsafe { &mut *(fd_set_ptr as *mut FdSet) };
                // fd_set.clear_all();
                if PollEvents::from_bits(fd.revents)
                    .unwrap()
                    .contains(PollEvents::POLLIN)
                {
                    fd_set.mark_fd(fd.fd as usize);
                    debug!(
                        "[update_by_fds_vec]: read fd set {:?}, fd set ptr {:#x}",
                        fd_set, fd_set_ptr
                    );
                }
            }
            if let Some(fd_set_ptr) = self.write_fd_set_ptr {
                let fd_set = unsafe { &mut *(fd_set_ptr as *mut FdSet) };
                // fd_set.clear_all();
                if PollEvents::from_bits(fd.revents)
                    .unwrap()
                    .contains(PollEvents::POLLOUT)
                {
                    fd_set.mark_fd(fd.fd as usize);
                    debug!("[update_by_fds_vec]: write fd set {:?}", fd_set);
                }
            }
            if let Some(fd_set_ptr) = self.except_fd_set_ptr {
                let fd_set = unsafe { &mut *(fd_set_ptr as *mut FdSet) };
                // fd_set.clear_all();
                if PollEvents::from_bits(fd.revents)
                    .unwrap()
                    .contains(PollEvents::POLLPRI)
                {
                    fd_set.mark_fd(fd.fd as usize);
                    debug!("[update_by_fds_vec]: except fd set {:?}", fd_set);
                }
            }
        }
        // if let Some(fd_set_ptr) = self.read_fd_set_ptr {
        //     log::debug!("[update_by_fds_vec] read fd set {:?}", unsafe {
        //         &*(fd_set_ptr as *mut FdSet)
        //     });
        // }
    }
}

impl IOMultiplexFuture {
    pub fn new(fds: Vec<PollFd>, user_format: IOMultiplexFormat) -> Self {
        Self { fds, user_format }
    }
}

impl Future for IOMultiplexFuture {
    type Output = SyscallRet;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        // if current_task().is_zombie() {
        //     return Poll::Ready(Ok(0));
        // }
        let waker = cx.waker().clone();
        let mut cnt = 0;
        let this = unsafe { self.get_unchecked_mut() };
        for fd in this.fds.iter_mut() {
            if let Some(file) =
                current_process().inner_handler(|proc| proc.fd_table.get(fd.fd as usize))
            {
                if let Some(events) = PollEvents::from_bits(fd.events) {
                    fd.revents = 0;
                    if events.contains(PollEvents::POLLIN) {
                        log::debug!("[IOMultiplexFuture::poll] pollin fd {}", fd.fd);
                        if let Some(res) = file.pollin(Some(waker.clone())).ok() {
                            if res {
                                fd.revents |= PollEvents::POLLIN.bits() as i16;
                                cnt += 1;
                            }
                        } else {
                            fd.revents |= PollEvents::POLLERR.bits() as i16;
                            cnt += 1;
                        }
                    }
                    if events.contains(PollEvents::POLLOUT) {
                        log::debug!("[IOMultiplexFuture::poll] pollout fd {}", fd.fd);
                        if let Some(res) = file.pollout(Some(waker.clone())).ok() {
                            if res {
                                fd.revents |= PollEvents::POLLOUT.bits() as i16;
                                cnt += 1;
                            }
                        } else {
                            fd.revents |= PollEvents::POLLERR.bits() as i16;
                            cnt += 1;
                        }
                    }
                } else {
                    warn!("Invalid events: {:#x}", fd.events);
                    // TODO: not sure
                    return Poll::Ready(Err(SyscallErr::EINVAL));
                }
            } else {
                warn!("No such file for fd {}", fd.fd);
                continue;
            }
        }
        if cnt > 0 {
            log::info!("[IOMultiplexFuture]: poll ready, cnt {}", cnt);
            // TODO: can we use user addr directly without copy overhead
            let _sum_guard = SumGuard::new();
            match &mut this.user_format {
                IOMultiplexFormat::PollFds(poll_fd_ptr) => {
                    let raw_fds: &mut [PollFd] = unsafe {
                        core::slice::from_raw_parts_mut(*poll_fd_ptr as *mut PollFd, this.fds.len())
                    };
                    raw_fds.copy_from_slice(&this.fds);
                }
                IOMultiplexFormat::FdSets(fd_set_rwe) => {
                    fd_set_rwe.update_by_fds_vec(&this.fds);
                }
            }
            Poll::Ready(Ok(cnt))
        } else {
            log::debug!("[IOMultiplexFuture]: no event happens");
            Poll::Pending
        }
    }
}
