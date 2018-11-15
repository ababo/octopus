OUTPUT_FORMAT(elf32-i386)

ENTRY(_start)

SECTIONS {
    .header : {
        *(.header);
    }

    . = 0x500;

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

    .rela.plt : {
        *(.rela.plt);
    }

    .shstrtab : {
        *(.shstrtab);
    }

    /DISCARD/ : {
        *(*);
    }
}
