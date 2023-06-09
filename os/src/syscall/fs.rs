//! File and filesystem-related syscalls
use core::ops::Add;
use core::ptr;
use core::ptr::copy_nonoverlapping;

use alloc::string::{String, ToString};
use alloc::sync::Arc;
use inode::InodeState;
use log::{debug, warn};

use crate::config::fs::RLIMIT_NOFILE;
use crate::fs::dirent::MAX_NAME_LEN;
use crate::fs::pipe::make_pipe;
use crate::fs::stat::{STAT, STAT_SIZE};
use crate::fs::{
    inode, Dirent, FileSystem, FileSystemType, Inode, InodeMode, Iovec, StatFlags, UtsName,
    DIRENT_SIZE, FILE_SYSTEM_MANAGER,
};
use crate::fs::{OpenFlags, UTSNAME_SIZE};
use crate::mm::user_check::UserCheck;
use crate::process::thread;
use crate::processor::{current_process, SumGuard};
use crate::signal::SigSet;
use crate::syscall::{MmapFlags, MmapProt, AT_FDCWD, SEEK_CUR, SEEK_END, SEEK_SET};
use crate::timer::get_time_spec;
use crate::timer::{get_time_ms, TimeSpec};
use crate::utils::error::{SyscallErr, SyscallRet};
use crate::utils::path::Path;
use crate::utils::string::{array_str_len, c_str_to_string};
use crate::{fs, stack_trace};

use super::PollFd;

