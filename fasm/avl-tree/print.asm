macro print {
    mov rax, 1
    mov rdi, 1
    syscall
}

macro log_print msg, len, lv {
    mov rbx, msg
    mov rcx, len
    mov r8, lv
    call log
}

segment readable executable

; log(inp_msg: rbx, inp_len: rcx, level: r8) -> ()
log:
    push rcx
    push rbx
    push r8

    cmp r8, 1
    je .debug

    cmp r8, 2
    je .info

    cmp r8, 3
    je .warn

    cmp r8, 4
    je .err

    jmp .continue

.debug:
    mov rsi, DEBUG_PREFIX
    mov rdx, DEBUG_PREFIX_LEN
    print
    jmp .continue

.info:
    mov rsi, INFO_PREFIX
    mov rdx, INFO_PREFIX_LEN
    print
    jmp .continue

.warn:
    mov rsi, WARN_PREFIX
    mov rdx, WARN_PREFIX_LEN
    print
    jmp .continue

.err:
    mov rsi, ERROR_PREFIX
    mov rdx, ERROR_PREFIX_LEN
    print
    jmp .continue

.continue:
    pop r8
    pop rbx
    pop rcx

    mov rsi, rbx
    mov rdx, rcx
    print
    ret

segment readable
DEBUG_PREFIX db 033o, "[36m[DEBUG] ", 27, "[0m"
DEBUG_PREFIX_LEN = $-DEBUG_PREFIX

INFO_PREFIX db 033o, "[32m[INFO] ", 27, "[0m"
INFO_PREFIX_LEN = $-INFO_PREFIX

WARN_PREFIX db 033o, "[33m[WARN] ", 27, "[0m"
WARN_PREFIX_LEN = $-WARN_PREFIX

ERROR_PREFIX db 033o, "[31m[ERROR] ", 27, "[0m"
ERROR_PREFIX_LEN = $-ERROR_PREFIX
