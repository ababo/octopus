OUTPUT_FORMAT(elf32-i386)

ENTRY(_start)

SECTIONS {
    . = 0x500;

    .mb_header : {
        *(.mb_header);
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
        *(.bss .bss.*);
    }

    /DISCARD/ : {
        *(*);
    }
}
