    .text
    .code32

    .global _start
_start:
    /* Preserve magic and multiboot_info. */
    movl %eax, %edi
    movl %ebx, %esi

    /* Disable PIC1 and PIC2 IRQs. */
    movb $0xFF, %al
    outb %al, $0x21
    outb %al, $0xA1

    /* Enable PAE and SSE. */
    movl %cr4, %edx
    orl $0x220, %edx
    movl %edx, %cr4

    /* Map first 2 MiB of memory 1:1 with a one page . */
    orl $0x3, _pml4
    orl $_pdp, _pml4
    orl $0x3, _pdp
    orl $_pd, _pdp
    orl $0x83, _pd
    movl $_pml4, %eax
    movl %eax, %cr3

    /* Enable long mode. */
    movl $0xC0000080, %ecx
    rdmsr
    orl $0x100, %eax
    wrmsr

    /* Enable paging. */
    movl %cr0, %eax
    orl $0x80000000, %eax
    movl %eax, %cr0

    /* Load GDT */
    lgdt _gdt_desc

    /* long jump to 64-bit code */
    ljmp $0x8, $start64


    .code64

start64:
    /* Set the data and stack segments. */
    movw $0x10, %ax
    movw %ax, %ds
    movw %ax, %ss

    /* Set the stack pointer. */
    mov $_bsp_stack, %rax
    addl _bsp_stack_size, %eax
    movq %rax, %rsp

    /* Extend preserved magic and multiboot_info. */
    shlq $32, %rdi
    shrq $32, %rdi
    shlq $32, %rsi
    shrq $32, %rsi

    /* Call the Rust entry point. */
    call _init

movw 0x400, %dx
movb $'H', %al
outb %al, %dx
h:
hlt
jmp h

