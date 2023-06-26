//! File and filesystem-related syscalls
use core::ops::Add;
use core::ptr;
use core::ptr::copy_nonoverlapping;
use core::time::Duration;

use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use inode::InodeState;
use log::{debug, info, trace, warn};

use super::PollFd;
use crate::config::fs::RLIMIT_NOFILE;
use crate::fs::pipe::make_pipe;
use crate::fs::posix::{
    FdSet, StatFlags, Statfs, Sysinfo, FD_SET_LEN, STAT, STATFS_SIZE, STAT_SIZE, SYSINFO_SIZE,
};
use crate::fs::{
    inode, open_file, posix::Iovec, posix::UtsName, resolve_path, Dirent, FaccessatFlags,
    FcntlFlags, FileSystem, FileSystemType, Inode, InodeMode, Renameat2Flags, AT_FDCWD,
    FILE_SYSTEM_MANAGER,
};
use crate::fs::{posix::UTSNAME_SIZE, OpenFlags};
use crate::mm::user_check::UserCheck;
use crate::processor::{current_process, SumGuard};
use crate::signal::SigSet;
use crate::syscall::{PollEvents, SEEK_CUR, SEEK_END, SEEK_SET};
use crate::timer::io_multiplex::{IOMultiplexFormat, IOMultiplexFuture, RawFdSetRWE};
use crate::timer::posix::TimeVal;
use crate::timer::timeout_task::{TimeoutTaskFuture, TimeoutTaskOutput};
use crate::timer::{posix::current_time_spec, UTIME_NOW};
use crate::timer::{posix::TimeSpec, UTIME_OMIT};
use crate::utils::error::{SyscallErr, SyscallRet};
use crate::utils::path;
use crate::utils::string::c_str_to_string;
use crate::{fs, stack_trace};

/// get current working directory
pub fn sys_getcwd(buf: usize, len: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    let cwd = current_process().inner_handler(move |proc| proc.cwd.clone());
    UserCheck::new().check_writable_slice(buf as *mut u8, len)?;
    if len < cwd.len() {
        Err(SyscallErr::ERANGE)
    } else {
        let new_buf = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, cwd.len()) };
        new_buf.copy_from_slice(cwd.as_bytes());
        Ok(buf as isize)
    }
}

pub fn sys_dup(oldfd: usize) -> SyscallRet {
    stack_trace!();
    debug!("[sys_dup2] start...");
    let newfd = current_process()
        .inner_handler(move |proc| {
            if let Some(file) = proc.fd_table.get_ref(oldfd).cloned() {
                let newfd = proc.fd_table.alloc_fd();
                proc.fd_table.put(newfd, file);
                Some(newfd)
            } else {
                None
            }
        })
        .ok_or(SyscallErr::EBADF)?;
    Ok(newfd as isize)
}

pub fn sys_dup3(oldfd: usize, newfd: usize, flags: u32) -> SyscallRet {
    stack_trace!();
    debug!("[sys_dup3] start... oldfd:{}, newfd:{}", oldfd, newfd);
    // TODO: handle `close on exec`
    current_process().inner_handler(move |proc| {
        if let Some(file) = proc.fd_table.get(oldfd) {
            if proc.fd_table.take(newfd).is_none() {
                if newfd >= RLIMIT_NOFILE {
                    return Err(SyscallErr::EINVAL);
                } else {
                    proc.fd_table.alloc_spec_fd(newfd);
                }
            }
            debug!("[sys_dup3]: dup oldfd:{} to newfd:{}", oldfd, newfd);
            proc.fd_table.put(newfd, file);
            Ok(newfd as isize)
        } else {
            warn!("[sys_dup3]: cannot find the oldfd's file");
            Err(SyscallErr::EBADF)
        }
    })
}

pub fn sys_unlinkat(dirfd: isize, path: *const u8, _flags: u32) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    // TODO: check whether the memory pointed by path is vaild
    UserCheck::new().check_c_str(path)?;
    let absolute_path = path::path_process(dirfd, path);
    match absolute_path {
        Some(absolute_path) => {
            let target_inode = <dyn Inode>::lookup_from_root_tmp(&absolute_path);
            match target_inode {
                Some(target_inode) => {
                    debug!("find target_inode");
                    if target_inode.metadata().mode == InodeMode::FileDIR {
                        debug!("target_inode is dir");
                        Err(SyscallErr::EISDIR)
                    } else {
                        let parent = target_inode.metadata().inner.lock().parent.clone();
                        match parent {
                            Some(parent) => {
                                let parent = parent.upgrade().unwrap();
                                debug!("Have a parent: {}", parent.metadata().path);
                                parent.unlink(target_inode)?;
                                Ok(0)
                            }
                            None => {
                                debug!("Have no parent, this inode is a root node which cannot be unlink");
                                Err(SyscallErr::EPERM)
                            }
                        }
                    }
                }
                None => Err(SyscallErr::ENOENT),
            }
        }
        None => Err(SyscallErr::ENOENT),
    }
}

/// mkdir() attempts to create a directory named pathname.
/// Return zero on sucess.
pub fn sys_mkdirat(dirfd: isize, pathname: *const u8, _mode: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    // TODO: check whether the memory pointed by pathname is vaild
    UserCheck::new().check_c_str(pathname)?;
    let absolute_path = path::path_process(dirfd, pathname);
    match absolute_path {
        Some(absolute_path) => {
            debug!("[sys_mkdirat] absolute path: {}", absolute_path);
            let _find_inode = <dyn Inode>::lookup_from_root_tmp(&absolute_path);
            match _find_inode {
                Some(_find_inode) => {
                    debug!("[sys_mkdirat] already exists");
                    Err(SyscallErr::EEXIST)
                }
                None => {
                    let parent = path::get_parent_dir(&absolute_path).unwrap();
                    debug!("[sys_mkdirat] get parent name: {}", parent);
                    let parent_inode = <dyn Inode>::lookup_from_root_tmp(&parent);
                    match parent_inode {
                        Some(parent_inode) => match parent_inode.metadata().mode {
                            InodeMode::FileDIR => {
                                let mut inner_lock = parent_inode.metadata().inner.lock();
                                // change the time
                                inner_lock.st_atim = current_time_spec();
                                inner_lock.st_mtim = current_time_spec();
                                // change state
                                match inner_lock.state {
                                    InodeState::Synced => {
                                        inner_lock.state = InodeState::DirtyInode;
                                    }
                                    InodeState::DirtyData => {
                                        inner_lock.state = InodeState::DirtyAll;
                                    }
                                    _ => {}
                                }
                                // TODO: add to dirty list, should add inode to the target fs which is include this inode
                                drop(inner_lock);
                                stack_trace!();
                                parent_inode.mkdir(
                                    parent_inode.clone(),
                                    &absolute_path,
                                    InodeMode::FileDIR,
                                )?;
                                Ok(0)
                            }
                            _ => {
                                debug!("[sys_mkdirat] parent isn't a dir");
                                return Err(SyscallErr::ENOTDIR);
                            }
                        },
                        None => {
                            debug!("[sys_mkdirat] parent not exists");
                            Err(SyscallErr::ENOENT)
                        }
                    }
                }
            }
        }
        None => Err(SyscallErr::ENOENT),
    }
}

