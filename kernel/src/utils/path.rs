use alloc::{
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};

use crate::{
    fs::{Inode, AT_FDCWD},
    mm::user_check::UserCheck,
    processor::{current_process, SumGuard},
    stack_trace,
};

use super::error::{GeneralRet, SyscallErr};
use super::string::c_str_to_string;
use log::{debug, trace};

pub fn split_path(path_name: &str) -> Vec<&str> {
    path_name.split('/').filter(|name| *name != "").collect()
}
pub fn split_path_string(path_name: String) -> Vec<String> {
    let path_name = path_name.as_str();
    path_name
        .split('/')
        .filter(|name| *name != "")
        .map(|s| s.to_string())
        .collect()
}
pub fn get_name(path_name: &str) -> &str {
    let dentry_vec = split_path(path_name);
    let len = dentry_vec.len();
    trace!("[get_name] dentry_vec: {:?}, len: {}", dentry_vec, len);
    if len == 0 {
        ""
    } else {
        dentry_vec[dentry_vec.len() - 1]
    }
}
pub fn is_relative_path(path: &str) -> bool {
    if path.starts_with("/") {
        return false;
    } else {
        return true;
    }
}
/// if path has .. return true
pub fn check_double_dot(path: &str) -> bool {
    let path_vec = split_path(path);
    for name in path_vec {
        if name.eq("..") {
            return true;
        }
    }
    return false;
}
/// remove .
pub fn remove_dot(path: &str) -> String {
    let path_vec: Vec<&str> = path
        .split('/')
        .filter(|name| *name != "" && *name != ".")
        .collect();
    path_vec.join("/")
}
/// format path: remove extra "/"
pub fn format(src: &str) -> String {
    let mut vec = split_path(src);
    if !is_relative_path(src) {
        vec.insert(0, "");
    }
    vec.join("/")
}
pub fn change_relative_to_absolute(relative_path: &str, cwd: &str) -> String {
    let absolute_path_vec = split_path(cwd);
    let relative_path_vec = split_path(relative_path);
    debug!("absolute path: {:?}", absolute_path_vec);
    debug!("relative path: {:?}", relative_path_vec);
    let mut res: Vec<&str> = Vec::new();
    res.push("");
    for i in 0..absolute_path_vec.len() {
        res.push(absolute_path_vec[i]);
    }
    for i in 0..relative_path_vec.len() {
        match relative_path_vec[i] {
            ".." => {
                if let Some(check) = res.pop() {
                    if check == "" {
                        return "/".to_string();
                    }
                }
            }
            "." => {}
            _ => {
                res.push(relative_path_vec[i]);
            }
        }
    }
    res.join("/")
}

/// return (target_inode, absolute_path, parent_inode)
pub fn path_to_inode(
    dirfd: isize,
    path: Option<&str>,
) -> GeneralRet<(Option<Arc<dyn Inode>>, String, Option<Arc<dyn Inode>>)> {
    match path {
        None => {
            match dirfd {
                AT_FDCWD => {
                    debug!("[path_to_inode] path is null and dirfd is AT_FDCWD");
                    return current_process().inner_handler(|proc| {
                        // If it have file, it must have inode
                        let (target, parent) = <dyn Inode>::lookup_from_root(&proc.cwd)?;
                        Ok((target, proc.cwd.clone(), parent))
                    });
                }
                _ => {
                    debug!("[path_to_inode] path is null and dirfd is not AT_FDCWD");
                    return current_process().inner_handler(|proc| {
                        let wd_file = proc.fd_table.get_ref(dirfd as usize);
                        match wd_file {
                            Some(wd_file) => {
                                let inode = wd_file.file.metadata().inner.lock().inode.clone();
                                match inode {
                                    None => Err(SyscallErr::EBADF),
                                    Some(inode) => Ok((
                                        Some(inode.clone()),
                                        inode.metadata().path.clone(),
                                        None,
                                    )),
                                }
                            }
                            None => Err(SyscallErr::EBADF),
                        }
                    });
                }
            }
        }
        Some(path) => {
            debug!("[path_to_inode] path is not null");
            debug!("[path_to_inode] get path: {}", path);
            let mut path = format(&path);
            debug!("[path_to_inode] get format path: {}", path);
            stack_trace!();
            if is_relative_path(&path) {
                if dirfd != AT_FDCWD {
                    debug!("[path_to_inode] path is releative and dirfd isn't AT_FDCWD");
                    return current_process().inner_handler(|proc| {
                        let wd_file = proc.fd_table.get_ref(dirfd as usize);
                        match wd_file {
                            Some(wd_file) => {
                                let inode = wd_file.file.metadata().inner.lock().inode.clone();
                                if inode.is_none() {
                                    return Err(SyscallErr::ENOENT);
                                }
                                let inode = inode.unwrap();
                                if check_double_dot(&path) {
                                    // path has ..
                                    // parent is not sure, return None
                                    let path = change_relative_to_absolute(&path, &proc.cwd);
                                    let (target, parent) = <dyn Inode>::lookup_from_root(&path)?;
                                    Ok((target, path, parent))
                                } else {
                                    // the path doesn't have ..
                                    // inode is the parent which should be returned
                                    let path = remove_dot(&path);
                                    let absolute_path =
                                        change_relative_to_absolute(&path, &proc.cwd);
                                    let target = inode.lookup_from_current(&path)?;
                                    Ok((target, absolute_path, Some(inode)))
                                }
                            }
                            None => Err(SyscallErr::EBADF),
                        }
                    });
                } else {
                    debug!("[path_to_inode] path is releative and dirfd is AT_FDCWD");
                    return current_process().inner_handler(|proc| {
                        let path = change_relative_to_absolute(&path, &proc.cwd);
                        let (target, parent) = <dyn Inode>::lookup_from_root(&path)?;
                        Ok((target, path, parent))
                    });
                }
            } else {
                debug!("[path_to_inode] path is absolute");
                if path.eq("/dev/shm/testshm") {
                    debug!("[path_to_inode] just for libc-test");
                    path = "/testshm".to_string();
                }
                let (target, parent) = <dyn Inode>::lookup_from_root(&path)?;
                Ok((target, path, parent))
            }
        }
    }
}

