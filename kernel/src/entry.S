    .section .text.entry
    .globl _start
_start:
    # a0 = hart id
    # pc = 0x80200000

    # set sp(each hart has one kstack)
    slli t0, a0, 16  # t0 = hart_id << 16(4096 * 16)
    la sp, boot_stack_top
    sub sp, sp, t0  # sp = stack top - hart_id * stack_size

    # since the base addr is 0xffff_ffc0_8020_0000
    # we need to activate pagetable here in case of absolute addressing
    # satp: 8 << 60 | boot_pagetable
    la t0, boot_pagetable
    li t1, 8 << 60
    srli t0, t0, 12
    or t0, t0, t1
    csrw satp, t0
    sfence.vma

    # lui t0, %hi(rust_main)

    # call rust_main
    call fake_main

    .section .bss.stack

    .globl boot_stack_lower_bound
boot_stack_lower_bound:

    .space 4096 * 16 * 8  # 8 CPUS at most

    .globl boot_stack_top
boot_stack_top:

    .section .data
    .align 12
boot_pagetable:
    # we need 2 pte here
    # 0x0000_0000_8000_0000 -> 0x0000_0000_8000_0000
    # 0xffff_fc00_8000_0000 -> 0x0000_0000_8000_0000
    .quad 0
    .quad 0
    .quad (0x80000 << 10) | 0xcf # VRWXAD
    .zero 8 * 255
    .quad (0x80000 << 10) | 0xcf # VRWXAD
    .zero 8 * 253



    .section .text.trampoline
    .align 12
    .global sigreturn_trampoline
sigreturn_trampoline:
    li	a7,139
    ecall