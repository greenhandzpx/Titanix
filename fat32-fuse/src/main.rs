use std::env;
use std::fs;
use std::fs::{read_dir, File, OpenOptions};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};

use clap::ArgMatches;
use clap::{App, Arg};
use fatfs::Dir;
use fatfs::{format_volume, FormatVolumeOptions, StdIoWrapper};
use fatfs::{FileSystem, FsOptions};
use fscommon::BufStream;

fn mkfs(filename: String) -> io::Result<()> {
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&filename)?;
    let buf_file = BufStream::new(file);
    format_volume(
        &mut StdIoWrapper::from(buf_file),
        FormatVolumeOptions::new().fat_type(fatfs::FatType::Fat32),
    )?;
    Ok(())
}

fn pack_elfs(matches: ArgMatches, filename: String) -> io::Result<()> {
    // let src_path = matches.value_of("source").unwrap();
    let target_path = matches.value_of("target").unwrap();
    let target_path2 = matches.value_of("target2").unwrap();
    let src_path = matches.value_of("source").unwrap();
    println!("src_path = {}\ntarget_path = {}", src_path, target_path);

    let img_file = match OpenOptions::new().read(true).write(true).open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open image!");
            return Err(err);
        }
    };
    let buf_stream = BufStream::new(img_file);
    let options = FsOptions::new().update_accessed_date(true);
    let fs = FileSystem::new(buf_stream, options)?;

    // Write preliminary tests
    let apps_pre: Vec<_> = read_dir(target_path)
        .unwrap()
        .into_iter()
        .map(|dir_entry| dir_entry.unwrap().file_name().into_string().unwrap())
        .filter(|name| *name != "mnt" && *name != "fs.img")
        .collect();
    let pre_dir = fs.root_dir().create_dir("preliminary")?;
    for app in apps_pre {
        // load app data from host file system
        let mut host_file = File::open(format!("{}{}", target_path, app)).unwrap();
        let mut all_data: Vec<u8> = Vec::new();
        host_file.read_to_end(&mut all_data).unwrap();
        // create a file in fat-fs
        let mut file = pre_dir.create_file(&app)?;
        // write data to fat-fs
        file.write_all(&all_data)?;
    }

    // Write busybox && lua tests && lmbench tests
    let busybox_path = "../testcases/busybox/";
    let apps_busybox: Vec<_> = read_dir(busybox_path)
        .unwrap()
        .into_iter()
        .map(|dir_entry| dir_entry.unwrap().file_name().into_string().unwrap())
        .collect();
    let busybox_dir = fs.root_dir().create_dir("lua_tests")?;
    for app in apps_busybox {
        // load app data from host file system
        let mut host_file = File::open(format!("{}{}", busybox_path, app)).unwrap();
        let mut all_data: Vec<u8> = Vec::new();
        host_file.read_to_end(&mut all_data).unwrap();
        if app.eq("busybox") {
            // create a file in fat-fs
            let mut file = fs.root_dir().create_file(&app)?;
            // write data to fat-fs
            file.write_all(&all_data)?;
        } 
        // create a file in fat-fs
        let mut file = busybox_dir.create_file(&app)?;
        // write data to fat-fs
        file.write_all(&all_data)?;
    }

    // Write libc && libc tests
    let libc_path = "../testcases/libc/";
    let apps_libc: Vec<_> = read_dir(libc_path)
        .unwrap()
        .into_iter()
        .map(|dir_entry| dir_entry.unwrap().file_name().into_string().unwrap())
        .collect();
    let libc_dir = fs.root_dir().create_dir("libc")?;
    for app in apps_libc {
        // load app data from host file system
        if app.eq("etc") {
            let etc = fs.root_dir().create_dir(&app)?;
            let apps_etc: Vec<_> = read_dir(format!("{}{}", libc_path, app))
                .unwrap()
                .into_iter()
                .map(|dir_entry| dir_entry.unwrap().file_name().into_string().unwrap())
                .collect();
            for app_etc in apps_etc {
                let mut host_file =
                    File::open(format!("{}{}/{}", libc_path, app, app_etc)).unwrap();
                let mut all_data: Vec<u8> = Vec::new();
                host_file.read_to_end(&mut all_data).unwrap();
                // create a file in fat-fs
                let mut file = etc.create_file(&app_etc)?;
                // write data to fat-fs
                file.write_all(&all_data)?;
            }
        } else {
            let mut host_file = File::open(format!("{}{}", libc_path, app)).unwrap();
            let mut all_data: Vec<u8> = Vec::new();
            host_file.read_to_end(&mut all_data).unwrap();
            // create a file in fat-fs
            // let mut file = fs.root_dir().create_file(&app)?;
            let mut file = libc_dir.create_file(&app)?;
            // write data to fat-fs
            file.write_all(&all_data)?;
        }
    }

    // // Write libc tests
    // let libc_test_path = "../testcases/libc-test/";
    // let apps_libc_test: Vec<_> = read_dir(libc_test_path)
    //     .unwrap()
    //     .into_iter()
    //     .map(|dir_entry| dir_entry.unwrap().file_name().into_string().unwrap())
    //     .collect();
    // for app in apps_libc_test {
    //     // load app data from host file system
    //     let mut host_file = File::open(format!("{}{}", libc_test_path, app)).unwrap();
    //     let mut all_data: Vec<u8> = Vec::new();
    //     host_file.read_to_end(&mut all_data).unwrap();
    //     // create a file in fat-fs
    //     let mut file = fs.root_dir().create_file(&app)?;
    //     // write data to fat-fs
    //     file.write_all(&all_data)?;
    // }

    // // Write busybox
    // let busybox_path = "../testcases/busybox/busybox";
    // let mut host_file = File::open(busybox_path).unwrap();
    // let mut all_data: Vec<u8> = Vec::new();
    // host_file.read_to_end(&mut all_data).unwrap();
    // // create a file in fat-fs
    // let mut file = fs.root_dir().create_file("busybox")?;
    // // write data to fat-fs
    // file.write_all(&all_data)?;

    // // Write busybox_debug
    // let busybox_path = "../testcases/busybox/busybox_debug";
    // let mut host_file = File::open(busybox_path).unwrap();
    // let mut all_data: Vec<u8> = Vec::new();
    // host_file.read_to_end(&mut all_data).unwrap();
    // // create a file in fat-fs
    // let mut file = fs.root_dir().create_file("busybox_debug")?;
    // // write data to fat-fs
    // file.write_all(&all_data)?;

    // // Write libc
    // let libc_path = "../testcases/libc/";
    // let libc_apps: Vec<_> = read_dir(libc_path)
    //     .unwrap()
    //     .into_iter()
    //     .map(|dir_entry| {
    //         dir_entry.unwrap().file_name().into_string().unwrap()
    //     })
    //     // .filter(|name| *name != "mnt" && *name != "fs.img")
    //     .collect();
    // for app in libc_apps {
    //     // load app data from host file system
    //     let mut host_file = File::open(format!("{}{}", libc_path, app)).unwrap();
    //     let mut all_data: Vec<u8> = Vec::new();
    //     host_file.read_to_end(&mut all_data).unwrap();
    //     // create a file in fat-fs
    //     let mut file = fs.root_dir().create_file(&app)?;
    //     // write data to fat-fs
    //     file.write_all(&all_data)?;
    // }

    // let rust_apps: Vec<_> = read_dir(src_path)
    //     .unwrap()
    //     .into_iter()
    //     .map(|dir_entry| {
    //         let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
    //         name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
    //         name_with_ext
    //     })
    //     .collect();
    // let rust_apps: Vec<&str> = vec!["initproc", "shell", "time_share_test"];
    // for app in rust_apps {
    //     let mut host_file = File::open(format!("{}{}", target_path2, app)).unwrap();
    //     let mut all_data: Vec<u8> = Vec::new();
    //     host_file.read_to_end(&mut all_data).unwrap();
    //     // create a file in fat-fs
    //     let mut file = fs.root_dir().create_file(&app)?;

    //     // write data to fat-fs
    //     file.write_all(&all_data)?;
    // }

    println!("pack apps finished");

    Ok(())
}

fn main() -> io::Result<()> {
    let matches = App::new("Fat32FileSystem packer")
        .arg(
            Arg::with_name("fs_img")
                .short("f")
                .long("fs_img")
                .takes_value(true)
                .help("Fs img"),
        )
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .takes_value(true)
                .help("Executable source dir(with backslash)"),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .takes_value(true)
                .help("Executable target dir(with backslash)"),
        )
        .arg(
            Arg::with_name("target2")
                .short("e")
                .long("target2")
                .takes_value(true)
                .help("Executable target dir(with backslash)"),
        )
        .get_matches();
    let filename = matches.value_of("fs_img").unwrap().to_string();
    mkfs(filename.clone())?;
    pack_elfs(matches, filename.clone())?;
    Ok(())
}
