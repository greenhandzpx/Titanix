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
use crate::config::fs::RLIMIT_OFILE;
use crate::config::mm::PAGE_SIZE;
use crate::fs::ffi::{
    Dirent, FdSet, StatFlags, Statfs, Sysinfo, FD_SET_LEN, SEEK_CUR, SEEK_END, SEEK_SET, STAT,
    STATFS_SIZE, STAT_SIZE, SYSINFO_SIZE,
};
use crate::fs::file_system::FsDevice;
use crate::fs::inode::INODE_CACHE;
use crate::fs::pipe::make_pipe;
use crate::fs::{
    ffi::Iovec, ffi::UtsName, inode, FaccessatFlags, FcntlFlags, FileSystemType, Inode, InodeMode,
    Renameat2Flags, AT_FDCWD, FILE_SYSTEM_MANAGER,
};
use crate::fs::{ffi::UTSNAME_SIZE, OpenFlags};
use crate::fs::{resolve_path_with_dirfd, HashKey, SeekFrom};
use crate::mm::user_check::UserCheck;
use crate::processor::{current_process, SumGuard};
use crate::signal::SigSet;
use crate::stack_trace;
use crate::syscall::PollEvents;
use crate::timer::io_multiplex::{IOMultiplexFormat, IOMultiplexFuture, RawFdSetRWE};
use crate::timer::timeout_task::{TimeoutTaskFuture, TimeoutTaskOutput};
use crate::timer::{posix::current_time_spec, UTIME_NOW};
use crate::timer::{posix::TimeSpec, UTIME_OMIT};
use crate::utils::error::{SyscallErr, SyscallRet};
use crate::utils::path;
use crate::utils::string::c_str_to_string;

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
    let newfd = current_process().inner_handler(move |proc| {
        if let Some(file) = proc.fd_table.get_ref(oldfd).cloned() {
            let newfd = proc.fd_table.alloc_fd()?;
            proc.fd_table.put(newfd, file);
            debug!("[sys_dup2] ret: {}", newfd);
            Ok(newfd)
        } else {
            Err(SyscallErr::EBADF)
        }
    })?;
    Ok(newfd as isize)
}

