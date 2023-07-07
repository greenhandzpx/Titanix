OUTPUT_ARCH(riscv)
ENTRY(_start)
/* BASE_ADDRESS = 0x80200000; */
BASE_ADDRESS = 0xffffffc080200000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        . = ALIGN(4K);
        strampoline = .;
        *(.text.trampoline)
        . = ALIGN(4K);
        etrampoline = .;
        *(.text .text.*)
        /* . = ALIGN(4K); */
        /* *(.text.trampoline) */
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    sstack = .;
    /* boot stack(in entry.S) */
    .stack : {
        *(.bss.stack)
    }

    . = ALIGN(4K);
    estack = .;
    /* sbss_with_stack = .; */
    sbss = .;
    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    /* strampoline = .;
    .trampoline : {
        *(.trampoline)
    }
    . = ALIGN(4K);
    etrampoline = .; */
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}