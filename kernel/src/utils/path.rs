use alloc::{
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};

use crate::{
    fs::{FileSystem, Inode},
    processor::current_process,
};

use super::error::{GeneralRet, SyscallErr};
use super::string::c_str_to_string;
use log::{debug, info};

// use crate::fs::Dentry;

pub const AT_FDCWD: isize = -100;

pub struct Path {
    // pub mnt: Arc<VfsMount>,
    // pub dentry: Arc<dyn Dentry>,
}

impl Path {
    pub fn path2vec(path_name: &str) -> Vec<&str> {
        path_name.split('/').filter(|name| *name != "").collect()
    }
    pub fn get_name(path_name: &str) -> &str {
        let dentry_vec: Vec<&str> = path_name.split('/').collect();
        dentry_vec[dentry_vec.len() - 1]
    }
    pub fn judge_is_relative(path: &str) -> bool {
        if path.starts_with("/") {
            return false;
        } else {
            return true;
        }
    }
    pub fn change_relative_to_absolute(relative_path: &str, cwd: &str) -> Option<String> {
        let absolute_path_vec = Self::path2vec(cwd);
        let relative_path_vec = Self::path2vec(relative_path);
        debug!("absolute path: {:?}", absolute_path_vec);
        debug!("relative path: {:?}", relative_path_vec);
        let mut res: Vec<&str> = Vec::new();
        if absolute_path_vec.len() == 0 {
            res.push("");
        } else {
            for i in 0..absolute_path_vec.len() {
                res.push(absolute_path_vec[i]);
            }
        }
        for i in 0..relative_path_vec.len() {
            match relative_path_vec[i] {
                ".." => {
                    if let Some(check) = res.pop() {
                        if check == "" {
                            return None;
                        }
                    }
                }
                "." => {}
                _ => {
                    res.push(relative_path_vec[i]);
                }
            }
        }
        Some(res.join("/"))
    }
    pub fn path_process(dirfd: isize, path: *const u8) -> Option<String> {
        let path_str = &c_str_to_string(path);
        debug!("[path_process] dirfd {}, path name {}", dirfd, path_str);
        let absolute_path;
        if Self::judge_is_relative(path_str) {
            debug!("[path_process] It is a relative path");
            if dirfd == AT_FDCWD {
                debug!("[path_process] dirfd is AT_FDCWD");
                let cwd = current_process().inner_handler(move |proc| proc.cwd.clone());
                debug!("[path_process] cwd {}", cwd);
                absolute_path = Self::change_relative_to_absolute(path_str, &cwd);
            } else {
                debug!("[path_process] dirfd is a normal fd");
                absolute_path = Self::path_with_dirfd(dirfd, path);
            }
        } else {
            debug!("[path_process] It is a absolute path");
            absolute_path = Some(path_str.clone());
        }
        absolute_path
    }
    pub fn path_with_dirfd(dirfd: isize, path: *const u8) -> Option<String> {
        let path = &c_str_to_string(path);
        let absolute_path = current_process().inner_handler(|proc| {
            let wd_inode = proc.fd_table.get_ref(dirfd as usize);
            match wd_inode {
                Some(wd_inode) => {
                    let wd = wd_inode.metadata().path.clone();
                    debug!("wd: {}", wd);
                    Self::change_relative_to_absolute(path, &wd)
                }
                None => None,
            }
        });
        absolute_path
    }
    pub fn user_path(file_system: Arc<dyn FileSystem>, path: &str) -> GeneralRet<Arc<dyn Inode>> {
        // need to find the dentry which is associated with this path
        // should call d_lookup_from_root_tmp() to get the dentry
        // also should prepare the filesystem (init it first)
        let path_vec = Path::path2vec(path);
        if path_vec[0].starts_with('/') {
            let mut target = <dyn Inode>::lookup_from_root_tmp(path);
            match target {
                Some(target) => Ok(target),
                None => Err(SyscallErr::ENOENT),
            }
        } else {
            Err(SyscallErr::ENOENT)
        }
    }
    pub fn get_parent_dir(path_name: &str) -> Option<String> {
        let dentry_vec: Vec<&str> = Self::path2vec(path_name);
        debug!("dentry vec {:?}", dentry_vec);
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
}
