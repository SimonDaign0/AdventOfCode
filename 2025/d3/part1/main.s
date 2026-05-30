.intel_syntax noprefix
.section .data
    filename: .asciz "input.txt"
    ln: .asciz "\n"
    buf_size = 22000

.section .bss
    buf: .skip buf_size
    itoa_buf: .skip 32

.section .text
    .global _start

_exit:
    mov rax, 60
    xor rdi, rdi
    syscall

_openfile:
    mov rax, 2
    lea rdi, filename
    xor rsi, rsi
    syscall
    ret

_closefile:
    mov rax, 3
    syscall
    ret

_readfile:
    xor rax, rax
    lea rsi, buf
    mov rdx, buf_size
    syscall
    ret

_itoa:
    lea r8, [itoa_buf + 31]
    mov byte ptr [r8], 0

    mov rax, rdi
    mov rbx, 10

    cmp rax, 0
    jne itoa_main

    dec r8
    mov byte ptr [r8], '0'
    mov rax, r8
    ret

itoa_main:
itoa_loop:
    xor rdx, rdx
    div rbx
    add dl, '0'
    dec r8
    mov byte ptr [r8], dl
    test rax, rax
    jne itoa_loop

    mov rax, r8
    ret

_strlen:
    xor rcx, rcx
strlen_loop:
    cmp byte ptr [rdi + rcx], 0
    je strlen_end
    inc rcx
    jmp strlen_loop
strlen_end:
    mov rax, rcx
    ret

_linelen:
    xor rcx, rcx
linelen_loop:
    cmp byte ptr [rdi + rcx], 0
    je linelen_end
    cmp byte ptr [rdi + rcx], '\n'
    je linelen_end
    inc rcx
    jmp linelen_loop
linelen_end:
    mov rax, rcx
    ret


_start:
    call _openfile
    mov r15, rax

    mov rdi, rax
    call _readfile
    mov r8, rax              # file size

    lea r9, buf              # line pointer
    lea rbx, [buf + r8]      # end pointer

    xor r10, r10             # total sum

next_line_start:
    cmp r9, rbx
    jae program_end

    mov rdi, r9
    call _linelen
    mov r11, rax             # line length

    xor r12, r12             # best per line
    xor rcx, rcx             # i = 0

i_loop_start:
    cmp rcx, r11
    jge line_done

    movzx rdx, byte ptr [r9 + rcx]

    cmp rdx, '0'
    jb i_next
    cmp rdx, '9'
    ja i_next
    sub rdx, '0'

    mov r13, rcx
    inc r13                  # j = i + 1

j_loop_start:
    cmp r13, r11
    jge i_next

    movzx rax, byte ptr [r9 + r13]

    cmp rax, '0'
    jb j_next
    cmp rax, '9'
    ja j_next
    sub rax, '0'

    mov r14, rdx
    imul r14, r14, 10
    add r14, rax

    cmp r14, r12
    jle j_next
    mov r12, r14

j_next:
    inc r13
    jmp j_loop_start

i_next:
    inc rcx
    jmp i_loop_start

line_done:
    add r10, r12

    add r9, r11
    inc r9
    jmp next_line_start


program_end:
mov rdi, r10
call _itoa              # rax = string pointer

mov rbx, rax            # SAVE pointer

mov rdi, rax
call _strlen            # rax = length

mov rdx, rax            # length
mov rsi, rbx            # pointer restored
mov rax, 1
mov rdi, 1
syscall

    mov rdi, r15
    call _closefile

    call _exit
