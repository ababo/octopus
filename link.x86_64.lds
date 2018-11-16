OUTPUT_FORMAT(elf32-i386)

ENTRY(_start)

SECTIONS {
    . = 0x500;

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