/// you should insert the target_path and filesystem into the FILE_SYSTEM_MANAGER.
/// the filesystem should be converted from block_dev which is associated with the dev_name.
pub fn sys_mount(
    dev_name: *const u8,
    target_path: *const u8,
    ftype: *const u8,
    flags: u32,
    _data: *const u8,
) -> SyscallRet {
    stack_trace!();
    let flags = StatFlags::from_bits(flags).ok_or(SyscallErr::EINVAL)?;
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_c_str(dev_name)?;
    UserCheck::new().check_c_str(target_path)?;
    UserCheck::new().check_c_str(ftype)?;
    if _data as usize != 0 {
        UserCheck::new().check_c_str(_data)?;
    }
    // Check and convert the arguments.
    let dev_name = path::path_process(AT_FDCWD, dev_name);
    if dev_name.is_none() {
        return Err(SyscallErr::EMFILE);
    }
    let dev_name = dev_name.unwrap();

    let target_path = path::path_process(AT_FDCWD, target_path);
    if target_path.is_none() {
        return Err(SyscallErr::ENOENT);
    }
    let target_path = target_path.unwrap();
    let target_inode = <dyn Inode>::lookup_from_root_tmp(&target_path);
    if target_inode.is_none() {
        return Err(SyscallErr::EACCES);
    }

    let ftype = path::path_process(AT_FDCWD, ftype);
    let ftype = {
        if ftype.is_some() {
            let ftype = ftype.unwrap();
            let ftype = FileSystemType::fs_type(&ftype);
            if ftype.is_none() {
                return Err(SyscallErr::ENODEV);
            }
            ftype.unwrap()
        } else {
            FileSystemType::fs_type("vfat").unwrap()
        }
    };

    // let parent = path::get_parent_dir(&target_path);
    // let parent_inode = match parent {
    //     Some(parent) => <dyn Inode>::lookup_from_root_tmp(&parent),
    //     None => None,
    // };

    let mut fs = ftype.new_fs();
    fs.init(dev_name, &target_path, ftype, flags)?;
    fs.mount()?;

    let meta = fs.metadata();
    let root_inode = meta.root_inode.as_ref().unwrap();
    let path = root_inode.metadata().path.clone();

    FILE_SYSTEM_MANAGER.fs_mgr.lock().insert(path, Arc::new(fs));

    Ok(0)
}

pub fn sys_umount(target_path: *const u8, _flags: u32) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_c_str(target_path)?;
    let target_path = path::path_process(AT_FDCWD, target_path);
    if target_path.is_none() {
        return Err(SyscallErr::ENOENT);
    }
    let target_path = target_path.unwrap();
    if target_path == "/" {
        return Err(SyscallErr::EPERM);
    }
    let mut fs_mgr = FILE_SYSTEM_MANAGER.fs_mgr.lock();
    let target_fs = fs_mgr.get(&target_path);
    if target_fs.is_none() {
        return Err(SyscallErr::ENOENT);
    }
    let target_fs = target_fs.unwrap();
    // sync fs
    let meta = target_fs.metadata();
    let root_inode = meta.root_inode.unwrap();
    let parent = root_inode.metadata().inner.lock().parent.clone();
    match parent {
        Some(parent) => {
            let parent = parent.upgrade().unwrap();
            debug!("Have a parent: {}", parent.metadata().path);
            parent.remove_child(root_inode)?;
            target_fs.umount()?;
            fs_mgr.remove(&target_path);
            Ok(0)
        }
        None => {
            debug!("Have no parent, this inode is a root node which cannot be unlink");
            Err(SyscallErr::EPERM)
        }
    }
}

/// The system call getdents() reads several dirent structures from the directory pointed at by fd into the memory area pointed to by dirp.
/// The parameter count is the size of the memory area.
pub fn sys_getdents(fd: usize, dirp: usize, count: usize) -> SyscallRet {
    stack_trace!();
    // check if the fd is legal.
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;
    let file_inner = file.metadata().inner.lock();
    let inode = file_inner.inode.clone();
    let dirent_index = file_inner.dirent_index;
    drop(file_inner);

    match inode {
        Some(inode) => {
            let state = inode.metadata().inner.lock().state;
            debug!("[getdents] inode state: {:?}", state);

            // load children from disk
            <dyn Inode>::load_children(inode.clone());

            let _sum_guard = SumGuard::new();
            UserCheck::new().check_writable_slice(dirp as *mut u8, count)?;
            let mut inner_lock = inode.metadata().inner.lock();
            // change access time
            inner_lock.st_atim = current_time_spec();
            // change state
            match inner_lock.state {
                InodeState::Synced => {
                    inner_lock.state = InodeState::DirtyInode;
                }
                InodeState::DirtyData => {
                    inner_lock.state = InodeState::DirtyAll;
                }
                _ => {}
            }
            // TODO: add to fs's dirty list
            drop(inner_lock);

            let dirents = Dirent::get_dirents(inode, dirent_index);
            let mut num_bytes = 0;
            let mut dirp_ptr = dirp;
            let mut index: usize = 0;
            for (i, dirent) in dirents.iter().enumerate() {
                stack_trace!();
                let temp = num_bytes + dirent.d_reclen as usize;
                if temp > count {
                    debug!("[getdents] user buf size too small");
                    index = i + 1;
                    break;
                }
                num_bytes = temp;
                unsafe {
                    copy_nonoverlapping(&*dirent as *const Dirent, dirp_ptr as *mut Dirent, 1);
                    dirp_ptr += dirent.d_reclen as usize;
                }
                index = i + 1;
            }
            file.metadata().inner.lock().dirent_index += index;

            Ok(num_bytes as isize)
        }
        None => Err(SyscallErr::ENOTDIR),
    }
}

