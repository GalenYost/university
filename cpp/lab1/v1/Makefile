FILE_EXT := cpp
BUILD_DIR := build
DEBUG_BUILD_DIR := debug
CXX ?= g++
CXXFLAGS ?= -Wall -Wextra -O2
DEBUG_FLAGS ?= -g -O0
EXE ?= app
DEBUG_EXE := $(DEBUG_BUILD_DIR)/$(EXE)
FILES := $(wildcard *.$(FILE_EXT))
SRC := $(notdir $(FILES))
OBJS := $(patsubst %.$(FILE_EXT),$(BUILD_DIR)/%.o,$(SRC))
DEBUG_OBJS := $(patsubst %.$(FILE_EXT),$(DEBUG_BUILD_DIR)/%.o,$(SRC))

all: build $(EXE)

$(EXE): $(OBJS)
	@$(CXX) $(OBJS) -o $@

$(BUILD_DIR)/%.o: %.$(FILE_EXT) | build
	@$(CXX) $(CXXFLAGS) -c $< -o $@

debug: $(DEBUG_EXE)
	@gdb $(DEBUG_EXE)
	@$(MAKE) clean

$(DEBUG_EXE): $(DEBUG_OBJS)
	@$(CXX) $(DEBUG_OBJS) -o $@

$(DEBUG_BUILD_DIR)/%.o: %.$(FILE_EXT) | build
	@mkdir -p $(DEBUG_BUILD_DIR)
	@$(CXX) $(DEBUG_FLAGS) -c $< -o $@

build:
	@mkdir -p $(BUILD_DIR)

execute: all
	@./$(EXE)

run: execute clean

clean:
	@rm -rf $(BUILD_DIR) $(DEBUG_BUILD_DIR) $(EXE)
