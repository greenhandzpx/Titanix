use alloc::{sync::Arc, vec::Vec};

use crate::{
    config::process::INITPROC_PID,
    driver::{getchar, CHAR_DEVICE},
    fs::{file::FileMetaInner, inode::InodeMeta, resolve_path, Inode, Mutex, OpenFlags, AT_FDCWD},
    mm::user_check::UserCheck,
    process::{PROCESS_GROUP_MANAGER, PROCESS_MANAGER},
    signal::SIGINT,
    stack_trace,
    sync::mutex::SpinLock,
    timer::POLL_QUEUE,
    utils::{
        cell::SyncUnsafeCell,
        error::{GeneralRet, SyscallRet},
    },
};
use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU8, Ordering},
    task::{Poll, Waker},
};

use crate::{process, processor::SumGuard, sync::mutex::SleepLock, utils::error::AsyscallRet};

use crate::fs::file::{File, FileMeta};
pub struct TtyInode {
    metadata: InodeMeta,
}

impl TtyInode {
    pub fn new(parent: Arc<dyn Inode>, path: &str) -> Self {
        stack_trace!();
        let metadata = InodeMeta::new(Some(parent), path, crate::fs::InodeMode::FileCHR, 0, None);
        Self { metadata }
    }
}

impl Inode for TtyInode {
    fn open(&self, _this: alloc::sync::Arc<dyn Inode>) -> GeneralRet<Arc<dyn crate::fs::File>> {
        stack_trace!();
        Ok(TTY.get_unchecked_mut().as_ref().unwrap().clone())
        // let file: Arc<dyn File> = Arc::new(TtyFile::new());
        // file.metadata().inner.lock().file = Some(Arc::downgrade(&file));
        // Ok(file)
    }
    fn metadata(&self) -> &crate::fs::inode::InodeMeta {
        stack_trace!();
        &self.metadata
    }

    fn set_metadata(&mut self, meta: crate::fs::inode::InodeMeta) {
        stack_trace!();
        self.metadata = meta;
    }

    fn load_children_from_disk(&self, _this: alloc::sync::Arc<dyn Inode>) {
        stack_trace!();
        panic!()
    }

    fn delete_child(&self, _child_name: &str) {
        stack_trace!();
        panic!()
    }
    fn child_removeable(&self) -> GeneralRet<()> {
        stack_trace!();
        Err(crate::utils::error::SyscallErr::EPERM)
    }
}

const PRINT_LOCKED: bool = false;

static PRINT_MUTEX: SleepLock<bool> = SleepLock::new(false);

type Pid = u32;

// For struct termios
/// Gets the current serial port settings.
const TCGETS: usize = 0x5401;
/// Sets the serial port settings immediately.
const TCSETS: usize = 0x5402;
/// Sets the serial port settings after allowing the input and output buffers to drain/empty.
const TCSETSW: usize = 0x5403;
/// Sets the serial port settings after flushing the input and output buffers.
const TCSETSF: usize = 0x5404;
/// For struct termio
/// Gets the current serial port settings.
const TCGETA: usize = 0x5405;
/// Sets the serial port settings immediately.
#[allow(unused)]
const TCSETA: usize = 0x5406;
/// Sets the serial port settings after allowing the input and output buffers to drain/empty.
#[allow(unused)]
const TCSETAW: usize = 0x5407;
/// Sets the serial port settings after flushing the input and output buffers.
#[allow(unused)]
const TCSETAF: usize = 0x5408;
/// If the terminal is using asynchronous serial data transmission, and arg is zero, then send a break (a stream of zero bits) for between 0.25 and 0.5 seconds.
const TCSBRK: usize = 0x5409;
/// Get the process group ID of the foreground process group on this terminal.
const TIOCGPGRP: usize = 0x540F;
/// Set the foreground process group ID of this terminal.
const TIOCSPGRP: usize = 0x5410;
/// Get window size.
const TIOCGWINSZ: usize = 0x5413;
/// Set window size.
const TIOCSWINSZ: usize = 0x5414;
/// Non-cloexec
#[allow(unused)]
const FIONCLEX: usize = 0x5450;
/// Cloexec
#[allow(unused)]
const FIOCLEX: usize = 0x5451;
/// rustc using pipe and ioctl pipe file with this request id
/// for non-blocking/blocking IO control setting
#[allow(unused)]
const FIONBIO: usize = 0x5421;
/// Read time
#[allow(unused)]
const RTC_RD_TIME: usize = 0x80247009;