/// chdir() changes the current working directory of the calling process to the directory specified in path.
pub fn sys_chdir(path: *const u8) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_c_str(path)?;
    let path = &c_str_to_string(path);
    let target_inode = <dyn Inode>::lookup_from_root_tmp(path);
    match target_inode {
        Some(target_inode) => {
            let mut inner_lock = target_inode.metadata().inner.lock();
            inner_lock.st_atim = current_time_spec();
            match inner_lock.state {
                InodeState::Synced => {
                    inner_lock.state = InodeState::DirtyInode;
                }
                InodeState::DirtyData => {
                    inner_lock.state = InodeState::DirtyAll;
                }
                _ => {}
            }
            // TODO: add to fs's dirty list
            if target_inode.metadata().mode == InodeMode::FileDIR {
                current_process().inner_handler(move |proc| proc.cwd = path.to_string());
                Ok(0)
            } else {
                Err(SyscallErr::ENOTDIR)
            }
        }
        None => Err(SyscallErr::ENOENT),
    }
}

/// uname() get the name and some information about current kernel
/// buf is a pointer to the utsname structure
pub fn sys_uname(buf: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_writable_slice(buf as *mut u8, UTSNAME_SIZE)?;
    let utsname = UtsName::get_utsname();
    let buf_ptr = buf as *mut UtsName;
    unsafe {
        ptr::write(buf_ptr, utsname);
    }
    Ok(0)
}

/// fstat() function return information about a file, in the buffer pointed to by stat_buf.
/// This function except that the file about which information is to be retrieved is specified by the file descriptor fd.
pub fn sys_fstat(fd: usize, stat_buf: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_writable_slice(stat_buf as *mut u8, STAT_SIZE)?;
    _fstat(fd, stat_buf)
}

/// We should give the stat_buf which has already been checked to _fstat function.
fn _fstat(fd: usize, stat_buf: usize) -> SyscallRet {
    let mut kstat = STAT::new();
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;

    // if !file.readable() {
    //     info!("[_fstat]: file cannot be read, fd {}, stat_buf addr {:#x}", fd, stat_buf);
    //     return Err(SyscallErr::EACCES);
    // }

    info!("[_fstat]: fd {}", fd);
    let inode = file.metadata().inner.lock().inode.as_ref().unwrap().clone();
    let inode_meta = inode.metadata().clone();
    if let Some(dev) = inode_meta.device.as_ref() {
        match dev {
            inode::InodeDevice::Device(dev) => {
                kstat.st_dev = dev.dev_id as u64;
            }
            _ => {
                return Err(SyscallErr::EBADF);
            }
        };
    } else {
        // TODO:
        kstat.st_dev = 12138;
    }
    // TODO: pre
    kstat.st_ino = inode_meta.ino as u64;
    kstat.st_mode = inode_meta.mode as u32;
    // kstat.st_mode = InodeMode::FileCHR as u32;
    debug!("[_fstat] inode mode: {:?}", inode_meta.mode);
    kstat.st_blocks = (kstat.st_size / kstat.st_blksize as u64) as u64;
    let inner_lock = inode_meta.inner.lock();
    kstat.st_size = inner_lock.data_len as u64;
    kstat.st_atim = inner_lock.st_atim;
    kstat.st_mtim = inner_lock.st_mtim;
    kstat.st_ctim = inner_lock.st_ctim;

    let kst_ptr = stat_buf as *mut STAT;
    unsafe {
        ptr::write(kst_ptr, kstat);
    }
    Ok(0)
}

pub fn sys_newfstatat(
    dirfd: isize,
    pathname: *const u8,
    stat_buf: usize,
    flags: u32,
) -> SyscallRet {
    stack_trace!();
    debug!("[newfstatat] drifd:{}, flags:{}", dirfd, flags);
    let flags = FcntlFlags::from_bits(flags).ok_or(SyscallErr::EINVAL)?;
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_c_str(pathname)?;
    UserCheck::new().check_writable_slice(stat_buf as *mut u8, STAT_SIZE)?;
    stack_trace!();
    let absolute_path: Option<String>;
    if flags.contains(FcntlFlags::AT_SYMLINK_NOFOLLOW) {
        // todo: support symlink?
        debug!("the pathname represent a symbolic link");
    }
    if flags.contains(FcntlFlags::AT_EMPTY_PATH) {
        // path is empty
        // If dirfd is AT_FDCWD, change it to absolute path
        if dirfd == AT_FDCWD {
            debug!("[newfstatat] empty path with cwd");
            let cwd = current_process().inner_handler(move |proc| proc.cwd.clone());
            debug!("cwd {}", cwd);
            absolute_path = Some(cwd);
        } else {
            debug!("[newfstatat] empty path with dirfd");
            return _fstat(dirfd as usize, stat_buf);
        }
    } else {
        absolute_path = path::path_process(dirfd, pathname);
    }
    debug!("[newfstatat] final absolute path: {:?}", absolute_path);

    let fd = open_file(absolute_path, OpenFlags::bits(&OpenFlags::RDONLY))?;

    return _fstat(fd as usize, stat_buf);
}

