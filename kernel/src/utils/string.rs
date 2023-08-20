use alloc::{string::String, vec::Vec};

use crate::{fs::ffi::MAX_NAME_LEN, processor::SumGuard};

/// Convert C-style string(end with '\0') to rust string
pub fn c_str_to_string(ptr: *const u8) -> String {
    let _sum_guard = SumGuard::new();
    let mut ptr = ptr as usize;
    let mut ret = String::new();
    loop {
        let ch: u8 = unsafe { *(ptr as *const u8) };
        if ch == 0 {
            break;
        }
        ret.push(ch as char);
        ptr += 1;
    }
    ret
}

pub fn string_to_array(s: String) -> [u8; MAX_NAME_LEN] {
    let mut bytes = s.into_bytes();
    bytes.resize(MAX_NAME_LEN, 0u8);
    bytes.try_into().unwrap()
}

#[allow(unused)]
pub fn string_to_vec(s: String) -> Vec<u8> {
    let mut bytes = s.into_bytes();
    let len = bytes.len();
    bytes.resize(len + 1, 0u8);
    bytes
}

pub fn str_to_array_65(s: &str) -> [u8; 65] {
    let mut bytes = s.as_bytes().to_vec();
    bytes.resize(65, 0u8);
    bytes.try_into().unwrap()
}

pub fn array_str_len(s: &[u8]) -> usize {
    let mut len = 0;
    for i in 0..s.len() {
        len += 1;
        if s[i] == 0 {
            return len;
        }
    }
    len
}
