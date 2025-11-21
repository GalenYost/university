MAKEFLAGS += --no-print-directory

FILE_EXT := cpp
BUILD_DIR := build
CXX ?= g++
CXXFLAGS ?= -O2 -I. -fPIC
EXE ?= app
FILES := $(wildcard *.$(FILE_EXT))
SRC := $(notdir $(FILES))
OBJS := $(patsubst %.$(FILE_EXT),$(BUILD_DIR)/%.o,$(SRC))

main:
	@$(CXX) -Llib.so main.cpp -o main
	@./main
	@rm -rf main

main2:
	@$(CXX) -I. main2.cpp wrapper.cpp lib.a -o main
	@./main
	@rm -rf main

$(EXE): $(OBJS)
	@$(CXX) $(OBJS) -o $@

$(BUILD_DIR)/%.o: %.$(FILE_EXT) | build
	@$(CXX) $(CXXFLAGS) -c $< -o $@

lib.a: $(OBJS)
	@ar rcs $@ $(OBJS)
	@$(MAKE) clean

lib.so: $(OBJS)
	@$(CXX) -shared -fPIC $(OBJS) -o $@
	@$(MAKE) clean
