    .section .text.entry
    .globl _start
_start:
    # a0 = hart id
    # pc = 0x80200000

    # set sp(each hart has one kstack)
    # TODO: now only support one hart
    slli t0, a0, 16  # t0 = hart_id << 16(4096 * 16)
    la sp, boot_stack_top
    sub sp, sp, t0  # sp = stack top - cpu_id * stack_size
    call rust_main

    .section .bss.stack

    .globl boot_stack_lower_bound
boot_stack_lower_bound:

    .space 4096 * 16 * 8  # 8 CPUS at most

    .globl boot_stack_top
boot_stack_top: