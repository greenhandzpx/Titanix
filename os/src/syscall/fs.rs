//! File and filesystem-related syscalls
use core::mem::size_of_val;
use core::ptr;
use core::ptr::copy_nonoverlapping;

use alloc::string::{String, ToString};
use alloc::sync::Arc;
use log::{debug, warn};

use crate::config::fs::RLIMIT_NOFILE;
use crate::fs::inode::INODE_CACHE;
use crate::fs::kstat::{KSTAT, KSTAT_SIZE};
use crate::fs::pipe::make_pipe;
use crate::fs::{
    dirent, inode, Dirent, FAT32FileSystem, File, FileSystem, FileSystemType, Inode, InodeMode,
    UtsName, DIRENT_SIZE, FILE_SYSTEM_MANAGER,
};
use crate::fs::{OpenFlags, UTSNAME_SIZE};
use crate::mm::memory_set::page_fault_handler::MmapPageFaultHandler;
use crate::mm::memory_set::vm_area::BackupFile;
use crate::mm::memory_set::{PageFaultHandler, VmArea};
use crate::mm::user_check::UserCheck;
use crate::mm::{MapPermission, VirtAddr};
use crate::processor::{current_process, SumGuard};
use crate::syscall::{MmapFlags, MmapProt, AT_FDCWD};
use crate::timer::get_time_ms;
use crate::utils::debug;
use crate::utils::error::{SyscallErr, SyscallRet};
use crate::utils::path::Path;
use crate::utils::string::c_str_to_string;
use crate::{fs, stack_trace};

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
                                parent_inode.metadata().inner.lock().st_atime =
                                    (get_time_ms() / 1000) as i64;
                                parent_inode.metadata().inner.lock().st_mtime =
                                    (get_time_ms() / 1000) as i64;
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
    let inode = file.metadata().inner.lock().inode.clone();
    match inode {
        Some(inode) => {
            let _sum_guard = SumGuard::new();
            UserCheck::new().check_writable_slice(dirp as *mut u8, count)?;
            inode.metadata().inner.lock().st_atime = (get_time_ms() / 1000) as i64;
            let dirents = Dirent::get_dirents(inode);
            let num_bytes = DIRENT_SIZE as usize * dirents.len();
            debug!(
                "count: {}, num_bytes: {}, dirents len: {}",
                count,
                num_bytes,
                dirents.len(),
            );
            // TODO: Actually, we should check the size of the dirp, but it is too small, so we don't check to pass the test.
            if count < num_bytes {
                Err(SyscallErr::EINVAL)
            } else {
                let mut dirp_ptr = dirp as *mut Dirent;
                for dirent in dirents {
                    debug!("dirent: {:?}", dirent);
                    stack_trace!();
                    unsafe {
                        copy_nonoverlapping(&dirent as *const Dirent, dirp_ptr, 1);
                        dirp_ptr = dirp_ptr.offset(1);
                    }
                }
                Ok(num_bytes as isize)
            }
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
            target_inode.metadata().inner.lock().st_atime = (get_time_ms() / 1000) as i64;
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

/// fstat() function return information about a file, in the buffer pointed to by kst.
/// This function except that the file about which information is to be retrieved is specified by the file descriptor fd.
pub fn sys_fstat(fd: usize, kst: usize) -> SyscallRet {
    stack_trace!();
    let _sum_guard = SumGuard::new();
    UserCheck::new().check_writable_slice(kst as *mut u8, KSTAT_SIZE)?;
    let mut kstat = KSTAT::new();
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
    // kstat.st_ino = inode_meta.ino as u64;
    kstat.st_ino = 1 as u64;
    kstat.st_mode = inode_meta.mode as u32;
    kstat.st_size = inode_meta.inner.lock().size as u64;
    kstat.st_blocks = (kstat.st_size / kstat.st_blsize as u64) as u64;
    kstat.st_atime_sec = inode_meta.inner.lock().st_atime;
    kstat.st_atime_nsec = kstat.st_atime_sec * 10i64.pow(9);
    kstat.st_mtime_sec = inode_meta.inner.lock().st_mtime;
    kstat.st_mtime_nsec = kstat.st_mtime_sec * 10i64.pow(9);
    kstat.st_ctime_sec = inode_meta.inner.lock().st_ctime;
    kstat.st_ctime_nsec = kstat.st_ctime_sec * 10i64.pow(9);

    let kst_ptr = kst as *mut KSTAT;
    unsafe {
        ptr::write(kst_ptr, kstat);
    }
    Ok(0)
}

pub fn sys_openat(dirfd: usize, filename_addr: *const u8, flags: u32, _mode: u32) -> SyscallRet {
    stack_trace!();

    let absolute_path: Option<String>;
    if dirfd as isize == AT_FDCWD {
        debug!("path with cwd");
        absolute_path = Path::path_process(filename_addr);
    } else {
        debug!("path with dirfd");
        absolute_path = Path::path_with_dirfd(dirfd as isize, filename_addr);
    }

    stack_trace!();
    // TODO: support standard sys_openat(we now only support `sys_open`)
    let flags = OpenFlags::from_bits(flags).ok_or(SyscallErr::EINVAL)?;
    match absolute_path {
        Some(absolute_path) => {
            debug!("file name {}", absolute_path);
            if let Some(inode) = fs::inode::open_file(&absolute_path, flags) {
                inode.metadata().inner.lock().st_atime = (get_time_ms() / 1000) as i64;
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
                Ok(fd as isize)
            } else {
                debug!("file {} doesn't exist", absolute_path);
                Err(SyscallErr::EACCES)
            }
        }
        None => Err(SyscallErr::ENOENT),
    }

    // _openat(dirfd, filename_addr, flags, _mode)
    // let filename = c_str_to_string(filename_addr);
    // let mut filename = String::from(&filename);
    // debug!("file name {}", filename);
    // if filename.starts_with("./") {
    //     filename.remove(0);
    //     filename.remove(0);
    // } else if filename == "." {
    //     return _openat(dirfd, filename_addr, flags, _mode);
    // }
    // if let Some(inode) = fs::fat32_tmp::open_file(
    //     &filename,
    //     OpenFlags::from_bits(flags).unwrap(),
    // ) {
    //     let fd = current_process().inner_handler(|proc| {
    //         let fd = proc.fd_table.alloc_fd();
    //         proc.fd_table.put(fd, inode);
    //         fd
    //     });
    //     Ok(fd as isize)
    // } else {
    //     debug!("file {} doesn't exist", filename);
    //     Err(SyscallErr::EACCES)
    // }
}

// vfs version
pub fn _openat(dirfd: usize, filename: *const u8, flags: u32, _mode: u32) -> SyscallRet {
    stack_trace!();
    // TODO: support standard sys_openat(we now only support `sys_open`)
    let flags = OpenFlags::from_bits(flags).ok_or(SyscallErr::EINVAL)?;
    let filename = Path::path_process(filename);
    // if dirfd as isize != AT_FDCWD {
    //     if current_process().inner_handler(|proc| {
    //         if let Some(file) = proc.fd_table.get_ref(dirfd) {
    //             file.metadata().
    //         }
    //     })
    // }
    match filename {
        Some(filename) => {
            debug!("file name {}", filename);
            if let Some(inode) = fs::inode::open_file(&filename, flags) {
                inode.metadata().inner.lock().st_atime = (get_time_ms() / 1000) as i64;
                let fd = current_process().inner_handler(|proc| {
                    let fd = proc.fd_table.alloc_fd();
                    let file = inode.open(inode.clone(), flags)?;
                    debug!("[openat]: fd {}", fd);
                    proc.fd_table.put(fd, file);
                    Ok(fd)
                })?;
                Ok(fd as isize)
            } else {
                debug!("file {} doesn't exist", filename);
                Err(SyscallErr::EACCES)
            }
        }
        None => Err(SyscallErr::ENOENT),
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

    // file.meta_data()
    //     .inode
    //     .lock()
    //     .as_ref()
    //     .unwrap()
    //     .meta_data()
    //     .inner
    //     .lock()
    //     .st_atime = get_time() as i64;

    UserCheck::new().check_readable_slice(buf as *const u8, len)?;
    // debug!("check readable slice sva {:#x} {:#x}", buf as *const u8 as usize, buf as *const u8 as usize + len);
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };
    file.write(buf).await
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
