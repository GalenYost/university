format ELF64 executable 3
entry main

include "print.asm"
include "input.asm"

segment readable

; prompts
MENU_PROMPT db "Options:", 10
MENU_PROMPT_LEN = $-MENU_PROMPT

; menu keys
EXIT_KEY db "0"
HELP_KEY db "1"

; messages
WRONG_INPUT_MSG db "Wrong input", 10
WRONG_INPUT_MSG_LEN = $-WRONG_INPUT_MSG

HELP_PROMPT db "This is help fn", 10
HELP_PROMPT_LEN = $-HELP_PROMPT

segment readable executable
main:
    log_print MENU_PROMPT, MENU_PROMPT_LEN, 0
    prompt_input

    cmp al, [EXIT_KEY]
    je exit_fn

    cmp al, [HELP_KEY]
    je help_fn

    log_print WRONG_INPUT_MSG, WRONG_INPUT_MSG_LEN, 0
    jmp main

exit_fn:
    mov rax, 60
    xor rdi, rdi
    syscall

help_fn:
    log_print HELP_PROMPT, HELP_PROMPT_LEN, 3
    jmp main