#[repr(C)]
#[derive(Clone, Copy)]
struct WinSize {
    ws_row: u16,
    ws_col: u16,
    xpixel: u16,
    ypixel: u16,
}

impl WinSize {
    fn new() -> Self {
        stack_trace!();
        Self {
            // ws_row: 67,
            // ws_col: 270,
            ws_row: 67,
            ws_col: 120,
            xpixel: 0,
            ypixel: 0,
        }
    }
}

pub fn init() {
    stack_trace!();
    // let tty_inode = resolve_path(AT_FDCWD, "/dev/tty", OpenFlags::empty());
    let tty_inode = <dyn Inode>::lookup_from_root("/dev/tty")
        .unwrap()
        .0
        .unwrap();
    // let tty_file = tty_inode.open(tty_inode.clone()).unwrap();
    let tty_file = Arc::new(TtyFile::new());
    let tty: Arc<dyn File> = tty_file.clone();
    tty_file.metadata().inner.lock().file = Some(Arc::downgrade(&tty));
    tty_file.metadata().inner.lock().inode = Some(tty_inode);
    *TTY.get_unchecked_mut() = Some(tty_file);
}

const CTRL_C: u8 = 3;

pub static TTY: SyncUnsafeCell<Option<Arc<TtyFile>>> = SyncUnsafeCell::new(None);

pub struct TtyFile {
    /// Temporarily save poll in data
    buf: AtomicU8,
    metadata: FileMeta,
    inner: SpinLock<TtyInner>,
}

struct TtyInner {
    fg_pgid: Pid,
    win_size: WinSize,
    termios: Termios,
}

impl TtyFile {
    pub fn new() -> Self {
        stack_trace!();
        Self {
            buf: AtomicU8::new(255),
            metadata: FileMeta {
                inner: Mutex::new(FileMetaInner {
                    inode: None,
                    mode: crate::fs::InodeMode::FileCHR,
                    pos: 0,
                    dirent_index: 0,
                    file: None,
                }),
                prw_lock: SleepLock::new(()),
            },
            inner: SpinLock::new(TtyInner {
                fg_pgid: INITPROC_PID as u32,
                win_size: WinSize::new(),
                termios: Termios::new(),
            }),
        }
    }

    pub fn handle_irq(&self, ch: u8) {
        stack_trace!();
        log::debug!("[TtyFile::handle_irq] handle irq, ch {}", ch);
        self.buf.store(ch, Ordering::Release);
        if ch == CTRL_C {
            let pids = PROCESS_GROUP_MANAGER.get_group_by_pgid(self.inner.lock().fg_pgid as usize);
            log::debug!("[TtyFile::handle_irq] fg pid {}", self.inner.lock().fg_pgid);
            for pid in pids {
                let process = PROCESS_MANAGER.get(pid);
                if let Some(p) = process {
                    p.inner_handler(|proc| {
                        for (_, thread) in proc.threads.iter() {
                            if let Some(t) = thread.upgrade() {
                                log::debug!("[TtyFile::handle_irq] kill tid {}", t.tid());
                                t.recv_signal(SIGINT);
                            }
                        }
                    })
                }
            }
        }
    }
}

impl File for TtyFile {
    fn metadata(&self) -> &FileMeta {
        stack_trace!();
        &self.metadata
    }

