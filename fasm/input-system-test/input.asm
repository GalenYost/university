macro prompt_input {
    log_print OPTIONS_PROMPT, OPTIONS_PROMPT_LEN, 0
    log_print INPUT_PROMPT, INPUT_PROMPT_LEN, 0

    mov rax, 0
    mov rdi, 0
    mov rsi, MENU_BUF
    mov rdx, MENU_BUF_SIZE
    syscall

    mov al, [MENU_BUF]
}

segment readable
OPTIONS_PROMPT db "0 - exit", 10
               db "1 - help", 10
OPTIONS_PROMPT_LEN = $-OPTIONS_PROMPT

INPUT_PROMPT db "Input: "
INPUT_PROMPT_LEN = $-INPUT_PROMPT

segment writable
MENU_BUF_SIZE = 2
MENU_BUF rb MENU_BUF_SIZE
