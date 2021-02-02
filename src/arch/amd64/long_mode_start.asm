global long_mode_start
extern rust_main

section .text
bits 64
long_mode_start:
    ; load 0 into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ;; Set up a NULL rbp for stack tracing
    xor   rbp, rbp       ; Set ebp to NULL
    push  rbp            ; Push a NULL return address to the stack
    call rust_main