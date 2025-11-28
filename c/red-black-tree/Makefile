MAKEFLAGS += --no-print-directory

FILE_EXT := c
BUILD_DIR := build

CC ?= clang
CFLAGS ?= -O2 -I. -Wall -Wextra -DLOG_USE_COLOR

EXE ?= app

FILES := $(shell find ./ -name "*.$(FILE_EXT)")
OBJS := $(patsubst %.c,$(BUILD_DIR)/%.o,$(FILES))

all: build $(EXE)

$(EXE): $(OBJS)
	@$(CC) $(OBJS) -o $@

$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	@$(CC) $(CFLAGS) -c $< -o $@

build:
	@mkdir -p $(BUILD_DIR)

test:
	@$(MAKE) run CFLAGS="$(CFLAGS) -DTEST_MODE"

execute: all
	@./$(EXE)

run: execute clean
release: all
	@rm -rf $(BUILD_DIR)

clean:
	@rm -rf $(EXE) $(BUILD_DIR)/*