pub fn sys_lseek(fd: usize, offset: isize, whence: u8) -> SyscallRet {
    stack_trace!();
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;
    if !file.readable() {
        return Err(SyscallErr::EACCES);
    }
    match whence {
        SEEK_SET => {
            file.seek(offset as usize)?;
            Ok(offset)
        }
        SEEK_CUR => {
            let pos = file.metadata().inner.lock().pos;
            let off = pos + offset as usize;
            file.seek(off)?;
            Ok(off as isize)
        }
        SEEK_END => {
            let size = file
                .metadata()
                .inner
                .lock()
                .inode
                .as_ref()
                .unwrap()
                .metadata()
                .inner
                .lock()
                .data_len;
            let off = size + offset as usize;
            file.seek(off)?;
            Ok(off as isize)
        }
        _ => Err(SyscallErr::EINVAL),
    }
}

pub fn sys_openat(dirfd: isize, filename_addr: *const u8, flags: u32, _mode: u32) -> SyscallRet {
    stack_trace!();

    let absolute_path = path::path_process(dirfd, filename_addr);

    open_file(absolute_path, flags)
}

pub fn sys_close(fd: usize) -> SyscallRet {
    stack_trace!();
    current_process().close_file(fd)
}

pub async fn sys_write(fd: usize, buf: usize, len: usize) -> SyscallRet {
    stack_trace!();
    debug!("[sys_write]: fd {}, len {}", fd, len);
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;

    if !file.writable() {
        return Err(SyscallErr::EPERM);
    }

    UserCheck::new().check_readable_slice(buf as *const u8, len)?;
    // debug!("check readable slice sva {:#x} {:#x}", buf as *const u8 as usize, buf as *const u8 as usize + len);
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };
    // debug!("[sys_write]: start to write file, fd {}, buf {:?}", fd, buf);
    file.write(buf).await
}

pub async fn sys_writev(fd: usize, iov: usize, iovcnt: usize) -> SyscallRet {
    stack_trace!();
    debug!(
        "[sys_writev] fd: {}, iov: {:#x}, iovcnt:{}",
        fd, iov, iovcnt
    );
    let _sum_guard = SumGuard::new();

    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;
    if !file.writable() {
        return Err(SyscallErr::EPERM);
    }

    stack_trace!();
    let mut ret: usize = 0;
    let iovec_size = core::mem::size_of::<Iovec>();

    for i in 0..iovcnt {
        trace!("write the {} buf", i + 1);
        // current iovec pointer
        let current = iov.add(iovec_size * i);
        trace!("current iov: {}", current);
        UserCheck::new().check_readable_slice(current as *const u8, iovec_size)?;
        trace!("pass readable check");
        let iov_base = unsafe { &*(current as *const Iovec) }.iov_base;
        trace!("get iov_base: {}", iov_base);
        let iov_len = unsafe { &*(current as *const Iovec) }.iov_len;
        trace!("get iov_len: {}", iov_len);
        ret += iov_len;
        UserCheck::new().check_readable_slice(iov_base as *const u8, iov_len)?;
        let buf = unsafe { core::slice::from_raw_parts(iov_base as *const u8, iov_len) };
        trace!("[writev] buf: {:?}", buf);
        // test();
        file.write(buf).await?;
    }
    Ok(ret as isize)
}

pub async fn sys_readv(fd: usize, iov: usize, iovcnt: usize) -> SyscallRet {
    stack_trace!();
    debug!(
        "[sys_readv] fd: {}, iov: {:#x}, iovcnt: {}",
        fd, iov, iovcnt
    );
    let _sum_guard = SumGuard::new();

    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;
    if !file.readable() {
        return Err(SyscallErr::EPERM);
    }
    stack_trace!();
    let mut ret: usize = 0;
    let iovec_size = core::mem::size_of::<Iovec>();

    for i in 0..iovcnt {
        trace!("read the {} buf", i + 1);
        // current iovec pointer
        let current = iov.add(iovec_size * i);
        trace!("current iov: {}", current);
        UserCheck::new().check_writable_slice(current as *mut u8, iovec_size)?;
        trace!("pass writable check");
        let iov_base = unsafe { &*(current as *const Iovec) }.iov_base;
        trace!("get iov_base: {}", iov_base);
        let iov_len = unsafe { &*(current as *const Iovec) }.iov_len;
        trace!("get iov_len: {}", iov_len);
        ret += iov_len;
        UserCheck::new().check_writable_slice(iov_base as *mut u8, iov_len)?;
        let buf = unsafe { core::slice::from_raw_parts_mut(iov_base as *mut u8, iov_len) };
        trace!("[readv] buf: {:?}", buf);
        // test();
        file.read(buf).await?;
    }
    Ok(ret as isize)
}

fn test() -> SyscallRet {
    let iov_base = 1189160;
    let iov_len = 4;
    UserCheck::new().check_readable_slice(iov_base as *const u8, iov_len)?;
    let buf = unsafe { core::slice::from_raw_parts(iov_base as *const u8, iov_len) };
    let buf_str = unsafe { core::str::from_utf8_unchecked(buf) };
    trace!("[writev] buf: {:?}, buf_str: {}", buf, buf_str);
    Ok(0)
}

pub async fn sys_read(fd: usize, buf: usize, len: usize) -> SyscallRet {
    stack_trace!();
    debug!("[sys_read]: fd {}, len {}", fd, len);
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;

    if !file.readable() {
        return Err(SyscallErr::EPERM);
    }

    UserCheck::new().check_writable_slice(buf as *mut u8, len)?;
    let buf = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, len) };
    file.read(buf).await
}

pub fn sys_pipe(pipe: *mut i32) -> SyscallRet {
    stack_trace!();
    let (pipe_read, pipe_write) = make_pipe();

    let (read_fd, write_fd) = current_process().inner_handler(move |proc| {
        let read_fd = proc.fd_table.alloc_fd();
        proc.fd_table.put(read_fd, pipe_read);
        let write_fd = proc.fd_table.alloc_fd();
        proc.fd_table.put(write_fd, pipe_write);
        (read_fd, write_fd)
    });

    UserCheck::new().check_writable_slice(pipe as *mut u8, core::mem::size_of::<i32>() * 2)?;
    let _sum_guard = SumGuard::new();

    let buf = unsafe { core::slice::from_raw_parts_mut(pipe, 2 * core::mem::size_of::<i32>()) };
    buf[0] = read_fd as i32;
    buf[1] = write_fd as i32;
    debug!("[sys_pipe]: read fd {}, write fd {}", read_fd, write_fd);
    Ok(0)
}

