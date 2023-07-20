//! Loading user applications into memory

/// Get the total number of applications.
use alloc::vec::Vec;
///get app number
pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}
/// get applications data
pub fn get_app_data(app_id: usize) -> &'static [u8] {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}

pub fn init() {
    let num_app = get_num_app();
    extern "C" {
        fn _app_names();
    }
    let mut start = _app_names as usize as *const u8;
    let mut v = Vec::new();
    unsafe {
        for _ in 0..num_app {
            let mut end = start;
            while end.read_volatile() != b'\0' {
                end = end.add(1);
            }
            let slice = core::slice::from_raw_parts(start, end as usize - start as usize);
            let str = core::str::from_utf8(slice).unwrap();
            v.push(str);
            start = end.add(1);
        }
        APP_NAMES = Some(v);
    }
}

static mut APP_NAMES: Option<Vec<&'static str>> = None;

#[allow(unused)]
///get app data from name
pub fn get_app_data_by_name(name: &str) -> Option<&'static [u8]> {
    // warn!("app name {}", name);
    let app_names = unsafe { APP_NAMES.as_ref().unwrap() };
    let num_app = get_num_app();
    (0..num_app)
        .find(|&i| app_names[i] == name)
        .map(get_app_data)
}
