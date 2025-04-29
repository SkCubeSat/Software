#############################
#### CubeObcLib Makefile ####
#############################

CUBEOBCLIB := libcubeobc
CUBEOBCLIB_ROOT := $(patsubst %/,%,$(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

#######################
#### BUILD OPTIONS ####
#######################

# Build directory: default to library root directory
BUILD_DIR ?= $(CUBEOBCLIB_ROOT)/build

# Compiler
CC ?= gcc

# Architecture: default to posix
ARCH ?= posix

SOCKETCAN ?=
LIBSOCKETCAN_SH ?= /usr/local/lib/libsocketcan.a

TERMIOS ?=

# Default to not using packed structs for tctlm
USE_TCTLM_PACKED ?= 0

CUBEOBCLIB_CFLAGS_DEBUG ?= -g -O0 -Wall -Wextra -Wno-unused-parameter -Werror
CUBEOBCLIB_CFLAGS_RELEASE ?= -g -O1 -Wall -Wextra -Wno-unused-parameter -Werror

# Verbosity
ifeq ($(VERBOSE),1)
	V=
else
	V=@
endif

###############
#### BUILD ####
###############

# The CUBEOBCLIB_SUBDIR variable is an optional set (?=) for internal use at CubeSpace only - It is not intended to be a build option
ifeq ($(DEBUG),)
	CUBEOBCLIB_SUBDIR ?= /RELEASE
	CUBEOBCLIB_CFLAGS := $(CUBEOBCLIB_CFLAGS_RELEASE)
else
	CUBEOBCLIB_SUBDIR ?= /DEBUG
	CUBEOBCLIB_CFLAGS := $(CUBEOBCLIB_CFLAGS_DEBUG)
endif

CUBEOBCLIB_BDIR := $(BUILD_DIR)/$(CUBEOBCLIB)$(CUBEOBCLIB_SUBDIR)

# The compiled shared library
CUBEOBCLIB_SH := $(CUBEOBCLIB_BDIR)/$(CUBEOBCLIB).a

# Retrieve library version number from file
include $(CUBEOBCLIB_ROOT)/version
CUBEOBCLIB_VERSION_MAJOR := $(word 1, $(subst ., , $(VERSION)))
CUBEOBCLIB_VERSION_MINOR := $(word 2, $(subst ., , $(VERSION)))
CUBEOBCLIB_VERSION_PATCH := $(word 3, $(subst ., , $(VERSION)))

# Retrieve system version number from file
include $(CUBEOBCLIB_ROOT)/systemversion
CUBEOBCLIB_SYSTEM_VERSION_MAJOR := $(word 1, $(subst ., , $(SYSTEM_VERSION)))
CUBEOBCLIB_SYSTEM_VERSION_MINOR := $(word 2, $(subst ., , $(SYSTEM_VERSION)))
CUBEOBCLIB_SYSTEM_VERSION_PATCH := $(word 3, $(subst ., , $(SYSTEM_VERSION)))

# Directory constants
CUBEOBCLIB_API = $(CUBEOBCLIB_ROOT)/api
CUBEOBCLIB_API_XML = $(CUBEOBCLIB_API)/xml
CUBEOBCLIB_API_SRC = $(CUBEOBCLIB_API)/src

CUBEOBCLIB_LD_FLAGS =
CUBEOBCLIB_LD_FLAGS += -lm

CUBEOBCLIB_CFLAGS_DEFS =
CUBEOBCLIB_CFLAGS_DEFS += -DVERSION_MAJOR=$(CUBEOBCLIB_VERSION_MAJOR)
CUBEOBCLIB_CFLAGS_DEFS += -DVERSION_MINOR=$(CUBEOBCLIB_VERSION_MINOR)
CUBEOBCLIB_CFLAGS_DEFS += -DVERSION_PATCH=$(CUBEOBCLIB_VERSION_PATCH)
CUBEOBCLIB_CFLAGS_DEFS += -DSYSTEM_VERSION_MAJOR=$(CUBEOBCLIB_SYSTEM_VERSION_MAJOR)
CUBEOBCLIB_CFLAGS_DEFS += -DSYSTEM_VERSION_MINOR=$(CUBEOBCLIB_SYSTEM_VERSION_MINOR)
CUBEOBCLIB_CFLAGS_DEFS += -DSYSTEM_VERSION_PATCH=$(CUBEOBCLIB_SYSTEM_VERSION_PATCH)
CUBEOBCLIB_CFLAGS_DEFS += -DUSE_TCTLM_PACKED=$(USE_TCTLM_PACKED)
ifeq ($(ARCH),posix)
    ifeq ($(SOCKETCAN),1)
    CUBEOBCLIB_CFLAGS_DEFS += -DSOCKETCAN	
    endif
    ifeq ($(TERMIOS),1)
    CUBEOBCLIB_CFLAGS_DEFS += -DTERMIOS
    endif
endif

CUBEOBCLIB_SRC =
CUBEOBCLIB_SRC += $(filter-out $(wildcard $(CUBEOBCLIB_ROOT)/examples/*.c), $(wildcard $(CUBEOBCLIB_ROOT)/*/*.c))
CUBEOBCLIB_SRC += $(wildcard $(CUBEOBCLIB_ROOT)/*/*/*.c)
CUBEOBCLIB_SRC += $(wildcard $(CUBEOBCLIB_ROOT)/*/drivers/*/*.c)
CUBEOBCLIB_SRC += $(wildcard $(CUBEOBCLIB_ROOT)/*/*/$(ARCH)/*.c)

CUBEOBCLIB_INC =
CUBEOBCLIB_INC += $(CUBEOBCLIB_ROOT)/api/src
CUBEOBCLIB_INC += $(CUBEOBCLIB_ROOT)/include

CUBEOBCLIB_OBJ =
CUBEOBCLIB_OBJ += $(subst $(CUBEOBCLIB_ROOT),$(CUBEOBCLIB_BDIR),$(patsubst %.c,%.o,$(CUBEOBCLIB_SRC)))

.PHONY: $(CUBEOBCLIB)
$(CUBEOBCLIB): info-$(CUBEOBCLIB) $(CUBEOBCLIB_SH)

clean: $(CUBEOBCLIB)-clean

.PHONY: $(CUBEOBCLIB)-clean
$(CUBEOBCLIB)-clean:
	$(V)rm -fr $(BUILD_DIR)/$(CUBEOBCLIB)

.PHONY: info-$(CUBEOBCLIB)
info-$(CUBEOBCLIB):
	@echo -e "$(P_WHITE)=============================================================$(P_RESET)"
	@echo -e "$(P_WHITE):: LIBCUBEOBC BUILD :: $(P_RESET)"
	@echo -e "$(P_WHITE)=============================================================$(P_RESET)"
	@echo -e

# Compile shared library
$(CUBEOBCLIB_SH): $(CUBEOBCLIB_SRC) $(CUBEOBCLIB_ROOT)/Makefile
	@echo -e "[SH]\t$@"
	$(V)mkdir -p $(CUBEOBCLIB_BDIR)
	$(V)$(CC) -shared $(CUBEOBCLIB_CFLAGS) $(CUBEOBCLIB_CFLAGS_DEFS) $(CUBEOBCLIB_LD_FLAGS) $(addprefix -I,$(CUBEOBCLIB_INC)) $(addprefix -I,$(USER_INC)) -o $(CUBEOBCLIB_SH) $(CUBEOBCLIB_SRC)

##################
#### EXAMPLES ####
##################
CUBEOBCLIB_EX_SRC = $(wildcard $(CUBEOBCLIB_ROOT)/examples/*.c)
CUBEOBCLIB_EX_OBJ = $(subst $(CUBEOBCLIB_ROOT),$(CUBEOBCLIB_BDIR),$(patsubst %.c,%.o,$(CUBEOBCLIB_EX_SRC)))
CUBEOBCLIB_EX_ELF = $(patsubst %.o,%.elf,$(CUBEOBCLIB_EX_OBJ))

.SECONDARY: $(CUBEOBCLIB_EX_OBJ)

.PHONY: info-$(CUBEOBCLIB)-examples
info-$(CUBEOBCLIB)-examples:
	@echo -e "$(P_WHITE)=============================================================$(P_RESET)"
	@echo -e "$(P_WHITE):: LIBCUBEOBC EXAMPLES BUILD :: $(P_RESET)"
	@echo -e "$(P_WHITE)=============================================================$(P_RESET)"
	@echo -e

.PHONY: $(CUBEOBCLIB)-examples
$(CUBEOBCLIB)-examples: info-$(CUBEOBCLIB)-examples $(CUBEOBCLIB_SH) $(CUBEOBCLIB_EX_ELF)

$(CUBEOBCLIB_BDIR)/examples/%.elf: $(CUBEOBCLIB_ROOT)/examples/%.c $(CUBEOBCLIB_ROOT)/Makefile
	@echo -e "[EX]\t$@"
	$(V)mkdir -p $(dir $@)
	$(V)$(CC) $(CUBEOBCLIB_CFLAGS) $(CUBEOBCLIB_CFLAGS_DEFS) $(USER_CFLAGS_DEFS) $(addprefix -I,$(CUBEOBCLIB_INC)) $(addprefix -I,$(USER_INC)) $< $(CUBEOBCLIB_SH) $(LIBCSP_SH) $(LIBSOCKETCAN_SH) $(CUBEOBCLIB_LD_FLAGS) -o $@
