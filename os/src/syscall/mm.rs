use log::debug;

use crate::{
    config::{fs::FILE_PAGE_SIZE, mm::PAGE_SIZE},
    mm::{
        memory_set::{
            page_fault_handler::{MmapPageFaultHandler, SBrkPageFaultHandler},
            vm_area::BackupFile,
            PageFaultHandler,
        },
        MapPermission, VirtAddr,
    },
    processor::current_process,
    stack_trace,
    syscall::{MmapFlags, MmapProt},
    utils::error::{SyscallErr, SyscallRet},
};

/// Note that we just ignore the `addr`
pub fn sys_mmap(
    _addr: *const u8,
    length: usize,
    prot: i32,
    flags: i32,
    fd: usize,
    offset: usize,
) -> SyscallRet {
    stack_trace!();
    debug!("[sys_mmap]: start... len {}, fd {}", length, fd);
    let prot = MmapProt::from_bits(prot as u32).ok_or(SyscallErr::EINVAL)?;
    let flags = MmapFlags::from_bits(flags as u32).ok_or(SyscallErr::EINVAL)?;
    let mut map_permission = MapPermission::from_bits(0).unwrap();
    if prot.contains(MmapProt::PROT_READ) {
        map_permission |= MapPermission::R;
    }
    if prot.contains(MmapProt::PROT_WRITE) {
        map_permission |= MapPermission::W;
    }
    if prot.contains(MmapProt::PROT_EXEC) {
        map_permission |= MapPermission::X;
    }

    if flags.contains(MmapFlags::MAP_ANONYMOUS) {
        debug!("handle anonymous mmap, prot {:?}, flags {:?}", prot, flags);
        if offset != 0 {
            return Err(SyscallErr::EINVAL);
        }
        current_process().inner_handler(|proc| {
            let mut vma = proc
                .memory_set
                .find_unused_area(length, map_permission)
                .ok_or(SyscallErr::ENOMEM)?;
            vma.mmap_flags = Some(flags);
            let handler = SBrkPageFaultHandler {};
            vma.handler = Some(handler.box_clone());
            let start_va: VirtAddr = vma.start_vpn().into();
            proc.memory_set.insert_area(vma);

            debug!("[sys_mmap]: finished, vma: {:#x}", start_va.0,);
            Ok(start_va.0 as isize)
        })
        // todo!("Handle anonymous mmap")
    } else {
        current_process().inner_handler(|proc| {
            let file = proc.fd_table.get(fd).ok_or(SyscallErr::EBADF)?;
            // let mut buf: [u8; 36] = [0; 36];
            // file.seek(0)?;
            // file.sync_read(&mut buf)?;
            let mut vma = proc
                .memory_set
                .find_unused_area(length, map_permission)
                .ok_or(SyscallErr::ENOMEM)?;
            vma.mmap_flags = Some(flags);
            let handler = MmapPageFaultHandler {};
            vma.handler = Some(handler.box_clone());
            vma.backup_file = Some(BackupFile {
                offset,
                file: file.clone(), // .metadata()
                                    // .inner
                                    // .lock()
                                    // .inode
                                    // .as_ref()
                                    // .cloned()
                                    // .unwrap(),
            });
            let start_va: VirtAddr = vma.start_vpn().into();
            proc.memory_set.insert_area(vma);

            debug!("[sys_mmap]: finished, vma: {:#x}", start_va.0,);
            Ok(start_va.0 as isize)
        })
        // let vm_area = VmArea::new()
    }
}

pub fn sys_munmap(addr: usize, length: usize) -> SyscallRet {
    // TODO
    Ok(0)
    // todo!()
}

pub fn sys_mprotect(addr: usize, len: usize, prot: i32) -> SyscallRet {
    if addr % PAGE_SIZE != 0 {
        return Err(SyscallErr::EINVAL);
    }
    let prot = MmapProt::from_bits(prot as u32).ok_or(SyscallErr::EINVAL)?;
    // current_process().inner_handler(| proc | {
    //     let vma = proc.memory_set.find_vm_area_mut_by_vpn(VirtAddr::from(addr).floor()).ok_or(Err(SyscallErr::))
    // })
    // TODO
    Ok(0)
}
