OUTPUT_FORMAT(elf64-littleaarch64)

ENTRY(_start)

SECTIONS {
    . = 1024M + 64K /* DTB */;

    .text : {
        *(.text .text.*);
    }

    .rodata : {
        *(.rodata .rodata.*);
    }

    .data : {
        *(.data .data.*);
    }

    .bss : {
        *(.bss .bss.*);
    }

    /DISCARD/ : {
        *(*);
    }
}
