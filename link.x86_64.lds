OUTPUT_FORMAT(elf32-i386)

ENTRY(_start)

SECTIONS {
    . = 1M;

    .header : {
        *(.header);
    }

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
        *(.bss);
    }

    /DISCARD/ : {
        *(*);
    }
}
