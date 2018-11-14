    .text
    .code32
    .global _start
_start:
    movw 0x400, %dx
    movb $'H', %al
    outb %al, %dx

h:  hlt
    jmp h
