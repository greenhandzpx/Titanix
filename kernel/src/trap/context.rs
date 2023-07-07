/*
 * @Author: greenhandzpx 893522573@qq.com
 * @Date: 2023-01-28 12:24:08
 * @LastEditors: greenhandzpx 893522573@qq.com
 * @LastEditTime: 2023-02-23 15:45:28
 * @FilePath: /oscomp-kernel/os/src/trap/context.rs
 * @Description:
 *
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 */
//! Implementation of [`TrapContext`]

use core::arch::asm;

use riscv::register::sstatus::{self, Sstatus, SPP};

/// Trap context structure containing sstatus, sepc and registers
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct TrapContext {
    /// user-to-kernel should save:
    /// general regs[0..31]
    pub user_x: [usize; 32],
    /// CSR sstatus      
    // pub sstatus: Sstatus, // 32
    pub sstatus: usize, // 32
    /// CSR sepc
    pub sepc: usize, // 33

    /// Unlike rCore-tutorial, we don't need to save
    /// trap_handler here, since we will trap back to kernel
    /// and go to trap handler by reloading kernel's ra(through __trap_from_user).
    // pub trap_handler: usize,

    /// kernel-to-user should save:
    ///
    pub kernel_sp: usize, // 34
    ///
    pub kernel_ra: usize, // 35
    ///
    pub kernel_s: [usize; 12], // 36 - 47
    ///
    pub kernel_fp: usize, // 48
    /// kernel hart address
    pub kernel_tp: usize, // 49
}

/// User context that used for signal handling
pub struct UserContext {
    /// general regs[0..31]
    pub user_x: [usize; 32],
    /// CSR sstatus      
    // pub sstatus: Sstatus, // 32
    pub sstatus: usize, // 32
    /// CSR sepc
    pub sepc: usize, // 33
}

impl UserContext {
    /// Construct a new user context from trap context
    pub fn from_trap_context(trap_context: &TrapContext) -> Self {
        Self {
            user_x: trap_context.user_x,
            sstatus: trap_context.sstatus,
            sepc: trap_context.sepc,
        }
    }
}

impl TrapContext {
    /// Set stack pointer to x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.user_x[2] = sp;
    }
    /// Init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        // set CPU privilege to User after trapping back
        sstatus.set_spp(SPP::User);
        sstatus.set_sie(false);
        sstatus.set_spie(false);
        // let tp: usize;
        // unsafe {
        //     asm!("mv {}, tp", out(reg) tp);
        // }
        let mut cx = Self {
            user_x: [0; 32],
            sstatus: sstatus.bits(),
            sepc: entry,
            // The following regs will be stored in asm funciton __restore
            // So we don't need to save them here
            kernel_sp: 0,
            kernel_ra: 0,
            kernel_s: [0; 12],
            kernel_fp: 0,
            // We will give the right kernel tp in `__return_to_user`
            kernel_tp: 0,
        };
        cx.set_sp(sp);
        cx
    }
}