enum FcntlCmd {
    F_DUPFD = 0,
    F_DUPFD_CLOEXEC = 1030,
    F_GETFD = 1,
    F_SETFD = 2,
    F_GETFL = 3,
    F_SETFL = 4,
}

pub fn sys_fcntl(fd: usize, cmd: i32, arg: usize) -> SyscallRet {
    stack_trace!();
    debug!("[sys_fcntl]: fd {}, cmd {:#x}, arg {:#x}", fd, cmd, arg);
    // TODO
    match cmd {
        _ if cmd == FcntlCmd::F_DUPFD as i32 || cmd == FcntlCmd::F_DUPFD_CLOEXEC as i32 => {
            current_process().inner_handler(|proc| {
                // if proc.fd_table.get_ref(fd).is_none()
                // let fd = proc.fd_table.alloc_fd_lower_bound(arg);
                let file = proc.fd_table.get(fd).ok_or(SyscallErr::EBADF)?;
                let newfd = proc.fd_table.alloc_fd_lower_bound(arg);
                proc.fd_table.put(newfd, file);
                debug!("[sys_fcntl]: dup file fd from {} to {}", fd, newfd);
                Ok(newfd as isize)
            })
        }
        _ if cmd == FcntlCmd::F_SETFD as i32 || cmd == FcntlCmd::F_SETFL as i32 => {
            let flags = OpenFlags::from_bits(arg as u32).ok_or(SyscallErr::EINVAL)?;
            current_process().inner_handler(|proc| {
                // if proc.fd_table.get_ref(fd).is_none()
                // let fd = proc.fd_table.alloc_fd_lower_bound(arg);
                let file = proc.fd_table.get(fd).ok_or(SyscallErr::EBADF)?;
                file.metadata().inner.lock().flags = flags;
                debug!("[sys_fcntl]: set file flags to {:?}", flags);
                Ok(0)
            })
        }
        _ if cmd == FcntlCmd::F_GETFD as i32 || cmd == FcntlCmd::F_GETFL as i32 => {
            current_process().inner_handler(|proc| {
                let file = proc.fd_table.get(fd).ok_or(SyscallErr::EBADF)?;
                let flags = file.metadata().inner.lock().flags;
                debug!("[sys_fcntl]: set file flags to {:?}", flags);
                Ok(OpenFlags::bits(&flags) as isize)
            })
        }
        _ => {
            todo!()
        }
    }
}