// const FD_STDIN: usize = 0;
// const FD_STDOUT: usize = 1;
// bitflags! {
//     pub struct OpenFlags: u32 {
//         const O_RDONLY = 0;
//         const O_WRONLY = 1 << 0;
//         const O_RDWR = 1 << 1;
//         const O_CREATE = 1 << 9;
//         const O_TRUNC = 1 << 10;
//     }
// }

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
    let absolute_path: Option<String>;
    if dirfd == AT_FDCWD {
        debug!("path with cwd");
        absolute_path = Path::path_process(path);
    } else {
        debug!("path with dirfd");
        absolute_path = Path::path_with_dirfd(dirfd, path);
    }
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
    let absolute_path: Option<String>;
    if dirfd == AT_FDCWD {
        debug!("path with cwd");
        absolute_path = Path::path_process(pathname);
    } else {
        debug!("path with dirfd");
        absolute_path = Path::path_with_dirfd(dirfd, pathname);
    }
    match absolute_path {
        Some(absolute_path) => {
            debug!("absolute path: {}", absolute_path);
            let _find_inode = <dyn Inode>::lookup_from_root_tmp(&absolute_path);
            match _find_inode {
                Some(_find_inode) => Err(SyscallErr::EEXIST),
                None => {
                    let parent = Path::get_parent_dir(&absolute_path).unwrap();
                    let parent_inode = <dyn Inode>::lookup_from_root_tmp(&parent);
                    match parent_inode {
                        Some(parent_inode) => match parent_inode.metadata().mode {
                            InodeMode::FileDIR => {
                                let mut inner_lock = parent_inode.metadata().inner.lock();
                                // change the time
                                inner_lock.st_atim = get_time_spec();
                                inner_lock.st_mtim = get_time_spec();
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
                                parent_inode.mkdir(
                                    parent_inode.clone(),
                                    &Path::get_name(&absolute_path),
                                    InodeMode::FileDIR,
                                )?;
                                Ok(0)
                            }
                            _ => {
                                return Err(SyscallErr::ENOTDIR);
                            }
                        },
                        None => Err(SyscallErr::ENOENT),
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
    _flags: usize,
    _data: *const u8,
) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_c_str(dev_name)?;
    UserCheck::new().check_c_str(target_path)?;
    UserCheck::new().check_c_str(ftype)?;
    if _data as usize != 0 {
        UserCheck::new().check_c_str(_data)?;
    }
    // Check and convert the arguments.
    let dev_name = Path::path_process(dev_name);
    if dev_name.is_none() {
        return Err(SyscallErr::EMFILE);
    }
    // let dev_name = Path::get_name(&dev_name.unwrap());

    let target_path = Path::path_process(target_path);
    if target_path.is_none() {
        return Err(SyscallErr::ENOENT);
    }
    let target_path = target_path.unwrap();
    let target_inode = <dyn Inode>::lookup_from_root_tmp(&target_path);
    if target_inode.is_none() {
        return Err(SyscallErr::EACCES);
    }

    let ftype = Path::path_process(ftype);
    let ftype = {
        if ftype.is_some() {
            let ftype = ftype.unwrap();
            let ftype = FileSystemType::fs_type(ftype);
            if ftype.is_none() {
                return Err(SyscallErr::ENODEV);
            }
            ftype.unwrap()
        } else {
            FileSystemType::fs_type("vfat".to_string()).unwrap()
        }
    };

    // let parent = Path::get_parent_dir(&target_path);
    // let parent_inode = match parent {
    //     Some(parent) => <dyn Inode>::lookup_from_root_tmp(&parent),
    //     None => None,
    // };

    let mut fs = ftype.new_fs();
    fs.init(&target_path, ftype)?;
    fs.mount();

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
    let target_path = Path::path_process(target_path);
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
    target_fs.sync_fs()?;
    let meta = target_fs.metadata();
    let root_inode = meta.root_inode.unwrap();
    let parent = root_inode.metadata().inner.lock().parent.clone();
    match parent {
        Some(parent) => {
            let parent = parent.upgrade().unwrap();
            debug!("Have a parent: {}", parent.metadata().path);
            parent.remove_child(root_inode)?;
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
            match state {
                InodeState::Init => {
                    // load children from disk
                    <dyn Inode>::load_children(inode.clone());
                    inode.metadata().inner.lock().state = InodeState::Synced;
                }
                _ => {
                    // do nothing
                }
            }

            let _sum_guard = SumGuard::new();
            UserCheck::new().check_writable_slice(dirp as *mut u8, count)?;
            let mut inner_lock = inode.metadata().inner.lock();
            // change access time
            inner_lock.st_atim = get_time_spec();
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
            for dirent in dirents.iter() {
                stack_trace!();
                num_bytes += dirent.d_reclen as usize;
                if num_bytes > count {
                    debug!("[getdents] user buf size too small");
                    return Err(SyscallErr::EINVAL);
                }
                unsafe {
                    copy_nonoverlapping(&*dirent as *const Dirent, dirp_ptr as *mut Dirent, 1);
                    dirp_ptr += dirent.d_reclen as usize;
                }
            }
            file.metadata().inner.lock().dirent_index += dirents.len();

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
            inner_lock.st_atim = get_time_spec();
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

    if !file.readable() {
        return Err(SyscallErr::EACCES);
    }

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
    debug!("[_fstat] inode mode: {:?}", inode_meta.mode);
    kstat.st_blocks = (kstat.st_size / kstat.st_blksize as u64) as u64;
    let inner_lock = inode_meta.inner.lock();
    kstat.st_size = inner_lock.size as u64;
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
    let flags = StatFlags::from_bits(flags).ok_or(SyscallErr::EINVAL)?;
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_c_str(pathname)?;
    UserCheck::new().check_writable_slice(stat_buf as *mut u8, STAT_SIZE)?;
    stack_trace!();
    let absolute_path: Option<String>;
    if flags.contains(StatFlags::AT_SYMLINK_NOFOLLOW) {
        // todo: support symlink?
        debug!("the pathname represent a symbolic link");
    }
    if flags.contains(StatFlags::AT_EMPTY_PATH) {
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
        if dirfd == AT_FDCWD {
            debug!("[newfstatat] path with cwd");
            absolute_path = Path::path_process(pathname);
        } else {
            debug!("[newfstatat] path with dirfd");
            absolute_path = Path::path_with_dirfd(dirfd, pathname);
        }
    }
    debug!("[newfstatat] final absolute path: {:?}", absolute_path);

    let fd = _openat(absolute_path, OpenFlags::bits(&OpenFlags::RDONLY))?;

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

    let absolute_path: Option<String>;
    if dirfd == AT_FDCWD {
        debug!("path with cwd");
        absolute_path = Path::path_process(filename_addr);
    } else {
        debug!("path with dirfd");
        absolute_path = Path::path_with_dirfd(dirfd, filename_addr);
    }

    _openat(absolute_path, flags)
}

/// We should give the absolute path (Or None) to _openat function.
fn _openat(absolute_path: Option<String>, flags: u32) -> SyscallRet {
    stack_trace!();
    debug!(
        "[_openat] absolute path: {:?}, flags: {}",
        absolute_path, flags
    );
    let flags = OpenFlags::from_bits(flags).ok_or(SyscallErr::EINVAL)?;
    match absolute_path {
        Some(absolute_path) => {
            debug!("[_openat] file name {}", absolute_path);
            if let Some(inode) = fs::inode::open_file(&absolute_path, flags) {
                stack_trace!();
                let mut inner_lock = inode.metadata().inner.lock();
                inner_lock.st_atim = get_time_spec();
                match inner_lock.state {
                    InodeState::Synced => {
                        inner_lock.state = InodeState::DirtyInode;
                    }
                    _ => {}
                }
                debug!(
                    "[_openat] inode ino: {}, name: {}",
                    inode.metadata().ino,
                    inode.metadata().name
                );
                // TODO: add to fs's dirty list
                let fd = current_process().inner_handler(|proc| {
                    let fd = proc.fd_table.alloc_fd();
                    let file = inode.open(inode.clone(), flags)?;

                    // // just for debug
                    // let file2 = inode.open(inode.clone(), flags)?;
                    // let buf: [u8; 3] = [1; 3];
                    // file2.sync_write(&buf)?;
                    // // just for debug
                    // let file2 = inode.open(inode.clone(), flags)?;
                    // file2.seek(0)?;
                    // let mut buf: [u8; 3] = [1; 3];
                    // file2.sync_read(&mut buf)?;

                    proc.fd_table.put(fd, file);
                    Ok(fd)
                })?;
                debug!("[_openat] find fd: {}", fd);
                Ok(fd as isize)
            } else {
                debug!("file {} doesn't exist", absolute_path);
                Err(SyscallErr::EACCES)
            }
        }
        None => {
            debug!("cannot find the file, absolute_path is none");
            Err(SyscallErr::ENOENT)
        }
    }
}

pub fn sys_close(fd: usize) -> SyscallRet {
    stack_trace!();
    let process = current_process();
    return process.inner_handler(move |proc| {
        if proc.fd_table.take(fd).is_none() {
            Err(SyscallErr::EBADF)
        } else {
            debug!("close fd {}", fd);
            Ok(0)
        }
    });
}

pub async fn sys_write(fd: usize, buf: usize, len: usize) -> SyscallRet {
    stack_trace!();
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;

    if !file.writable() {
        return Err(SyscallErr::EPERM);
    }

    UserCheck::new().check_readable_slice(buf as *const u8, len)?;
    // debug!("check readable slice sva {:#x} {:#x}", buf as *const u8 as usize, buf as *const u8 as usize + len);
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };
    file.write(buf).await
}

pub async fn sys_writev(fd: usize, iov: usize, iovcnt: usize) -> SyscallRet {
    stack_trace!();
    debug!("start writev, fd: {}, iov: {}, iovcnt:{}", fd, iov, iovcnt);
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;
    if !file.writable() {
        return Err(SyscallErr::EPERM);
    }

    stack_trace!();
    let mut ret: usize = 0;
    let iovec_size = core::mem::size_of::<Iovec>();

    let _sum_guard = SumGuard::new();

    for i in 0..iovcnt {
        debug!("write the {} buf", i + 1);
        // current iovec pointer
        let current = iov.add(iovec_size * i);
        debug!("current iov: {}", current);
        UserCheck::new().check_readable_slice(current as *const u8, iovec_size)?;
        debug!("pass readable check");
        let iov_base = unsafe { &*(current as *const Iovec) }.iov_base;
        debug!("get iov_base: {}", iov_base);
        let iov_len = unsafe { &*(current as *const Iovec) }.iov_len;
        debug!("get iov_len: {}", iov_len);
        ret += iov_len;
        UserCheck::new().check_readable_slice(iov_base as *const u8, iov_len)?;
        let buf = unsafe { core::slice::from_raw_parts(iov_base as *const u8, iov_len) };
        let buf_str = unsafe { core::str::from_utf8_unchecked(buf) };
        debug!("[writev] buf: {:?}", buf_str);
        let fw_ret = file.write(buf).await;
        // if error, return
        if fw_ret.is_err() {
            return fw_ret;
        }
    }
    Ok(ret as isize)
}

pub async fn sys_read(fd: usize, buf: usize, len: usize) -> SyscallRet {
    stack_trace!();
    let file = current_process()
        .inner_handler(move |proc| proc.fd_table.get_ref(fd).cloned())
        .ok_or(SyscallErr::EBADF)?;

    if !file.readable() {
        return Err(SyscallErr::EPERM);
    }

    // file.meta_data()
    //     .inode
    //     .lock()
    //     .as_ref()
    //     .unwrap()
    //     .meta_data()
    //     .inner
    //     .lock()
    //     .st_atime = get_time() as i64;

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

pub fn sys_fcntl(fd: usize, cmd: i32, arg: usize) -> SyscallRet {
    stack_trace!();
    debug!("[sys_fcntl]: fd {}, cmd {:#x}, arg {:#x}", fd, cmd, arg);
    // TODO
    Ok(0)
}

bitflags! {
    pub struct PollEvents: u16 {
        const POLLIN = 1 << 0;
        const POLLPRI = 1 << 1;
        const POLLOUT = 1 << 2;
        const POLLERR = 1 << 3;
        const POLLHUP = 1 << 4;
        const POLLNVAL = 1 << 5;
    }
}

pub async fn sys_ppoll(fds: usize, nfds: usize, timeout_ptr: usize, sigmask: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();

    stack_trace!();
    UserCheck::new().check_writable_slice(fds as *mut u8, core::mem::size_of::<PollFd>() * nfds)?;
    let fds: &mut [PollFd] = unsafe { core::slice::from_raw_parts_mut(fds as *mut PollFd, nfds) };

    let start_ms = get_time_ms();
    let infinite_timeout: bool;
    let timeout: usize;
    if timeout_ptr == 0 {
        debug!("[sys_ppoll]: infinite timeout");
        infinite_timeout = true;
        timeout = 0;
    } else {
        infinite_timeout = false;
        stack_trace!();
        UserCheck::new()
            .check_readable_slice(timeout_ptr as *const u8, core::mem::size_of::<TimeSpec>())?;
        let timeout_delta = unsafe { *(timeout_ptr as *const TimeSpec) };
        // if timeout_delta < 0 {
        //     warn!("invalid timeout");
        //     return Err(SyscallErr::EINVAL);
        // }
        timeout = timeout_delta.sec * 1000 + timeout_delta.nsec / 1000000;
    }
    let expire_time = start_ms + timeout;

    if sigmask != 0 {
        stack_trace!();
        UserCheck::new()
            .check_readable_slice(sigmask as *const u8, core::mem::size_of::<SigSet>())?;
        let sigmask = unsafe { *(sigmask as *const usize) };
        current_process().inner_handler(|proc| {
            if let Some(new_sig_mask) = SigSet::from_bits(sigmask) {
                proc.pending_sigs.blocked_sigs |= new_sig_mask;
            } else {
                warn!("invalid set arg");
            }
        });
    }

    loop {
        // TODO: how to avoid user modify the address?
        let mut cnt = 0;
        for i in 0..nfds {
            let current_fd = &mut fds[i];
            debug!("[sys_ppoll]: poll fd {}", current_fd.fd);
            if let Some(file) =
                current_process().inner_handler(|proc| proc.fd_table.get(current_fd.fd as usize))
            {
                if let Some(events) = PollEvents::from_bits(current_fd.events as u16) {
                    current_fd.revents = 0;
                    if events.contains(PollEvents::POLLIN) {
                        // file.read()
                        if file.pollin()? {
                            current_fd.revents |= PollEvents::POLLIN.bits() as i16;
                            cnt += 1;
                            continue;
                        }
                    }
                    if events.contains(PollEvents::POLLOUT) {
                        // file.read()
                        if file.pollout()? {
                            current_fd.revents |= PollEvents::POLLOUT.bits() as i16;
                            cnt += 1;
                            continue;
                        }
                    }
                } else {
                    warn!("Invalid events: {:#x}", current_fd.events);
                    // TODO: not sure
                    return Err(SyscallErr::EINVAL);
                }
            } else {
                debug!("No such file for fd {}", current_fd.fd);
                continue;
            }
        }
        if cnt > 0 {
            return Ok(cnt as isize);
        } else if !infinite_timeout && get_time_ms() >= expire_time {
            debug!("[sys_ppoll]: timeout!");
            return Ok(0);
        } else {
            thread::yield_now().await;
        }
    }
}
