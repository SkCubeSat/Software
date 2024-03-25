# Define the names of executables
x86_64_exec_name = server_client_test_x86
arm_exec_name = csp_server_client_arm

# Define compilers and flags
CC = gcc
CC_ARM = arm-linux-gnueabihf-gcc
CFLAGS = -g -I. -I/usr/local/include
CPPFLAGS = -Wall -Wextra -pedantic

# Define directories
BUILD_DIR := build
X86_64_OBJ_DIR := $(BUILD_DIR)/x86_64/obj
ARM_OBJ_DIR := $(BUILD_DIR)/arm/obj
X86_64_BIN_DIR := $(BUILD_DIR)/x86_64/bin
ARM_BIN_DIR := $(BUILD_DIR)/arm/bin

# Libraries
arm_csp = /usr/local/lib/Linuxarmv7l
x86_64_csp = /usr/local/lib/Linuxx86_64

# List of source files
SRC_FILES := $(wildcard src/*.c)

# List of object files x86_64
X86_64_OBJ_FILES := $(patsubst src/%.c,$(X86_64_OBJ_DIR)/%.o,$(SRC_FILES))

# List of object files ARM
ARM_OBJ_FILES := $(patsubst src/%.c,$(ARM_OBJ_DIR)/%.o,$(SRC_FILES))

# Main target
all: x86_64 arm

# Create directories
$(BUILD_DIR) $(X86_64_OBJ_DIR) $(ARM_OBJ_DIR) $(X86_64_BIN_DIR) $(ARM_BIN_DIR):
	mkdir -p $@

# Rule to compile C files x86_64
$(X86_64_OBJ_DIR)/%.o: src/%.c | $(X86_64_OBJ_DIR)
	$(CC) $(CFLAGS) $(CPPFLAGS) -o $@ -c $<

# Rule to compile C files ARM
$(ARM_OBJ_DIR)/%.o: src/%.c | $(ARM_OBJ_DIR)
	$(CC_ARM) $(CFLAGS) $(CPPFLAGS) -o $@ -c $<

# Rule to link executables
$(X86_64_BIN_DIR)/$(x86_64_exec_name): $(X86_64_OBJ_FILES) | $(X86_64_BIN_DIR)
	$(CC) $(CFLAGS) $(CPPFLAGS) -o $@ $^ -L$(x86_64_csp) -pthread -lcsp

$(ARM_BIN_DIR)/$(arm_exec_name): $(ARM_OBJ_FILES) | $(ARM_BIN_DIR)
	$(CC_ARM) $(CFLAGS) $(CPPFLAGS) -o $@ $^ -L$(arm_csp) -pthread -lcsp

# Target for x86_64
x86_64: $(X86_64_BIN_DIR)/$(x86_64_exec_name)

# Target for ARM
arm: $(ARM_BIN_DIR)/$(arm_exec_name)

# Clean target
clean:
	rm -rf $(BUILD_DIR)

.PHONY: all clean x86_64 arm
