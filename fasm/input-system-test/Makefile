MAKEFLAGS += --no-print-directory

SRCS := $(wildcard *.asm)
OUT ?= avl

all: run clean

run: $(OUT)
	@./$(OUT)

build: $(OUT)

clean:
	@rm -rf ./$(OUT)

$(OUT):
	@fasm main.asm $(OUT) > /dev/null 2>&1
