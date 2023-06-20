# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/kernel
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm
# FS_IMG := ../user/target/$(TARGET)/$(MODE)/fs.img
FS_IMG_FAT32_PREFIX := ../fat32-src
FS_IMG_FAT32 := $(FS_IMG_FAT32_PREFIX)/fat32.img
PRELIMINARY = 0

# BOARD
BOARD := qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# KERNEL ENTRY
KERNEL_ENTRY_PA := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJDUMP_PATH := ../objdump/kernel.asm
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly
DISASM ?= -d

# Run usertests or usershell
TEST ?=

# Qemu opts
ifndef CPUS
CPUS := 2
endif


QFLAGS := 
QFLAGS += -machine virt 
QFLAGS += -nographic
QFLAGS += -smp $(CPUS)
QFLAGS += -kernel $(KERNEL_BIN)
# QFLAGS += -bios $(BOOTLOADER) 
QFLAGS += -drive file=$(FS_IMG_FAT32),if=none,format=raw,id=x0 
QFLAGS += -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 


# build-pre: env $(KERNEL_BIN) fs-img-pre

build: env $(KERNEL_BIN) fs-img


env:
	# (rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	# cargo install cargo-binutils
	# rustup component add rust-src
	# rustup component add llvm-tools-preview


$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@


kernel-bin: $(KERNEL_BIN)


kernel:
	@echo "build user and kernel"
	@cd ../user && make build TEST=$(TEST) PRELIMINARY=$(PRELIMINARY)
	@echo Platform: $(BOARD)
	@cp src/linker64.ld src/linker.ld
	@cargo build --release 
	@rm src/linker.ld


fs-img: $(APPS)
	@cd ../user && make build TEST=$(TEST)
	@mkdir -p  $(FS_IMG_FAT32_PREFIX)
	@rm -rf $(FS_IMG_FAT32)
	@dd if=/dev/zero of=$(FS_IMG_FAT32) bs=512 count=147456
	@cd ../fat32-fuse && cargo run --release  -- -f $(FS_IMG_FAT32) -s ../user/src/bin/ -t ../testcases/preliminary/ -e ../user/target/riscv64gc-unknown-none-elf/release/
	@echo "finish pack fs-img"


clean:
	@cargo clean


objdump: 
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(OBJDUMP_PATH)
	@echo "kernel.bin has been dumped to $(OBJDUMP_PATH)"


disasm-vim: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)


run: run-inner


run-fat32:
	@echo "No target for run-fat32. You should input: [make run]:)"


run-inner: build
	@echo "start to run kernel in qemu..."
	@echo $(KERNEL_BIN)
	@qemu-system-riscv64 $(QFLAGS)


debug: build
	@tmux new-session -d \
		"qemu-system-riscv64 $(QFLAGS) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d


gdbserver: build
	@qemu-system-riscv64 $(QFLAGS) -s -S


gdbclient:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'


.PHONY: build env kernel clean disasm disasm-vim run-inner gdbserver gdbclient