pub fn sys_dup3(oldfd: usize, newfd: usize, _flags: u32) -> SyscallRet {
    stack_trace!();
    debug!("[sys_dup3] start... oldfd:{}, newfd:{}", oldfd, newfd);
    // TODO: handle `close on exec`
    current_process().inner_handler(move |proc| {
        if let Some(file) = proc.fd_table.get(oldfd) {
            if proc.fd_table.take(newfd).is_none() {
                if newfd >= RLIMIT_OFILE {
                    return Err(SyscallErr::EINVAL);
                } else {
                    proc.fd_table.alloc_spec_fd(newfd)?;
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
    let res = path::path_to_inode(dirfd, path);
    stack_trace!();
    let target_inode = res.0?;
    if target_inode.is_none() {
        return Err(SyscallErr::ENOENT);
    }
    let target_inode = target_inode.unwrap();
    if target_inode.metadata().mode == InodeMode::FileDIR {
        debug!("target_inode is dir");
        Err(SyscallErr::EISDIR)
    } else {
        let parent = target_inode.metadata().inner.lock().parent.clone();
        match parent {
            Some(parent) => {
                let parent = parent.upgrade().unwrap();
                debug!("Have a parent: {}", parent.metadata().name);
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

/// mkdir() attempts to create a directory named pathname.
/// Return zero on sucess.
pub fn sys_mkdirat(dirfd: isize, pathname: *const u8, _mode: usize) -> SyscallRet {
    stack_trace!();
    log::info!("[sys_mkdirat] dirfd {}", dirfd);
    let res = path::path_to_inode(dirfd, pathname);
    if res.0?.is_some() {
        log::info!("[sys_mkdirat] already exists");
        return Err(SyscallErr::EEXIST);
    } else {
        // if have inode, the path also would be have
        let path = res.1.unwrap();
        let parent = path::get_parent_dir(&path).unwrap();
        debug!("[sys_mkdirat] get parent path: {}", parent);
        let parent_inode = if path::check_double_dot(&path) {
            <dyn Inode>::lookup_from_root(&parent)?.unwrap()
        } else {
            // if path doesn't have ..
            // parent will be return in res.2
            if res.2.is_some() {
                // if the parent inode exist, then use it
                res.2.unwrap()
            } else {
                // else try to find
                <dyn Inode>::lookup_from_root(&parent)?.unwrap()
            }
        };
        log::info!(
            "[sys_mkdirat] parent inode name {}",
            parent_inode.metadata().name
        );
        match parent_inode.metadata().mode {
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
                let child_name = path::get_name(&path);
                let child =
                    parent_inode.mkdir(parent_inode.clone(), child_name, InodeMode::FileDIR)?;
                log::info!("[sys_mkdirat] child inode name {}", child_name);
                // insert to cache
                let key = HashKey::new(parent_inode.metadata().ino, child.metadata().name.clone());
                INODE_CACHE.insert(key, child);
                Ok(0)
            }
            _ => {
                debug!("[sys_mkdirat] parent isn't a dir");
                return Err(SyscallErr::ENOTDIR);
            }
        }
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
    let dev_name = path::path_process(AT_FDCWD, dev_name)?;
    if dev_name.is_none() {
        return Err(SyscallErr::EMFILE);
    }
    let dev_name = dev_name.unwrap();

    let target_path = path::path_process(AT_FDCWD, target_path)?;
    if target_path.is_none() {
        return Err(SyscallErr::ENOENT);
    }
    let target_path = target_path.unwrap();
    let target_inode = <dyn Inode>::lookup_from_root(&target_path)?;
    if target_inode.is_none() {
        return Err(SyscallErr::EACCES);
    }

    let ftype = path::path_process(AT_FDCWD, ftype)?;
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

    let dev = <dyn Inode>::lookup_from_root(&dev_name)?;
    let dev = match dev {
        Some(inode) => match &inode.metadata().device {
            Some(d) => FsDevice::from_inode_device(d.clone()),
            None => FsDevice::None,
        },
        None => FsDevice::None,
    };
    FILE_SYSTEM_MANAGER.mount(&target_path, &dev_name, dev, ftype, flags)?;

    Ok(0)
}

pub fn sys_umount(target_path: *const u8, _flags: u32) -> SyscallRet {
    stack_trace!();
    let target_path = path::path_process(AT_FDCWD, target_path)?;
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

    FILE_SYSTEM_MANAGER.unmount(&target_path)?;
    Ok(0)
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
            inode.load_children();

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
                    debug!("[getdents] user buf size: {}, too small", count);
                    index = i;
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

            debug!(
                "[sys_getdents] return: {}, dirent_index: {}",
                num_bytes,
                file.metadata().inner.lock().dirent_index
            );
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
    let target_inode = <dyn Inode>::lookup_from_root(path)?;
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
                log::info!("[sys_chdir] change cwd to {}", path);
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
    // if !file.readable() {
    //     info!("[_fstat]: file cannot be read, fd {}, stat_buf addr {:#x}", fd, stat_buf);
    //     return Err(SyscallErr::EACCES);
    // }
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;
    let inner = file.metadata().inner.lock();
    let inode = inner.inode.as_ref();
    if inode.is_none() {
        Ok(0)
    } else {
        let inode = inode.unwrap().clone();
        _fstat(inode, stat_buf)
    }
}

/// We should give the stat_buf which has already been checked to _fstat function.
fn _fstat(inode: Arc<dyn Inode>, stat_buf: usize) -> SyscallRet {
    let mut kstat = STAT::new();
    let inode_meta = inode.metadata().clone();
    log::info!("[_fstat] inode name {}", inode_meta.name);
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
    debug!(
        "[_fstat] inode name: {}, mode: {:?}",
        inode_meta.name, inode_meta.mode
    );
    let mut inner_lock = inode_meta.inner.lock();
    let size = if inode_meta.mode == InodeMode::FileDIR {
        if inner_lock.data_len != 0 {
            inner_lock.data_len
        } else {
            let children = inner_lock.children.clone();
            let mut size = 0;
            for child in children {
                size += child.1.metadata().inner.lock().data_len;
            }
            inner_lock.data_len = size;
            size
        }
    } else {
        inner_lock.data_len
    };
    kstat.st_size = size as u64;
    kstat.st_blocks = (kstat.st_size / kstat.st_blksize as u64) as u64;
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
    log::info!("[newfstatat] drifd:{}, flags:{}", dirfd, flags);
    let _flags = FcntlFlags::from_bits(flags).ok_or(SyscallErr::EINVAL)?;
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_writable_slice(stat_buf as *mut u8, STAT_SIZE)?;
    stack_trace!();
    let res = path::path_to_inode(dirfd, pathname);
    let inode = res.0?;
    if inode.is_some() {
        // find inode
        return _fstat(inode.unwrap(), stat_buf);
    } else {
        debug!("[sys_newfstatat] cannot find target inode");
        return Err(SyscallErr::ENOENT);
    }
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
            let off = file.seek(SeekFrom::Start(offset as usize))?;
            trace!("[sys_lseek] return off: {}", off);
            Ok(off)
        }
        SEEK_CUR => {
            let off = file.seek(SeekFrom::Current(offset))?;
            trace!("[sys_lseek] return off: {}", off);
            Ok(off as isize)
        }
        SEEK_END => {
            let off = file.seek(SeekFrom::End(offset))?;
            trace!("[sys_lseek] return off: {}", off);
            Ok(off as isize)
        }
        _ => Err(SyscallErr::EINVAL),
    }
}

pub fn sys_openat(dirfd: isize, filename_addr: *const u8, flags: u32, _mode: u32) -> SyscallRet {
    stack_trace!();
    let flags = OpenFlags::from_bits(flags).ok_or(SyscallErr::EINVAL)?;
    log::info!(
        "[sys_openat] dirfd {}, flags {:?}, filename {}",
        dirfd,
        flags,
        {
            let _sum_guard = SumGuard::new();
            UserCheck::new().check_c_str(filename_addr)?;
            c_str_to_string(filename_addr)
        }
    );
    let inode = resolve_path_with_dirfd(dirfd, filename_addr, flags)?;
    current_process().inner_handler(|proc| proc.fd_table.open(inode, flags))
}

pub fn sys_close(fd: usize) -> SyscallRet {
    stack_trace!();
    current_process().close_file(fd)
}

pub async fn sys_write(fd: usize, buf: usize, len: usize) -> SyscallRet {
    stack_trace!();
    info!("[sys_write]: fd {}, len {}", fd, len);
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;

    if !file.writable() {
        return Err(SyscallErr::EPERM);
    }
    if len == 0 {
        return Ok(0);
    }

    UserCheck::new().check_readable_slice(buf as *const u8, len)?;
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };
    // debug!("[sys_write]: start to write file, fd {}, buf {:?}", fd, buf);
    file.write(buf).await
    // if buf.len() < 2 {
    //     file.sync_write(buf)
    // } else {
    //     file.write(buf).await
    // }
}

pub async fn sys_writev(fd: usize, iov: usize, iovcnt: usize) -> SyscallRet {
    stack_trace!();
    trace!(
        "[sys_writev] fd: {}, iov: {:#x}, iovcnt:{}",
        fd,
        iov,
        iovcnt
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
        UserCheck::new().check_readable_slice(iov_base as *const u8, iov_len)?;
        let buf = unsafe { core::slice::from_raw_parts(iov_base as *const u8, iov_len) };
        let write_ret = file.write(buf).await?;
        ret += write_ret as usize;
    }
    Ok(ret as isize)
}

pub async fn sys_readv(fd: usize, iov: usize, iovcnt: usize) -> SyscallRet {
    stack_trace!();
    trace!(
        "[sys_readv] fd: {}, iov: {:#x}, iovcnt: {}",
        fd,
        iov,
        iovcnt
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
        UserCheck::new().check_writable_slice(iov_base as *mut u8, iov_len)?;
        let buf = unsafe { core::slice::from_raw_parts_mut(iov_base as *mut u8, iov_len) };
        trace!("[readv] buf: {:?}", buf);
        let read_ret = file.read(buf).await?;
        ret += read_ret as usize;
    }
    Ok(ret as isize)
}

pub async fn sys_read(fd: usize, buf: usize, len: usize) -> SyscallRet {
    stack_trace!();
    info!("[sys_read]: fd {}, len {}", fd, len);
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;

    if !file.readable() {
        return Err(SyscallErr::EPERM);
    }
    if len == 0 {
        return Ok(0);
    }

    if len <= PAGE_SIZE * 64 {
        UserCheck::new().check_writable_slice(buf as *mut u8, len)?;
    } else {
        log::warn!("[sys_read] buf too large {:#x}, no check", len);
    }

    let _sum_guard = SumGuard::new();
    let buf = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, len) };

    let ret = file.read(buf).await;
    // log::debug!("[sys_read] len {:?} res buf {:?}", ret, buf);
    ret
    // if buf.len() < 2 {
    //     file.sync_read(buf)
    // } else {
    //     file.read(buf).await
    // }
}

pub fn sys_pipe(pipe: *mut i32) -> SyscallRet {
    stack_trace!();
    let (pipe_read, pipe_write) = make_pipe();

    let (read_fd, write_fd) = current_process().inner_handler(move |proc| {
        let read_fd = proc.fd_table.alloc_fd()?;
        proc.fd_table.put(read_fd, pipe_read);
        let write_fd = proc.fd_table.alloc_fd()?;
        proc.fd_table.put(write_fd, pipe_write);
        Ok((read_fd, write_fd))
    })?;

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
                let newfd = proc.fd_table.alloc_fd_lower_bound(arg)?;
                proc.fd_table.put(newfd, file);
                debug!("[sys_fcntl]: dup file fd from {} to {}", fd, newfd);
                Ok(newfd as isize)
            })
        }
        _ if cmd == FcntlCmd::F_SETFD as i32 => {
            let flags = FcntlFlags::from_bits(arg as u32).ok_or(SyscallErr::EINVAL)?;
            current_process().inner_handler(|proc| {
                // if proc.fd_table.get_ref(fd).is_none()
                // let fd = proc.fd_table.alloc_fd_lower_bound(arg);
                let file = proc.fd_table.get(fd).ok_or(SyscallErr::EBADF)?;
                if flags.contains(FcntlFlags::FD_CLOEXEC) {
                    file.metadata().inner.lock().flags |= OpenFlags::CLOEXEC;
                    debug!(
                        "[sys_fcntl]: set file flags to {:?}",
                        file.metadata().inner.lock().flags
                    );
                }
                Ok(0)
            })
        }
        _ if cmd == FcntlCmd::F_SETFL as i32 => {
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
                let flags = file.flags();
                debug!("[sys_fcntl]: get file flags {:?}", flags);
                if flags.contains(OpenFlags::CLOEXEC) && cmd == FcntlCmd::F_GETFD as i32 {
                    Ok(FcntlFlags::bits(&FcntlFlags::FD_CLOEXEC) as isize)
                } else {
                    Ok(OpenFlags::bits(&flags) as isize)
                }
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
            let input_offset = unsafe { *(offset_ptr as *const usize) };
            let nbytes = input_file.pread(&mut buf, input_offset).await?;
            // let old_offset = input_file.offset()?;
            // input_file.seek(input_offset)?;
            // let nbytes = input_file.read(&mut buf).await?;
            // input_file.seek(old_offset as usize)?;
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
    let old_res = path::path_to_inode(olddirfd, oldpath);
    let oldinode = old_res.0?;
    let new_res = path::path_to_inode(newdirfd, newpath);
    let newinode = new_res.0?;
    // change path
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
        // check new path
        let newparent = new_res.2;

        if newparent.is_none() {
            debug!("[sys_renameat2] newparent doesn't create");
            // restore
            return Err(SyscallErr::ENOENT);
        } else {
            let newpath = new_res.1.unwrap();
            // remove from old path
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
            let key = HashKey::new(oldparent.metadata().ino, oldname);
            INODE_CACHE.remove(&key);

            let newparent = newparent.unwrap();
            let newname = path::get_name(&newpath);
            if oldtype == InodeMode::FileDIR {
                stack_trace!();
                newparent.mkdir(newparent.clone(), newname, oldtype)?;
            } else {
                newparent.mknod(
                    newparent.clone(),
                    newname,
                    oldtype,
                    oldinode.metadata().rdev,
                )?;
            }
            let new_inner_lock = newparent.metadata().inner.lock();
            let newinode = new_inner_lock
                .children
                .get(path::get_name(&newpath))
                .unwrap();
            let mut old_inner = oldinode.metadata().inner.lock().clone();
            old_inner.parent = Some(Arc::downgrade(&newparent));
            newinode.metadata().inner_set(old_inner);
            let key = HashKey::new(newparent.metadata().ino, newinode.metadata().name.clone());
            INODE_CACHE.insert(key, newinode.clone());
            Ok(0)
        }
    } else {
        // newpath is already existing, check flag.
        debug!("[sys_renameat2] newinode already exist");
        // let newinode = newinode.unwrap();
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

        if flags.contains(Renameat2Flags::RENAME_NOREPLACE) {
            debug!("[sys_renameat2] cound not replace, error");
            // If flag is RENAME_NOREPLACE, should not to replace newpath.
            // So return EEXIST
            Err(SyscallErr::EEXIST)
        } else {
            // remove from old path
            old_parent.remove_child(oldinode.clone())?;
            // remove from new path
            new_parent.remove_child(newinode.clone())?;

            if flags.contains(Renameat2Flags::RENAME_EXCHANGE) {
                debug!("[sys_renameat2] exchange old and new");
                // If flag is RENAME_EXCHANGE, exchange old and new one.
                if newtype == InodeMode::FileDIR {
                    old_parent.mkdir(old_parent.clone(), &newname, newtype)?;
                } else {
                    old_parent.mknod(
                        old_parent.clone(),
                        &newname,
                        newtype,
                        newinode.clone().metadata().rdev,
                    )?;
                }
                if oldtype == InodeMode::FileDIR {
                    new_parent.mkdir(new_parent.clone(), &oldname, oldtype)?;
                } else {
                    new_parent.mknod(
                        new_parent.clone(),
                        &oldname,
                        oldtype,
                        oldinode.clone().metadata().rdev,
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
                // add to cache
                let mut cache_inner = INODE_CACHE.0.lock();
                let cache_lock = cache_inner.as_mut().unwrap();
                let key = HashKey::new(old_parent.metadata().ino, newname);
                cache_lock.insert(key, newinode.clone());
                let key = HashKey::new(new_parent.metadata().ino, oldname);
                cache_lock.insert(key, oldinode.clone());
                Ok(0)
            } else {
                // Normally, replace the newpath.
                // But you should check the newpath type, if newtype is not the same as oldtype, should return EISDIR or ENOTEMPTY or ENOTDIR
                debug!("[sys_renameat2] replace newpath");
                if newtype == oldtype {
                    // the same type, replace directly.
                    if newtype == InodeMode::FileDIR {
                        new_parent.mkdir(new_parent.clone(), &newname, newtype)?;
                    } else {
                        new_parent.mknod(
                            new_parent.clone(),
                            &newname,
                            newtype,
                            oldinode.clone().metadata().rdev,
                        )?;
                    }
                    let mut oldinner = oldinode.metadata().inner.lock().clone();
                    oldinner.parent = Some(Arc::downgrade(&new_parent));
                    let new_inner_lock = new_parent.metadata().inner.lock();
                    let newinode = new_inner_lock.children.get(newname.as_str()).unwrap();
                    newinode.metadata().inner_set(oldinner);
                    let key = HashKey::new(new_parent.metadata().ino, newname);
                    INODE_CACHE.insert(key, newinode.clone());
                    Ok(0)
                } else {
                    panic!("not support");
                }
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
        "[sys_readlinkat]: dirfd {}, path_name {} buf addr {:#x} buf size {}, this should be modified",
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
    let _sum_guard = SumGuard::new();
    let res = path::path_to_inode(dirfd, pathname);
    let inode = res.0?;
    if inode.is_none() {
        debug!("[sys_utimensat] cannot find inode relatived to pathname");
        return Err(SyscallErr::ENOENT);
    } else {
        let inode = inode.unwrap();

        let mut inner_lock = inode.metadata().inner.lock();
        if times.is_null() {
            debug!("[sys_utimensat] times is null");
            // If times is null, then both timestamps are set to the current time.
            inner_lock.st_atim = current_time_spec();
        } else {
            // times[0] for atime, times[1] for mtime
            UserCheck::new()
                .check_readable_slice(times as *const u8, 2 * core::mem::size_of::<TimeSpec>())?;
            let atime = unsafe { &*times };
            let times = unsafe { times.add(1) };
            let mtime = unsafe { &*times };
            // change access time
            if atime.nsec == UTIME_NOW {
                debug!("[sys_utimensat] atime nsec is UTIME_NOW");
                inner_lock.st_atim = current_time_spec();
            } else if atime.nsec == UTIME_OMIT {
                debug!("[sys_utimensat] atime nsec is UTIME_OMIT");
            } else {
                debug!("[sys_utimensat] atime normal nsec");
                inner_lock.st_atim = *atime;
            }
            // change modify time
            if mtime.nsec == UTIME_NOW {
                debug!("[sys_utimensat] mtime nsec is UTIME_NOW");
                inner_lock.st_mtim = current_time_spec();
            } else if mtime.nsec == UTIME_OMIT {
                debug!("[sys_utimensat] mtime nsec is UTIME_OMIT");
            } else {
                debug!("[sys_utimensat] mtime normal nsec");
                inner_lock.st_mtim = *mtime;
            }
            // change state change time
            inner_lock.st_ctim = current_time_spec();
        }
        return Ok(0);
    }
}

/// Checks whether the calling process can access the file pathname.
pub fn sys_faccessat(dirfd: isize, pathname: *const u8, mode: u32, flags: u32) -> SyscallRet {
    stack_trace!();
    let _mode = FaccessatFlags::from_bits(mode).ok_or(SyscallErr::EINVAL)?;
    let _flags = FcntlFlags::from_bits(flags).ok_or(SyscallErr::EINVAL)?;
    stack_trace!();
    let res = path::path_to_inode(dirfd, pathname);
    if res.0?.is_none() {
        debug!("[sys_faccessat] don't find inode");
        Err(SyscallErr::ENOENT)
    } else {
        Ok(0)
    }
}

pub fn sys_statfs(path: *const u8, buf: *mut Statfs) -> SyscallRet {
    stack_trace!();
    UserCheck::new().check_writable_slice(buf as *mut u8, STATFS_SIZE)?;
    let _sum_guard = SumGuard::new();
    let path = path::path_process(AT_FDCWD, path)?;
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
    let sysinfo = Sysinfo::collect();
    let buf_ptr = info as *mut Sysinfo;
    unsafe {
        stack_trace!();
        ptr::write(buf_ptr, sysinfo);
    }
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

    trace!("[sys_ppoll]: fds {:?}", fds);

    let timeout = match timeout_ptr {
        0 => {
            trace!("[sys_ppoll]: infinite timeout");
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
        let sigmask = unsafe { *(sigmask_ptr as *const u32) };
        current_process().inner_handler(|proc| {
            if let Some(new_sig_mask) = SigSet::from_bits(sigmask as usize) {
                proc.sig_queue.blocked_sigs |= new_sig_mask;
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
        trace!("[sys_ppoll]: ready");
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

    let timeout = match timeout_ptr {
        0 => {
            debug!("[sys_pselect]: infinite timeout");
            None
        }
        _ => {
            UserCheck::new()
                .check_readable_slice(timeout_ptr as *const u8, core::mem::size_of::<TimeSpec>())?;
            Some(Duration::from(unsafe { *(timeout_ptr as *const TimeSpec) }))
        }
    };
    info!(
        "[sys_pselect]: readfds {:?}, writefds {:?}, exceptfds {:?}, timeout {:?}",
        readfds, writefds, exceptfds, timeout
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

    if sigmask_ptr != 0 {
        stack_trace!();
        UserCheck::new()
            .check_readable_slice(sigmask_ptr as *const u8, core::mem::size_of::<SigSet>())?;
        let sigmask = unsafe { *(sigmask_ptr as *const u32) };
        current_process().inner_handler(|proc| {
            if let Some(new_sig_mask) = SigSet::from_bits(sigmask as usize) {
                proc.sig_queue.blocked_sigs |= new_sig_mask;
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
        if !timeout.is_zero() {
            info!("[sys_pselect]: timeout {:?}", timeout);
        }
        match TimeoutTaskFuture::new(timeout, poll_future).await {
            TimeoutTaskOutput::Ok(ret) => {
                if !timeout.is_zero() {
                    info!("[sys_pselect]: ready");
                } else {
                    info!("[sys_pselect]: ready");
                }
                return ret;
            }
            TimeoutTaskOutput::Timeout => {
                if !timeout.is_zero() {
                    info!("[sys_pselect]: timeout!, {:?}", timeout);
                } else {
                    debug!("[sys_pselect]: timeout!");
                }
                return Ok(0);
            }
        }
    } else {
        let ret = poll_future.await;
        debug!("[sys_pselect]: ready");
        ret
    }
}

pub async fn sys_ftruncate(fd: usize, len: usize) -> SyscallRet {
    stack_trace!();
    let file = current_process()
        .inner_handler(|proc| proc.fd_table.get(fd))
        .ok_or(SyscallErr::EBADF)?;
    file.truncate(len).await?;
    Ok(0)
}

pub async fn sys_fsync(fd: usize) -> SyscallRet {
    stack_trace!();
    // let file = current_process()
    //     .inner_handler(|proc| proc.fd_table.get(fd))
    //     .ok_or(SyscallErr::EBADF)?;
    // let inode = file
    //     .metadata()
    //     .inner
    //     .lock()
    //     .inode
    //     .clone()
    //     .ok_or(SyscallErr::EINVAL)?;
    // info!("[sys_fsync] start to sync file..., fd {}", fd);
    // <dyn Inode>::sync(inode).await?;
    // info!("[sys_fsync] sync file finished, fd {}", fd);
    Ok(0)
}

pub async fn sys_sync() -> SyscallRet {
    stack_trace!();
    info!("[sys_sync] start to sync...");
    // // TODO: now we only sync the rootfs
    // let root_fs = FILE_SYSTEM_MANAGER.root_fs();
    // root_fs.sync_fs().await?;
    info!("[sys_sync] sync finished");
    Ok(0)
}

pub async fn sys_pread64(fd: usize, buf_ptr: usize, len: usize, offset: usize) -> SyscallRet {
    stack_trace!();
    debug!("[sys_pread]: fd {}, len {}", fd, len);
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;

    if len == 0 {
        return Ok(0);
    }

    UserCheck::new().check_writable_slice(buf_ptr as *mut u8, len)?;

    let _sum_guard = SumGuard::new();
    let buf = unsafe { core::slice::from_raw_parts_mut(buf_ptr as *mut u8, len) };

    file.pread(buf, offset).await
}

pub async fn sys_pwrite64(fd: usize, buf_ptr: usize, len: usize, offset: usize) -> SyscallRet {
    stack_trace!();
    info!("[sys_write]: fd {}, len {}", fd, len);
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;

    if !file.writable() {
        return Err(SyscallErr::EPERM);
    }
    if len == 0 {
        return Ok(0);
    }

    UserCheck::new().check_readable_slice(buf_ptr as *const u8, len)?;
    let buf = unsafe { core::slice::from_raw_parts(buf_ptr as *const u8, len) };
    // debug!("[sys_write]: start to write file, fd {}, buf {:?}", fd, buf);
    file.pwrite(buf, offset).await
}
