use log::{debug, info, warn};

use crate::{
    config::mm::PAGE_SIZE,
    mm::{
        memory_space::{
            page_fault_handler::{MmapPageFaultHandler, SBrkPageFaultHandler},
            vm_area::{BackupFile, VmAreaType},
            PageFaultHandler,
        },
        MapPermission, VPNRange, VirtAddr, SHARED_MEMORY_MANAGER,
    },
    processor::current_process,
    stack_trace,
    syscall::{MmapFlags, MmapProt},
    utils::error::{SyscallErr, SyscallRet},
};

/// Note that we just ignore the `addr` when the `MAP_FIXED` isn't specified.
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
    info!(
        "[sys_mmap]: start...  addr {:#x}, len {:#x}, fd {}, offset {:#x}, flags {:?}, prot {:?}",
        addr, length, fd, offset, flags, prot
    );

    if flags.contains(MmapFlags::MAP_ANONYMOUS) {
        if offset != 0 {
            return Err(SyscallErr::EINVAL);
        }
        // TODO: support shared memory(i.e. MAP_ANONYMOUS | MAP_SHARED)
        current_process().inner_handler(|proc| {
            let mut vma = {
                if flags.contains(MmapFlags::MAP_FIXED) {
                    proc.memory_space
                        .allocate_spec_area(length, map_permission, addr.into(), VmAreaType::Mmap)?
                        .ok_or(SyscallErr::ENOMEM)?
                } else {
                    proc.memory_space
                        .allocate_area(length, map_permission, VmAreaType::Mmap)
                        .ok_or(SyscallErr::ENOMEM)?
                }
            };
            vma.map_perm = map_permission | MapPermission::U;
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
            Ok(start_va.0)
        })
    } else {
        if offset % PAGE_SIZE != 0 {
            warn!(
                "[sys_mmap] the offset {:#x} isn't a multiple of the page size",
                offset
            );
            return Err(SyscallErr::EINVAL);
        }
        // if flags.contains(MmapFlags::MAP_FIXED) {
        //     exit_and_terminate_all_threads(0);
        // }
        current_process().inner_handler(|proc| {
            let file = proc.fd_table.get(fd).ok_or(SyscallErr::EBADF)?;
            let mut vma = {
                if flags.contains(MmapFlags::MAP_FIXED) {
                    proc.memory_space
                        .allocate_spec_area(length, map_permission, addr.into(), VmAreaType::Mmap)?
                        .ok_or(SyscallErr::ENOMEM)?
                } else {
                    proc.memory_space
                        .allocate_area(length, map_permission, VmAreaType::Mmap)
                        .ok_or(SyscallErr::ENOMEM)?
                }
            };
            vma.map_perm = map_permission | MapPermission::U;
            vma.mmap_flags = Some(flags);
            let handler = MmapPageFaultHandler {};
            vma.handler = Some(handler.arc_clone());

            // if vma.backup_file.is_none() {
            //     vma.backup_file = Some(BackupFile { offset, file });
            // }
            // if flags.contains(MmapFlags::MAP_FIXED) {
            //     vma.backup_file = Some(BackupFile { offset: 0, file });
            // }
            vma.backup_file = Some(BackupFile {
                offset,
                file: file.file,
            });

            let start_va: VirtAddr = vma.start_vpn().into();
            debug!(
                "[sys_mmap]: finished, vma: {:#x}, map perm {:?}",
                start_va.0, vma.map_perm
            );
            proc.memory_space.insert_area(vma);
            Ok(start_va.0)
        })
    }
}

pub fn sys_munmap(addr: usize, length: usize) -> SyscallRet {
    // TODO
    stack_trace!();
    info!("[sys_munmap] addr {:#x}, len {:#x}...", addr, length);
    if addr % PAGE_SIZE != 0 {
        return Err(SyscallErr::EINVAL);
    }
    current_process().inner_handler(|proc| {
        let start_vpn = VirtAddr::from(addr).floor();
        let end_vpn = VirtAddr::from(addr + length).ceil();
        let vma = proc
            .memory_space
            .find_vm_area_mut_by_vpn(start_vpn)
            .ok_or(SyscallErr::EINVAL)?;
        // TODO: maybe we should check wether the user owns the permission to unmap the vma?
        let old_start_vpn = vma.start_vpn();
        let splited_vma = vma.unmap_area(VPNRange::new(start_vpn, end_vpn))?;
        if vma.start_vpn() != old_start_vpn {
            let vma = proc.memory_space.remove_vm_area(old_start_vpn).unwrap();
            if vma.start_vpn() < vma.end_vpn() {
                proc.memory_space.insert_area(vma);
            }
        }
        if let Some(splited_vma) = splited_vma {
            proc.memory_space.insert_area(splited_vma);
        }
        info!("[sys_munmap] addr {:#x}, len {:#x} finished", addr, length);
        Ok(0)
    })
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
    if prot == MmapProt::PROT_NONE {
        log::warn!("[sys_mprotect] PROT_NONE, ignore");
        return Ok(0);
    }
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

pub fn sys_msync(addr: usize, len: usize, flags: i32) -> SyscallRet {
    stack_trace!();
    log::warn!(
        "[sys_msync] not yet implemented. addr {:#x}, len {:#x}, flags {:#x}",
        addr,
        len,
        flags
    );
    Ok(0)
}

pub fn sys_brk(addr: usize) -> SyscallRet {
    stack_trace!();
    debug!("handle sys brk");
    if addr == 0 {
        debug!("[sys_brk]: addr: 0");
        return Ok(
            current_process().inner_handler(|proc| proc.memory_space.heap_range.unwrap().end().0)
        );
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
                Ok(proc.memory_space.heap_range.unwrap().end().0)
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
                Ok(proc.memory_space.heap_range.unwrap().end().0)
            }
        }
    })
}

pub fn sys_madvise() -> SyscallRet {
    stack_trace!();
    Ok(0)
}

const IPC_PRIVATE: usize = 0;

pub fn sys_shmget(key: usize, len: usize, _shmflag: u32) -> SyscallRet {
    stack_trace!();
    if key != IPC_PRIVATE {
        panic!("[sys_shmget] unsupported operation, key {:#X}", key);
    }
    Ok(SHARED_MEMORY_MANAGER.lock().alloc(key, len))
}

pub fn sys_shmat(shmid: usize, shmaddr: usize, _shmflag: u32) -> SyscallRet {
    stack_trace!();
    let addr = match shmaddr {
        0 => None,
        addr => Some(VirtAddr::from(addr)),
    };

    SHARED_MEMORY_MANAGER.lock().attach(shmid, addr)
}

pub fn sys_shmctl() -> SyscallRet {
    stack_trace!();
    Ok(0)
}
