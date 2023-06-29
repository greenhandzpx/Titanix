use log::{debug, info, warn};

use crate::{
    config::mm::PAGE_SIZE,
    mm::{
        memory_space::{
            page_fault_handler::{MmapPageFaultHandler, SBrkPageFaultHandler},
            vm_area::BackupFile,
            PageFaultHandler,
        },
        MapPermission, VirtAddr, VPNRange,
    },
    processor::current_process,
    stack_trace,
    syscall::{MmapFlags, MmapProt},
    utils::error::{SyscallErr, SyscallRet},
};

/// Note that we just ignore the `addr`
pub fn sys_mmap(
    addr: usize,
    length: usize,
    prot: i32,
    flags: i32,
    fd: usize,
    offset: usize,
) -> SyscallRet {
    stack_trace!();
    let prot = MmapProt::from_bits(prot as u32).ok_or(SyscallErr::EINVAL)?;
    let flags = MmapFlags::from_bits(flags as u32).ok_or(SyscallErr::EINVAL)?;
    let map_permission: MapPermission = prot.into();
    debug!(
        "[sys_mmap]: start...  addr {:#x}, len {}, fd {}, offset {}, flags {:?}, prot {:?}",
        addr, length, fd, offset, flags, prot
    );

    if flags.contains(MmapFlags::MAP_ANONYMOUS) {
        if offset != 0 {
            return Err(SyscallErr::EINVAL);
        }
        current_process().inner_handler(|proc| {
            let mut vma = {
                if flags.contains(MmapFlags::MAP_FIXED) {
                    proc.memory_space
                        .allocate_spec_area(length, map_permission, addr.into())?
                        .ok_or(SyscallErr::ENOMEM)?
                } else {
                    proc.memory_space
                        .allocate_area(length, map_permission)
                        .ok_or(SyscallErr::ENOMEM)?
                }
            };
            vma.mmap_flags = Some(flags);
            let handler = SBrkPageFaultHandler {};
            vma.handler = Some(handler.arc_clone());
            let start_va: VirtAddr = vma.start_vpn().into();
            let end_va: VirtAddr = vma.end_vpn().into();
            proc.memory_space.insert_area(vma);

            debug!("[sys_mmap]: finished, vma: {:#x}", start_va.0,);
            debug!(
                "handle anonymous mmap, vma {:#x}-{:#x}, prot {:?}, flags {:?}, map perm {:?}",
                start_va.0, end_va.0, prot, flags, map_permission
            );
            Ok(start_va.0 as isize)
        })
        // todo!("Handle anonymous mmap")
    } else {
        current_process().inner_handler(|proc| {
            let file = proc.fd_table.get(fd).ok_or(SyscallErr::EBADF)?;
            let mut vma = {
                if flags.contains(MmapFlags::MAP_FIXED) {
                    proc.memory_space
                        .allocate_spec_area(length, map_permission, addr.into())?
                        .ok_or(SyscallErr::ENOMEM)?
                } else {
                    proc.memory_space
                        .allocate_area(length, map_permission)
                        .ok_or(SyscallErr::ENOMEM)?
                }
            };
            vma.mmap_flags = Some(flags);
            let handler = MmapPageFaultHandler {};
            vma.handler = Some(handler.arc_clone());
            vma.backup_file = Some(BackupFile {
                offset,
                file: file.clone(),
            });
            let start_va: VirtAddr = vma.start_vpn().into();
            proc.memory_space.insert_area(vma);

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
    stack_trace!();
    if addr % PAGE_SIZE != 0 {
        return Err(SyscallErr::EINVAL);
    }
    let prot = MmapProt::from_bits(prot as u32).ok_or(SyscallErr::EINVAL)?;
    let map_permission: MapPermission = prot.into();
    debug!(
        "[sys_mprotect]: addr {:#x} len {:#x}, prot {:?}",
        addr, len, prot
    );
    current_process().inner_handler(|proc| {
        let vma = proc
            .memory_space
            .find_vm_area_mut_by_vpn(VirtAddr::from(addr).floor())
            .ok_or(SyscallErr::EINVAL)?;
        vma.map_perm = map_permission;
        Ok(())
    })?;
    Ok(0)
}

pub fn sys_brk(addr: usize) -> SyscallRet {
    stack_trace!();
    debug!("handle sys brk");
    if addr == 0 {
        debug!("[sys_brk]: addr: 0");
        return Ok(current_process()
            .inner_handler(|proc| proc.memory_space.heap_range.unwrap().end().0)
            as isize);
    }

    current_process().inner_handler(|proc| {
        let heap_start: VirtAddr = proc.memory_space.heap_range.unwrap().start();
        let current_heap_end: VirtAddr = proc.memory_space.heap_range.unwrap().end();
        let new_heap_end: VirtAddr = addr.into();
        debug!(
            "[sys_brk]: old heap end: {:#x}, new heap end: {:#x}",
            current_heap_end.0, new_heap_end.0
        );
        if addr > current_heap_end.0 {
            // allocate memory lazily
            if proc
                .memory_space
                .check_vpn_range_conflict(heap_start.floor(), new_heap_end.ceil())
            {
                warn!("[sys_brk]: new addr invalid");
                Err(SyscallErr::ENOMEM)
            } else {
                let heap_vma = proc
                    .memory_space
                    .find_vm_area_mut_by_vpn_included(heap_start.floor())
                    .unwrap();
                // modify vma
                heap_vma.vpn_range.modify_right_bound(new_heap_end.ceil());
                // modify process info(lazy allocation)
                proc.memory_space
                    .heap_range
                    .as_mut()
                    .unwrap()
                    .modify_right_bound(new_heap_end);
                debug!(
                    "new heap end {:#x}",
                    proc.memory_space.heap_range.unwrap().end().0
                );
                Ok(proc.memory_space.heap_range.unwrap().end().0 as isize)
            }
        } else {
            // deallocate memory
            if addr < heap_start.0 {
                Err(SyscallErr::ENOMEM)
            } else {
                let heap_vma = proc
                    .memory_space
                    .find_vm_area_mut_by_vpn(heap_start.floor())
                    .unwrap();
                heap_vma.vpn_range.modify_right_bound(new_heap_end.ceil());
                let data_frames = unsafe { &mut (*heap_vma.data_frames.get()) };
                // modify vma
                heap_vma.vpn_range.modify_right_bound(new_heap_end.ceil());
                let page_table = unsafe { &mut (*proc.memory_space.page_table.get()) };
                let removed_vpns = VPNRange::new(new_heap_end.ceil(), current_heap_end.ceil());
                for vpn in removed_vpns {
                    if data_frames.0.contains_key(&vpn) {
                        data_frames.0.remove(&vpn);
                        page_table.unmap(vpn);
                    }
                }
                page_table.activate();
                // modify process info
                proc.memory_space
                    .heap_range
                    .unwrap()
                    .modify_right_bound(new_heap_end);
                // Ok(0)
                Ok(proc.memory_space.heap_range.unwrap().end().0 as isize)
            }
        }
    })
}