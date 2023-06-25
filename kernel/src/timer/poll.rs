use core::{future::Future, task::Poll};

use alloc::vec::Vec;
use log::{warn, debug};

use crate::{syscall::{PollFd, PollEvents}, processor::current_process, utils::error::{SyscallRet, SyscallErr}};

pub struct FilePollFuture {
    fds: Vec<PollFd>,
}

impl FilePollFuture {
    pub fn new(fds: Vec<PollFd>) -> Self {
        Self {
            fds
        }
    }
}

impl Future for FilePollFuture {
    type Output = SyscallRet;
    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {

        let waker = cx.waker().clone();
        let mut cnt = 0;
        let this = unsafe { self.get_unchecked_mut() };
        for fd in this.fds.iter_mut() {
                     if let Some(file) =
                current_process().inner_handler(|proc| proc.fd_table.get(fd.fd as usize))
            {
                if let Some(events) = PollEvents::from_bits(fd.events as u16) {
                    fd.revents = 0;
                    if events.contains(PollEvents::POLLIN) {
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
            debug!("[FilePollFuture]: poll ready, cnt {}", cnt);
            Poll::Ready(Ok(cnt as isize))
        // } else if !infinite_timeout && current_time_ms() >= expire_time {
        //     debug!("[sys_ppoll]: timeout!");
        //     return Ok(0);
        // } else {
        //     thread::yield_now().await;
        } else {
            debug!("[FilePollFuture]: no event happens");
            Poll::Pending
        }
    }
}