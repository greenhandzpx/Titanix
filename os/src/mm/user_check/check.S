    .section .text
    .attribute arch, "rv64imac"
    .global __try_read_user_u8
    .global __try_write_user_u8
    .global __try_access_user_error_trap

// Note that we should set stvec in advance before the following check
// arg: a0: input_addr
// return: a0 = 0 if no exception
__try_read_user_u8:
    mv a1, a0
    mv a0, zero
    # lb a1, 0(a1)
    lb a1, 0(a1)
    ret
    .align 2
__try_write_user_u8:
    mv a2, a0
    mv a0, zero
    lb a3, 0(a2)
    sb a1, 0(a2)
    sb a3, 0(a2)
    ret
    .align 2

// if pagefault occurs, return: (a0, a1) <- (1, scause).
    .align 6
__try_access_user_error_trap:
    csrw sepc, ra   # ra -> __try_x_user_u8's return addr
    li a0, 1
    csrr a1, scause
    sret
    # addi sp, sp, -16
    # sd   ra, 8(sp)
    # csrr ra, scause
    # bltz ra, access_interrupt
    # csrw sepc, ra
    # li   a0, 1
    # csrr a1, scause
    # ld   ra, 8(sp)
    # addi sp, sp, 16
    # sret
# access_interrupt:
#     ld   ra, 8(sp)
#     addi sp, sp, 16
#     j __kernel_default_interrupt_entry