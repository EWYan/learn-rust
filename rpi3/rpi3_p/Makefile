DOCKER_IMAGE := rustembedded/osdev-utils:2021.12

# Default to the RPi3.
BSP ?= rpi3

ifeq ($(BSP),rpi3)
    TARGET            = aarch64-unknown-none-softfloat
    KERNEL_BIN        = kernel8.img
    QEMU_BINARY       = qemu-system-aarch64
    QEMU_MACHINE_TYPE = raspi3
    QEMU_RELEASE_ARGS = -serial stdio -display none
    OBJDUMP_BINARY    = aarch64-none-elf-objdump
    NM_BINARY         = aarch64-none-elf-nm
    READELF_BINARY    = aarch64-none-elf-readelf
    LD_SCRIPT_PATH    = $(shell pwd)/src/bsp/raspberrypi
    RUSTC_MISC_ARGS   = -C target-cpu=cortex-a53
else ifeq ($(BSP),rpi4)
    TARGET            = aarch64-unknown-none-softfloat
    KERNEL_BIN        = kernel8.img
    QEMU_BINARY       = qemu-system-aarch64
    QEMU_MACHINE_TYPE =
    QEMU_RELEASE_ARGS = -serial stdio -display none
    OBJDUMP_BINARY    = aarch64-none-elf-objdump
    NM_BINARY         = aarch64-none-elf-nm
    READELF_BINARY    = aarch64-none-elf-readelf
    LD_SCRIPT_PATH    = $(shell pwd)/src/bsp/raspberrypi
    RUSTC_MISC_ARGS   = -C target-cpu=cortex-a72
endif

# Export for build.rs.
export LD_SCRIPT_PATH

KERNEL_MANIFEST      = Cargo.toml
KERNEL_LINKER_SCRIPT = kernel.ld
LAST_BUILD_CONFIG    = target/$(BSP).build_config
KERNEL_ELF      = target/$(TARGET)/release/kernel
KERNEL_ELF_DEPS = $(filter-out %: ,$(file < $(KERNEL_ELF).d)) $(KERNEL_MANIFEST) $(LAST_BUILD_CONFIG)


RUSTFLAGS = $(RUSTC_MISC_ARGS)                   \
    -C link-arg=--library-path=$(LD_SCRIPT_PATH) \
    -C link-arg=--script=$(KERNEL_LINKER_SCRIPT)

RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) \
    -D warnings                   

FEATURES      = --features bsp_$(BSP)
COMPILER_ARGS = --target=$(TARGET) \
    $(FEATURES)                    \
    --release

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
DOC_CMD     = cargo doc $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy \
    --strip-all            \
    -O binary

EXEC_QEMU = $(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE)

##------------------------------------------------------------------------------
## Dockerization
##------------------------------------------------------------------------------
DOCKER_CMD          = docker run -t --rm -v $(shell pwd):/work/tutorial -w /work/tutorial
DOCKER_CMD_INTERACT = $(DOCKER_CMD) -i

# DOCKER_IMAGE defined in include file (see top of this file).
DOCKER_QEMU  = $(DOCKER_CMD_INTERACT) $(DOCKER_IMAGE)
DOCKER_TOOLS = $(DOCKER_CMD) $(DOCKER_IMAGE)



##--------------------------------------------------------------------------------------------------
## Targets
##--------------------------------------------------------------------------------------------------
.PHONY: all doc qemu clippy clean readelf objdump nm check

all: $(KERNEL_BIN)

$(LAST_BUILD_CONFIG):
	@rm -f target/*.build_config
	@mkdir -p target
	@touch $(LAST_BUILD_CONFIG)


$(KERNEL_ELF): $(KERNEL_ELF_DEPS)
	$(call color_header, "Compiling kernel ELF - $(BSP)")
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

$(KERNEL_BIN): $(KERNEL_ELF)
	$(call color_header, "Generating stripped binary")
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)
	$(call color_progress_prefix, "Name")
	@echo $(KERNEL_BIN)
	$(call color_progress_prefix, "Size")
	$(call disk_usage_KiB, $(KERNEL_BIN))


qemu: $(KERNEL_BIN)
	$(call color_header, "Launching QEMU")
	$(DOCKER_QEMU) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN)


clean:
	rm -rf target $(KERNEL_BIN)