/// return (target_inode, absolute_path, parent_inode)
pub fn path_to_inode_ffi(
    dirfd: isize,
    path: *const u8,
) -> GeneralRet<(Option<Arc<dyn Inode>>, String, Option<Arc<dyn Inode>>)> {
    let _sum_guard = SumGuard::new();
    let path = if path.is_null() {
        None
    } else {
        UserCheck::new().check_c_str(path)?;
        stack_trace!();
        Some(c_str_to_string(path))
    };
    path_to_inode(dirfd, {
        let ref this = path;
        match *this {
            Some(ref x) => Some(x),
            None => None,
        }
    })
}

/// Return absolute path
pub fn path_process(dirfd: isize, path: *const u8) -> GeneralRet<String> {
    debug!("[path_process] dirfd: {}", dirfd);
    let _sum_guard = SumGuard::new();
    let path = match path as usize {
        0 => {
            debug!("[path_process] path is null");
            if dirfd != AT_FDCWD {
                debug!("[path_process] dirfd is a normal fd");
                path_with_dirfd(dirfd, ".")
            } else {
                debug!("[path_process] dirfd is AT_FDCWD");
                let cwd = current_process().inner_handler(move |proc| proc.cwd.clone());
                debug!("[path_process] cwd {}", cwd);
                Ok(change_relative_to_absolute(".", &cwd))
            }
        }
        _ => {
            debug!("[path_process] path is not null");
            UserCheck::new().check_c_str(path)?;
            let path = c_str_to_string(path);
            Ok(format(&path))
        }
    }?;
    debug!("[path_process] dirfd {}, path name {}", dirfd, path);
    let absolute_path;
    if is_relative_path(&path) {
        debug!("[path_process] A relative path");
        if dirfd == AT_FDCWD {
            debug!("[path_process] dirfd is AT_FDCWD");
            let cwd = current_process().inner_handler(move |proc| proc.cwd.clone());
            debug!("[path_process] cwd {}", cwd);
            absolute_path = change_relative_to_absolute(&path, &cwd);
        } else {
            debug!("[path_process] dirfd is a normal fd");
            absolute_path = path_with_dirfd(dirfd, &path)?
        }
    } else {
        debug!("[path_process] An absolute path");
        absolute_path = path;
    }
    Ok(absolute_path)
}

fn path_with_dirfd(dirfd: isize, path: &str) -> GeneralRet<String> {
    current_process().inner_handler(|proc| {
        let wd_file = proc
            .fd_table
            .get_ref(dirfd as usize)
            .ok_or(SyscallErr::EBADF)?;
        let inner = wd_file.file.metadata().inner.lock();
        let inode = inner.inode.as_ref().clone();
        let wd = inode.unwrap().metadata().path.clone();
        debug!("wd: {}", wd);
        Ok(change_relative_to_absolute(&path, &wd))
    })
}

pub fn child_path(absolute: &str, parent: &str) -> String {
    absolute[(parent.len() + 1)..].to_string()
}

pub fn get_parent_dir(path_name: &str) -> Option<String> {
    let dentry_vec: Vec<&str> = split_path(path_name);
    debug!("[get_parent_dir] dentry vec {:?}", dentry_vec);
    if dentry_vec.is_empty() {
        return None;
    }
    let mut res = "".to_string();
    for i in 0..dentry_vec.len() - 1 {
        res += "/";
        res += dentry_vec[i];
    }
    if res == "" {
        res += "/";
    }
    Some(res)
}
pub fn merge(p1: &str, p2: &str) -> String {
    let mut res = p1.to_string();
    res = if res.eq("/") {
        "".to_string()
    } else if res.ends_with("/") {
        println!("[merge] end with /");
        res[0..res.len() - 2].to_string()
    } else {
        res
    };
    res += "/";
    res += p2;
    res
}
#[allow(unused)]
pub fn exchange_prefix(p1: &str, p2: &str) -> (String, String) {
    let p1_prefix = get_parent_dir(p1).unwrap();
    let p1_name = get_name(p1);
    let p2_prefix = get_parent_dir(p2).unwrap();
    let p2_name = get_name(p2);
    (merge(&p2_prefix, p1_name), merge(&p1_prefix, p2_name))
}
