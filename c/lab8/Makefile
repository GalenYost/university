MAKEFLAGS += --no-print-directory

FILE_EXT := c
BUILD_DIR := build
CC ?= gcc
CFLAGS ?= -O2 -Wall -Wextra -I.
EXE ?= app
FILES := $(wildcard *.$(FILE_EXT))
SRC := $(notdir $(FILES))
OBJS := $(patsubst %.$(FILE_EXT),$(BUILD_DIR)/%.o,$(SRC))

all: build $(EXE)

$(EXE): $(OBJS)
	@$(CC) $(OBJS) -o $@

$(BUILD_DIR)/%.o: %.$(FILE_EXT) | build
	@$(CC) $(CFLAGS) -c $< -o $@

debug:
	@$(MAKE) run CFLAGS="$(CFLAGS) -DDEBUG_MODE"

build:
	@mkdir -p $(BUILD_DIR)

execute: all
	@./$(EXE)

run: execute clean

clean:
	@rm -rf $(BUILD_DIR) $(DEBUG_BUILD_DIR) $(EXE)
