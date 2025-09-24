.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text
    .globl __trap_from_user
    .globl __return_to_user
    .globl __trap_from_kernel
    .align 2


# user -> kernel
__trap_from_user:
    csrrw sp, sscratch, sp
    # now sp->*TrapContext in kernel space, sscratch->user stack
    # save other general purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    # save x3~x31 (x4 is tp)
    .set n, 3
    .rept 29
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they have been saved in TrapContext
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it in TrapContext
    csrr t2, sscratch
    sd t2, 2*8(sp)

    # # move to kernel_sp
    # load kernel ra
    ld ra, 35*8(sp)
    # load callee-saved regs
    ld s0, 36*8(sp)
    ld s1, 37*8(sp)
    ld s2, 38*8(sp)
    ld s3, 39*8(sp)
    ld s4, 40*8(sp)
    ld s5, 41*8(sp)
    ld s6, 42*8(sp)
    ld s7, 43*8(sp)
    ld s8, 44*8(sp)
    ld s9, 45*8(sp)
    ld s10, 46*8(sp)
    ld s11, 47*8(sp)
    # load kernel fp
    ld fp, 48*8(sp)
    ld tp, 49*8(sp)
    ld sp, 34*8(sp)
    # return to kernel ra
    ret  

# kernel -> user
__return_to_user:
    # a0: *TrapContext in user space(Constant); a1: user space token
    # switch to user space

    # let sscratch store the trap context's address
    csrw sscratch, a0
    # save kernel callee-saved regs
    sd sp, 34*8(a0)
    sd ra, 35*8(a0)
    sd s0, 36*8(a0)
    sd s1, 37*8(a0)
    sd s2, 38*8(a0)
    sd s3, 39*8(a0)
    sd s4, 40*8(a0)
    sd s5, 41*8(a0)
    sd s6, 42*8(a0)
    sd s7, 43*8(a0)
    sd s8, 44*8(a0)
    sd s9, 45*8(a0)
    sd s10, 46*8(a0)
    sd s11, 47*8(a0)
    sd fp, 48*8(a0)
    sd tp, 49*8(a0)

    mv sp, a0
    # now sp points to TrapContext in kernel space, start restoring based on it
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    # restore general purpose registers except x0/sp/
    ld x1, 1*8(sp)
    .set n, 3
    .rept 29
        LOAD_GP %n
        .set n, n+1
    .endr
    # back to user stack
    ld sp, 2*8(sp)
    sret

# kernel -> kernel
__trap_from_kernel:
    # only need to save caller-saved regs
    # note that we don't save sepc & stvec here
    addi sp, sp, -17*8
    sd  ra,  1*8(sp)
    sd  t0,  2*8(sp)
    sd  t1,  3*8(sp)
    sd  t2,  4*8(sp)
    sd  t3,  5*8(sp)
    sd  t4,  6*8(sp)
    sd  t5,  7*8(sp)
    sd  t6,  8*8(sp)
    sd  a0,  9*8(sp)
    sd  a1, 10*8(sp)
    sd  a2, 11*8(sp)
    sd  a3, 12*8(sp)
    sd  a4, 13*8(sp)
    sd  a5, 14*8(sp)
    sd  a6, 15*8(sp)
    sd  a7, 16*8(sp)
    call kernel_trap_handler
    ld  ra,  1*8(sp)
    ld  t0,  2*8(sp)
    ld  t1,  3*8(sp)
    ld  t2,  4*8(sp)
    ld  t3,  5*8(sp)
    ld  t4,  6*8(sp)
    ld  t5,  7*8(sp)
    ld  t6,  8*8(sp)
    ld  a0,  9*8(sp)
    ld  a1, 10*8(sp)
    ld  a2, 11*8(sp)
    ld  a3, 12*8(sp)
    ld  a4, 13*8(sp)
    ld  a5, 14*8(sp)
    ld  a6, 15*8(sp)
    ld  a7, 16*8(sp)
    addi sp, sp, 17*8
    sret