    fn read<'a>(&'a self, buf: &'a mut [u8], _flags: OpenFlags) -> AsyscallRet {
        stack_trace!();
        // println!("[TtyFile::read] read...");
        struct TtyFuture<'a> {
            tty_file: &'a TtyFile,
            buf: &'a mut [u8],
            cnt: usize,
        }
        impl<'a> Future for TtyFuture<'a> {
            type Output = SyscallRet;
            fn poll(
                self: core::pin::Pin<&mut Self>,
                cx: &mut core::task::Context<'_>,
            ) -> core::task::Poll<Self::Output> {
                stack_trace!();
                let _sum_guard = SumGuard::new();
                let ch: u8;
                let self_buf = self.tty_file.buf.load(Ordering::Acquire);
                if self_buf != 0xff {
                    self.tty_file.buf.store(0xff, Ordering::Release);
                    ch = self_buf;
                } else {
                    ch = getchar();
                    if ch == 0xff {
                        CHAR_DEVICE
                            .get_unchecked_mut()
                            .as_ref()
                            .unwrap()
                            .register_waker(cx.waker().clone());
                        log::debug!("[TtyFuture::poll] nothing to read");
                        return Poll::Pending;
                    }
                }
                log::debug!(
                    "[TtyFuture::poll] recv ch {}, cnt {}, len {}",
                    ch,
                    self.cnt,
                    self.buf.len()
                );
                let this = unsafe { Pin::get_unchecked_mut(self) };
                this.buf[this.cnt] = ch;

                this.cnt += 1;
                if this.cnt == this.buf.len() {
                    Poll::Ready(Ok(this.buf.len()))
                } else {
                    Poll::Pending
                }
            }
        }
        Box::pin(TtyFuture {
            tty_file: self,
            buf,
            cnt: 0,
        })
    }

    fn write<'a>(&'a self, buf: &'a [u8], _flags: OpenFlags) -> AsyscallRet {
        stack_trace!();
        // println!("[TtyFile::write] buf {:?}...", buf);
        Box::pin(async move {
            let _sum_guard = SumGuard::new();
            let utf8_buf: Vec<u8> = buf.iter().filter(|c| c.is_ascii()).map(|c| *c).collect();
            if PRINT_LOCKED {
                let _locked = PRINT_MUTEX.lock().await;
                print!("{}", unsafe { core::str::from_utf8_unchecked(&utf8_buf) });
            } else {
                print!("{}", unsafe { core::str::from_utf8_unchecked(&utf8_buf) });
            }
            Ok(buf.len())
        })
    }

    fn pollin(&self, waker: Option<Waker>) -> GeneralRet<bool> {
        stack_trace!();
        stack_trace!();
        #[cfg(feature = "submit")]
        {
            Ok(true)
        }
        #[cfg(all(not(feature = "submit"), feature = "board_u740"))]
        {
            if self.buf.load(Ordering::Acquire) != 255 {
                return Ok(true);
            }
            let _sum_guard = SumGuard::new();
            let c = getchar();
            if c == 0xff {
                if let Some(waker) = waker {
                    CHAR_DEVICE
                        .get_unchecked_mut()
                        .as_ref()
                        .unwrap()
                        .register_waker(waker);
                    log::debug!("[TtyFuture::pollin] nothing to read");
                    // POLL_QUEUE.register(
                    //     self.metadata().inner.lock().file.as_ref().unwrap().clone(),
                    //     waker,
                    //     true,
                    // )
                }
                return Ok(false);
            } else {
                self.buf.store(c as u8, Ordering::Release);
                return Ok(true);
            }
        }
        #[cfg(all(not(feature = "submit"), not(feature = "board_u740")))]
        {
            if self.buf.load(Ordering::Acquire) != 255 {
                return Ok(true);
            }
            let _sum_guard = SumGuard::new();
            let c = getchar();
            if c == 0xff {
                if let Some(waker) = waker {
                    POLL_QUEUE.register(
                        self.metadata().inner.lock().file.as_ref().unwrap().clone(),
                        waker,
                        true,
                    );
                    log::debug!("[TtyFuture::pollin] nothing to read");
                }
                return Ok(false);
            } else {
                self.buf.store(c as u8, Ordering::Release);
                return Ok(true);
            }
        }
    }

    fn ioctl(&self, command: usize, value: usize) -> SyscallRet {
        stack_trace!();
        log::info!(
            "[TtyFile::ioctl] command {:#x}, value {:#x}",
            command,
            value
        );
        match command {
            TCGETS | TCGETA => {
                let _sum_guard = SumGuard::new();
                UserCheck::new()
                    .check_writable_slice(value as *mut u8, core::mem::size_of::<Termios>())?;
                unsafe {
                    // (value as *mut Termios).copy_from(&self.inner.lock().termios as *const Termios, 1);
                    *(value as *mut Termios) = self.inner.lock().termios;
                }
                Ok(0)
            }
            TCSETS | TCSETSW | TCSETSF => {
                let _sum_guard = SumGuard::new();
                UserCheck::new()
                    .check_readable_slice(value as *const u8, core::mem::size_of::<Termios>())?;
                unsafe {
                    self.inner.lock().termios = *(value as *const Termios);
                }
                Ok(0)
            }
            TIOCGPGRP => {
                let _sum_guard = SumGuard::new();
                UserCheck::new()
                    .check_writable_slice(value as *mut u8, core::mem::size_of::<Pid>())?;
                unsafe {
                    *(value as *mut Pid) = self.inner.lock().fg_pgid;
                    log::info!("[TtyFile::ioctl] get fg pgid {}", *(value as *const Pid));
                }
                Ok(0)
            }
            TIOCSPGRP => {
                let _sum_guard = SumGuard::new();
                UserCheck::new()
                    .check_readable_slice(value as *const u8, core::mem::size_of::<Pid>())?;
                unsafe {
                    log::info!("[TtyFile::ioctl] set fg pgid {}", *(value as *const Pid));
                    self.inner.lock().fg_pgid = *(value as *const Pid);
                }
                Ok(0)
            }
            TIOCGWINSZ => {
                let _sum_guard = SumGuard::new();
                UserCheck::new()
                    .check_writable_slice(value as *mut u8, core::mem::size_of::<WinSize>())?;
                unsafe {
                    *(value as *mut WinSize) = self.inner.lock().win_size;
                }
                Ok(0)
            }
            TIOCSWINSZ => {
                let _sum_guard = SumGuard::new();
                UserCheck::new()
                    .check_readable_slice(value as *const u8, core::mem::size_of::<WinSize>())?;
                unsafe {
                    self.inner.lock().win_size = *(value as *const WinSize);
                }
                Ok(0)
            }
            TCSBRK => Ok(0),
            _ => todo!(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Termios {
    /// Input modes
    pub iflag: u32,
    /// Ouput modes
    pub oflag: u32,
    /// Control modes
    pub cflag: u32,
    /// Local modes
    pub lflag: u32,
    pub line: u8,
    /// Terminal special characters.
    pub cc: [u8; 19],
    // pub cc: [u8; 32],
    // pub ispeed: u32,
    // pub ospeed: u32,
}

impl Termios {
    fn new() -> Self {
        stack_trace!();
        Self {
            // IMAXBEL | IUTF8 | IXON | IXANY | ICRNL | BRKINT
            iflag: 0o66402,
            // OPOST | ONLCR
            oflag: 0o5,
            // HUPCL | CREAD | CSIZE | EXTB
            cflag: 0o2277,
            // IEXTEN | ECHOTCL | ECHOKE ECHO | ECHOE | ECHOK | ISIG | ICANON
            lflag: 0o105073,
            line: 0,
            cc: [
                3,   // VINTR Ctrl-C
                28,  // VQUIT
                127, // VERASE
                21,  // VKILL
                4,   // VEOF Ctrl-D
                0,   // VTIME
                1,   // VMIN
                0,   // VSWTC
                17,  // VSTART
                19,  // VSTOP
                26,  // VSUSP Ctrl-Z
                255, // VEOL
                18,  // VREPAINT
                15,  // VDISCARD
                23,  // VWERASE
                22,  // VLNEXT
                255, // VEOL2
                0, 0,
            ],
            // ispeed: 0,
            // ospeed: 0,
        }
    }
}