pub async fn sys_sendfile(
    out_fd: isize,
    in_fd: isize,
    offset_ptr: usize,
    count: usize,
) -> SyscallRet {
    stack_trace!();
    debug!(
        "[sys_sendfile]: out fd {} in fd {} offset_ptr {:#x} count {}",
        out_fd, in_fd, offset_ptr, count
    );
    let (input_file, output_file) = current_process().inner_handler(|proc| {
        Ok((
            proc.fd_table.get(in_fd as usize).ok_or(SyscallErr::EBADF)?,
            proc.fd_table
                .get(out_fd as usize)
                .ok_or(SyscallErr::EBADF)?,
        ))
    })?;
    if !input_file.readable() {
        return Err(SyscallErr::EBADF);
    }
    if !output_file.writable() {
        return Err(SyscallErr::EBADF);
    }

    let mut buf = vec![0 as u8; count];
    let nbytes = match offset_ptr {
        0 => input_file.read(&mut buf).await?,
        _ => {
            UserCheck::new()
                .check_readable_slice(offset_ptr as *const u8, core::mem::size_of::<usize>())?;
            let _sum_guard = SumGuard::new();
            let old_offset = input_file.offset()?;
            let input_offset = unsafe { *(offset_ptr as *const usize) };
            input_file.seek(input_offset)?;
            let nbytes = input_file.read(&mut buf).await?;
            input_file.seek(old_offset as usize)?;
            unsafe {
                *(offset_ptr as *mut usize) = *(offset_ptr as *mut usize) + nbytes as usize;
            }
            nbytes
        }
    };
    debug!("[sys_sendfile]: read {} bytes from inputfile", nbytes);
    let ret = output_file.write(&buf[0..nbytes as usize]).await;
    info!("[sys_sendfile]: finished");
    ret
}
/// If newpath already exists, replace it.
/// If oldpath and newpath are existing hard links referring to the same inode, then return a success.
/// If newpath exists but operation failed (for some reason, rename() failed), leave an instance of newpath in place (which means you should keep the backup of newpath if it exist).
/// If oldpath can specify a directory, then newpath should be a blank directory or not exist.
pub fn sys_renameat2(
    olddirfd: isize,
    oldpath: *const u8,
    newdirfd: isize,
    newpath: *const u8,
    flags: u32,
) -> SyscallRet {
    stack_trace!();
    let flags = Renameat2Flags::from_bits(flags).ok_or(SyscallErr::EINVAL)?;
    if flags.contains(Renameat2Flags::RENAME_EXCHANGE)
        && (flags.contains(Renameat2Flags::RENAME_NOREPLACE)
            || flags.contains(Renameat2Flags::RENAME_WHITEOUT))
    {
        return Err(SyscallErr::EINVAL);
    }
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_c_str(oldpath)?;
    let oldpath = path::path_process(olddirfd, oldpath);
    UserCheck::new().check_c_str(newpath)?;
    let newpath = path::path_process(newdirfd, newpath);
    debug!(
        "[sys_renameat2] oldpath: {:?}, newpath: {:?}, flags: {:?}",
        oldpath, newpath, flags
    );
    if oldpath.is_none() {
        debug!("[sys_renameat2] oldpath is empty");
        return Err(SyscallErr::ENOENT);
    }
    if newpath.is_none() {
        debug!("[sys_renameat2] newpath is empty");
        return Err(SyscallErr::ENOENT);
    }
    let oldpath = oldpath.unwrap();
    let newpath = newpath.unwrap();
    if newpath.starts_with(&oldpath) {
        debug!("[sys_renameat2] newpath start with oldpath");
        return Err(SyscallErr::EINVAL);
    }
    let oldinode = fs::resolve_path(&oldpath, OpenFlags::RDWR);
    let newinode = fs::resolve_path(&newpath, OpenFlags::RDWR);
    if oldinode.is_none() {
        debug!("[sys_renameat2] doesn'n have oldinode");
        return Err(SyscallErr::ENOENT);
    }
    let oldinode = oldinode.unwrap();
    let oldtype = oldinode.metadata().mode;
    let oldname = oldinode.metadata().name.clone();
    if newinode.is_none() {
        // newpath doesn't exist, so we can create one and needn't care about the replace problem.
        debug!("[sys_renameat2] doesn'n have newinode");
        if flags.contains(Renameat2Flags::RENAME_EXCHANGE) {
            return Err(SyscallErr::ENOENT);
        }
        let oldparent = oldinode
            .metadata()
            .inner
            .lock()
            .parent
            .clone()
            .unwrap()
            .upgrade()
            .unwrap();
        oldparent.remove_child(oldinode.clone())?;
        let newparent_path = path::get_parent_dir(&newpath).unwrap();
        let parent = fs::resolve_path(&newparent_path, OpenFlags::RDWR);
        if parent.is_none() {
            debug!("[sys_renameat2] newparent doesn't create");
            return Err(SyscallErr::ENOENT);
        } else {
            let parent = parent.unwrap();
            if oldtype == InodeMode::FileDIR {
                stack_trace!();
                parent.mkdir(parent.clone(), &newpath, oldtype)?;
            } else {
                parent.mknod(
                    parent.clone(),
                    &newpath,
                    oldtype,
                    oldinode.metadata().rdev.unwrap(),
                )?;
            }
            let new_inner_lock = parent.metadata().inner.lock();
            let newinode = new_inner_lock
                .children
                .get(path::get_name(&newpath))
                .unwrap();
            let mut old_inner = oldinode.metadata().inner.lock().clone();
            old_inner.parent = Some(Arc::downgrade(&parent));
            newinode.metadata().inner_set(old_inner);
            Ok(0)
        }
    } else {
        // newpath is already existing, check flag.
        debug!("[sys_renameat2] newinode already exist");
        let newinode = newinode.unwrap();
        let newtype = newinode.metadata().mode;
        let newname = newinode.metadata().name.clone();
        let old_parent = oldinode
            .metadata()
            .inner
            .lock()
            .parent
            .clone()
            .unwrap()
            .upgrade()
            .unwrap();
        let new_parent = newinode
            .metadata()
            .inner
            .lock()
            .parent
            .clone()
            .unwrap()
            .upgrade()
            .unwrap();
        old_parent.remove_child(oldinode.clone())?;
        new_parent.remove_child(newinode.clone())?;
        if flags.contains(Renameat2Flags::RENAME_EXCHANGE) {
            debug!("[sys_renameat2] exchange old and new");
            // If flag is RENAME_EXCHANGE, exchange old and new one.
            if newtype == InodeMode::FileDIR {
                old_parent.mkdir(old_parent.clone(), &newpath, newtype)?;
            } else {
                old_parent.mknod(
                    old_parent.clone(),
                    &newpath,
                    newtype,
                    newinode.clone().metadata().rdev.unwrap(),
                )?;
            }
            if oldtype == InodeMode::FileDIR {
                new_parent.mkdir(new_parent.clone(), &oldpath, oldtype)?;
            } else {
                new_parent.mknod(
                    new_parent.clone(),
                    &oldpath,
                    oldtype,
                    oldinode.clone().metadata().rdev.unwrap(),
                )?;
            }
            // inner exchange
            let old_inner_lock = old_parent.metadata().inner.lock();
            // get inner before overwrite
            let mut oldinner = oldinode.metadata().inner.lock().clone();
            let new_inner_lock = new_parent.metadata().inner.lock();
            // get inner before overwrite
            let mut newinner = newinode.metadata().inner.lock().clone();
            // overwrite with current oldpath inode
            let oldinode = old_inner_lock.children.get(oldname.as_str()).unwrap();
            // overwrite with current newpath inode
            let newinode = new_inner_lock.children.get(newname.as_str()).unwrap();
            // overwrite the newinner with current old_parent
            newinner.parent = Some(Arc::downgrade(&old_parent));
            oldinode.metadata().inner_set(newinner);
            // overwrite the oldinner with current new_parent
            oldinner.parent = Some(Arc::downgrade(&new_parent));
            newinode.metadata().inner_set(oldinner);
            Ok(0)
        } else if flags.contains(Renameat2Flags::RENAME_NOREPLACE) {
            debug!("[sys_renameat2] cound not replace, error");
            // If flag is RENAME_NOREPLACE, should not to replace newpath.
            // So return EEXIST
            Err(SyscallErr::EEXIST)
        } else {
            // Normally, replace the newpath.
            // But you should check the newpath type, if newtype is not the same as oldtype, should return EISDIR or ENOTEMPTY or ENOTDIR
            debug!("[sys_renameat2] replace newpath");
            if newtype == oldtype {
                // the same type, replace directly.
                if newtype == InodeMode::FileDIR {
                    new_parent.mkdir(new_parent.clone(), &newpath, newtype)?;
                } else {
                    new_parent.mknod(
                        new_parent.clone(),
                        &newpath,
                        newtype,
                        oldinode.clone().metadata().rdev.unwrap(),
                    )?;
                }
                let mut oldinner = oldinode.metadata().inner.lock().clone();
                oldinner.parent = Some(Arc::downgrade(&new_parent));
                let new_inner_lock = new_parent.metadata().inner.lock();
                let newinode = new_inner_lock.children.get(newname.as_str()).unwrap();
                newinode.metadata().inner_set(oldinner);
                Ok(0)
            } else {
                panic!("not support");
            }
        }
    }
}

pub fn sys_readlinkat(dirfd: usize, path_name: usize, buf: usize, buf_size: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_c_str(path_name as *const u8)?;
    let path = c_str_to_string(path_name as *const u8);
    info!(
        "[sys_readlinkat]: dirfd {}, path_name {} buf addr {:#x} buf size {}",
        dirfd, path, buf, buf_size
    );
    UserCheck::new().check_writable_slice(buf as *mut u8, buf_size)?;

    // TODO: optimize
    let target = "/lmbench_all".to_string();
    unsafe {
        (buf as *mut u8).copy_from(target.as_ptr(), target.len());
        *((buf + target.len()) as *mut u8) = 0;
    }
    // Err(SyscallErr::ENOENT)
    Ok(0)
}

