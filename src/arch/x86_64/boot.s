    .set PIC1_DATA_PORT, 0x21
    .set PIC2_DATA_PORT, 0xA1

    .set CR0_PG, 1 << 31

    .set CR4_PAE, 1 << 5
    .set CR4_OSFXSR, 1 << 9

    .set PTE_PRESENT, 1 << 0
    .set PTE_WRITE, 1 << 1
    .set PTE_PS, 1 << 7

    .set MSR_EFER, 0xC0000080
    .set MSR_EFER_LME, 1 << 8

    .set GDT_TYPE_DATA, 0x2 << 40
    .set GDT_TYPE_CODE, 0xA << 40
    .set GDT_NONSYS, 1 << 44
    .set GDT_PRESENT, 1 << 47
    .set GDT_BITS64, 1 << 53
    .set GDT_BITS32, 1 << 54

    .set SEGMENT_CODE, 0x8
    .set SEGMENT_DATA, 0x10

    .set STACK_SIZE, 8 * 1024

    .bss
    .align 4096
pml4:
    .fill 512, 8
pdp0:
    .fill 512, 8
pd0:
    .fill 512, 8

    .align 16
stack:
    .fill STACK_SIZE

    .data
    .align 4
gdt:
    .quad 0
    .quad GDT_TYPE_CODE | GDT_NONSYS | GDT_PRESENT | GDT_BITS64
    .quad GDT_TYPE_DATA | GDT_NONSYS | GDT_PRESENT | GDT_BITS32
end_of_gdt:

gdti:
    .word end_of_gdt - gdt - 1
    .quad gdt

    .text
    .code32
    .global boot_mb
boot_mb:
    /* Preserve magic and multiboot_info. */
    movl %eax, %edi
    movl %ebx, %esi

    /* Disable IRQs. */
    movb $0xFF, %al
    outb %al, $PIC1_DATA_PORT
    outb %al, $PIC2_DATA_PORT

    /* Enable PAE and SSE. */
    movl %cr4, %edx
    orl $(CR4_PAE | CR4_OSFXSR), %edx
    movl %edx, %cr4

    /* Link page table entries and set page map. */
    orl $(PTE_PRESENT | PTE_WRITE), pml4
    orl $pdp0, pml4
    orl $(PTE_PRESENT | PTE_WRITE), pdp0
    orl $pd0, pdp0
    orl $(PTE_PRESENT | PTE_WRITE | PTE_PS), pd0
    movl $pml4, %eax
    movl %eax, %cr3

    /* Enable long mode. */
    movl $MSR_EFER, %ecx
    rdmsr
    orl $MSR_EFER_LME, %eax
    wrmsr

    /* Enable paging. */
    movl %cr0, %eax
    orl $CR0_PG, %eax
    movl %eax, %cr0

    /* Load GDT. */
    lgdt gdti

    /* Long jump to 64-bit code. */
    ljmp $SEGMENT_CODE, $start64

    .code64
start64:
    /* Set segments and stack. */
    movw $SEGMENT_DATA, %ax
    movw %ax, %ds
    movw %ax, %ss
    movq $(stack + STACK_SIZE), %rsp

    /* Extend preserved magic and multiboot_info. */
    shlq $32, %rdi
    shrq $32, %rdi
    shlq $32, %rsi
    shrq $32, %rsi

    /* Call the Rust entry point. */
    call start_mb
