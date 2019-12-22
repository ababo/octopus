    .set CPACR_EL1_FPEN, 0b11 << 20

    .set STACK_SIZE, 32 * 1024

    .bss
    .balign 16
stack:
    .fill STACK_SIZE

    .text
    .global _start
_start:
    /* Disable FP and SIMD traps. */
    mov x0, #CPACR_EL1_FPEN
    msr cpacr_el1, x0

    /* Set stack. */
    adr x0, stack
    add sp, x0, #STACK_SIZE

    /* Call the Rust entry point */
    bl main