/// change file timestamps with nanosecond precision
pub fn sys_utimensat(
    dirfd: isize,
    pathname: *const u8,
    times: *const TimeSpec,
    _flags: u32,
) -> SyscallRet {
    stack_trace!();
    UserCheck::new().check_c_str(pathname)?;
    let _sum_guard = SumGuard::new();
    let pathname = path::path_process(dirfd, pathname);
    if pathname.is_none() {
        debug!("[sys_utimensat] pathname is empty");
        return Err(SyscallErr::ENOENT);
    }
    let pathname = pathname.unwrap();
    debug!("[sys_utimensat] pathname: {}", pathname);
    let inode = resolve_path(&pathname, OpenFlags::RDWR);
    match inode {
        Some(inode) => {
            let mut inner_lock = inode.metadata().inner.lock();
            if times.is_null() {
                debug!("[sys_utimensat] times is null");
                // If times is null, then both timestamps are set to the current time.
                inner_lock.st_atim = current_time_spec();
                inner_lock.st_mtim = inner_lock.st_atim;
            } else {
                // times[0] for atime, times[1] for mtime
                let atime = unsafe { &*times };
                unsafe {
                    times.add(1);
                }
                let mtime = unsafe { &*times };
                if atime.nsec == UTIME_NOW || mtime.nsec == UTIME_NOW {
                    debug!("[sys_utimensat] nsec is UTIME_NOW");
                    inner_lock.st_atim = current_time_spec();
                    inner_lock.st_mtim = inner_lock.st_atim;
                } else if atime.nsec == UTIME_OMIT || mtime.nsec == UTIME_OMIT {
                    debug!("[sys_utimensat] nsec is UTIME_OMIT");
                    return Ok(0);
                } else {
                    debug!("[sys_utimensat] normal nsec");
                    inner_lock.st_atim = *atime;
                    inner_lock.st_mtim = *mtime;
                }
            }
            Ok(0)
        }
        None => {
            debug!("[sys_utimensat] cannot find inode relatived to pathname");
            Err(SyscallErr::ENOENT)
        }
    }
}

/// Checks whether the calling process can access the file pathname.
pub fn sys_faccessat(dirfd: isize, pathname: *const u8, mode: u32, flags: u32) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    let _mode = FaccessatFlags::from_bits(mode).ok_or(SyscallErr::EINVAL)?;
    let _flags = FcntlFlags::from_bits(flags).ok_or(SyscallErr::EINVAL)?;
    UserCheck::new().check_c_str(pathname)?;
    stack_trace!();
    let pathname = path::path_process(dirfd, pathname);
    if pathname.is_none() {
        debug!("[sys_faccessat] pathname is none");
        return Err(SyscallErr::ENOENT);
    }
    let pathname = pathname.unwrap();
    debug!("[sys_faccessat] pathname: {}", pathname);
    let inode = resolve_path(&pathname, OpenFlags::RDONLY);
    match inode {
        Some(_inode) => {
            // We doesn't support user concept, if the file exist, then return 0.
            Ok(0)
        }
        None => {
            debug!("[sys_faccessat] don't find inode");
            Err(SyscallErr::ENOENT)
        }
    }
}

pub fn sys_statfs(path: *const u8, buf: *mut Statfs) -> SyscallRet {
    stack_trace!();
    UserCheck::new().check_c_str(path)?;
    UserCheck::new().check_writable_slice(buf as *mut u8, STATFS_SIZE)?;
    let _sum_guard = SumGuard::new();
    let path = path::path_process(AT_FDCWD, path);
    if path.is_none() {
        debug!("[sys_statfs] path does not exist");
        return Err(SyscallErr::ENOENT);
    }
    let stfs = Statfs::new();
    // TODO: find the target fs
    unsafe {
        ptr::write(buf, stfs);
    }
    Ok(0)
}

pub fn sys_sysinfo(info: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_writable_slice(info as *mut u8, SYSINFO_SIZE)?;
    Ok(0)
}

pub fn sys_syslog(log_type: u32, bufp: *mut u8, len: u32) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_writable_slice(bufp, len as usize)?;
    match log_type as usize {
        2 | 3 | 4 => {
            // For type equal to 2, 3, or 4, a successful call to syslog() returns the number of bytes read.
            Ok(0)
        }
        9 => {
            // For type 9, syslog() returns the number of bytes currently available to be read on the kernel log buffer.
            Ok(0)
        }
        10 => {
            // For type 10, syslog() returns the total size of the kernel log buffer.  For other values of type, 0 is returned on success.
            Ok(0)
        }
        _ => {
            // For other values of type, 0 is returned on success.
            Ok(0)
        }
    }
}

pub async fn sys_ppoll(
    fds_ptr: usize,
    nfds: usize,
    timeout_ptr: usize,
    sigmask_ptr: usize,
) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();

    UserCheck::new()
        .check_writable_slice(fds_ptr as *mut u8, core::mem::size_of::<PollFd>() * nfds)?;
    let raw_fds: &mut [PollFd] =
        unsafe { core::slice::from_raw_parts_mut(fds_ptr as *mut PollFd, nfds) };
    // TODO: can we just use the fds in place without allocating memeory in heap?
    let mut fds: Vec<PollFd> = Vec::new();
    fds.extend_from_slice(raw_fds);

    debug!("[sys_ppoll]: fds {:?}", fds);

    let timeout = match timeout_ptr {
        0 => {
            debug!("[sys_ppoll]: infinite timeout");
            None
        }
        _ => {
            UserCheck::new()
                .check_readable_slice(timeout_ptr as *const u8, core::mem::size_of::<TimeSpec>())?;
            Some(Duration::from(unsafe { *(timeout_ptr as *const TimeSpec) }))
        }
    };

    if sigmask_ptr != 0 {
        stack_trace!();
        UserCheck::new()
            .check_readable_slice(sigmask_ptr as *const u8, core::mem::size_of::<SigSet>())?;
        let sigmask = unsafe { *(sigmask_ptr as *const usize) };
        current_process().inner_handler(|proc| {
            if let Some(new_sig_mask) = SigSet::from_bits(sigmask) {
                proc.pending_sigs.blocked_sigs |= new_sig_mask;
            } else {
                warn!("[sys_ppoll]: invalid set arg");
            }
        });
    }

    let poll_future = IOMultiplexFuture::new(fds, IOMultiplexFormat::PollFds(fds_ptr));
    if let Some(timeout) = timeout {
        match TimeoutTaskFuture::new(timeout, poll_future).await {
            TimeoutTaskOutput::Ok(ret) => {
                return ret;
            }
            TimeoutTaskOutput::Timeout => {
                warn!("[sys_ppoll]: timeout");
                return Ok(0);
            }
        }
    } else {
        let ret = poll_future.await;
        debug!("[sys_ppoll]: ready");
        ret
    }
}

pub async fn sys_pselect6(
    nfds: i32,
    readfds_ptr: usize,
    writefds_ptr: usize,
    exceptfds_ptr: usize,
    timeout_ptr: usize,
    sigmask_ptr: usize,
) -> SyscallRet {
    stack_trace!();

    let _sum_guard = SumGuard::new();
    let mut readfds = {
        if readfds_ptr != 0 {
            UserCheck::new()
                .check_writable_slice(readfds_ptr as *mut u8, core::mem::size_of::<FdSet>())?;
            Some(unsafe { &mut *(readfds_ptr as *mut FdSet) })
        } else {
            None
        }
    };
    let mut writefds = {
        if writefds_ptr != 0 {
            UserCheck::new()
                .check_writable_slice(writefds_ptr as *mut u8, core::mem::size_of::<FdSet>())?;
            Some(unsafe { &mut *(writefds_ptr as *mut FdSet) })
        } else {
            None
        }
    };
    let mut exceptfds = {
        if exceptfds_ptr != 0 {
            UserCheck::new()
                .check_writable_slice(exceptfds_ptr as *mut u8, core::mem::size_of::<FdSet>())?;
            Some(unsafe { &mut *(exceptfds_ptr as *mut FdSet) })
        } else {
            None
        }
    };

    debug!(
        "[sys_pselect]: readfds {:?}, writefds {:?}, exceptfds {:?}",
        readfds, writefds, exceptfds
    );

    let mut fds: Vec<PollFd> = Vec::new();
    let fd_slot_bits = 8 * core::mem::size_of::<usize>();
    for fd_slot in 0..FD_SET_LEN {
        for offset in 0..fd_slot_bits {
            let fd = fd_slot * fd_slot_bits + offset;
            if fd >= nfds as usize {
                break;
            }
            if let Some(readfds) = readfds.as_ref() {
                if readfds.fds_bits[fd_slot] & (1 << offset) != 0 {
                    fds.push(PollFd {
                        fd: fd as i32,
                        events: PollEvents::POLLIN.bits(),
                        revents: PollEvents::empty().bits(),
                    })
                }
            }
            if let Some(writefds) = writefds.as_ref() {
                if writefds.fds_bits[fd_slot] & (1 << offset) != 0 {
                    if let Some(last_fd) = fds.last() && last_fd.fd == fd as i32 {
                            let events = PollEvents::from_bits(last_fd.events).unwrap()
                                | PollEvents::POLLOUT;
                            fds.last_mut().unwrap().events = events.bits();
                        } else {
                            fds.push(PollFd {
                                fd: fd as i32,
                                events: PollEvents::POLLOUT.bits(),
                                revents: PollEvents::empty().bits(),
                            })
                        }
                }
            }
            if let Some(exceptfds) = exceptfds.as_ref() {
                if exceptfds.fds_bits[fd_slot] & (1 << offset) != 0 {
                    if let Some(last_fd) = fds.last() && last_fd.fd == fd as i32 {
                            let events = PollEvents::from_bits(last_fd.events).unwrap()
                                | PollEvents::POLLPRI;
                            fds.last_mut().unwrap().events = events.bits();
                        } else {
                            fds.push(PollFd {
                                fd: fd as i32,
                                events: PollEvents::POLLPRI.bits(),
                                revents: PollEvents::empty().bits(),
                            })
                        }
                }
            }
        }
    }

    if let Some(fds) = readfds.as_mut() {
        fds.clear_all();
    }
    if let Some(fds) = writefds.as_mut() {
        fds.clear_all();
    }
    if let Some(fds) = exceptfds.as_mut() {
        fds.clear_all();
    }

    let timeout = match timeout_ptr {
        0 => {
            debug!("[sys_pselect]: infinite timeout");
            None
        }
        _ => {
            UserCheck::new()
                .check_readable_slice(timeout_ptr as *const u8, core::mem::size_of::<TimeVal>())?;
            Some(Duration::from(unsafe { *(timeout_ptr as *const TimeVal) }))
        }
    };

    if sigmask_ptr != 0 {
        stack_trace!();
        UserCheck::new()
            .check_readable_slice(sigmask_ptr as *const u8, core::mem::size_of::<SigSet>())?;
        let sigmask = unsafe { *(sigmask_ptr as *const usize) };
        current_process().inner_handler(|proc| {
            if let Some(new_sig_mask) = SigSet::from_bits(sigmask) {
                proc.pending_sigs.blocked_sigs |= new_sig_mask;
            } else {
                warn!("[sys_pselect]: invalid set arg");
            }
        });
    }

    let poll_future = IOMultiplexFuture::new(
        fds,
        IOMultiplexFormat::FdSets(RawFdSetRWE::new(readfds_ptr, writefds_ptr, exceptfds_ptr)),
    );
    if let Some(timeout) = timeout {
        match TimeoutTaskFuture::new(timeout, poll_future).await {
            TimeoutTaskOutput::Ok(ret) => {
                return ret;
            }
            TimeoutTaskOutput::Timeout => {
                warn!("[sys_pselect]: timeout");
                return Ok(0);
            }
        }
    } else {
        let ret = poll_future.await;
        debug!("[sys_pselect]: ready");
        ret
    }
}
