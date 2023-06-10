# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm
FS_IMG := ../user/target/$(TARGET)/$(MODE)/fs.img
# FS_IMG_PRE := ../preliminary/fs.img
FS_IMG_FAT32_PREFIX := ../fat32-src
FS_IMG_FAT32 := $(FS_IMG_FAT32_PREFIX)/fat32.img
# FS_IMG_FAT32 := ../fat32-fuse/fat32.img
PRELIMINARY = 1

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
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly
DISASM ?= -x

# Run usertests or usershell
TEST ?=

# Qemu opts
ifndef CPUS
CPUS := 2
endif



build: env $(KERNEL_BIN) fs-img

# build-pre: env $(KERNEL_BIN) fs-img-pre

build-fat32: env $(KERNEL_BIN) fs-img-fat32

env:
	# (rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	# cargo install cargo-binutils
	# rustup component add rust-src
	# rustup component add llvm-tools-preview

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

# kernel:
# 	@echo "build user and kernel"
# 	@cd ../user && make build TEST=$(TEST) PRELIMINARY=$(PRELIMINARY)
# 	@echo Platform: $(BOARD)
# 	@cp src/linker-$(BOARD).ld src/linker.ld
# 	@cargo build --release
# 	@rm src/linker.ld
kernel-bin: $(KERNEL_BIN)

kernel:
	@echo "build user and kernel"
	@cd ../user && make build TEST=$(TEST) PRELIMINARY=$(PRELIMINARY)
	@echo Platform: $(BOARD)
	@cp src/linker64.ld src/linker.ld
	@cargo build --release --offline
	@rm src/linker.ld


fs-img-fat32: $(APPS)
	@cd ../user && make build TEST=$(TEST)
	@mkdir -p  $(FS_IMG_FAT32_PREFIX)
	@rm -rf $(FS_IMG_FAT32)
	@dd if=/dev/zero of=$(FS_IMG_FAT32) bs=512 count=147456
	@cd ../fat32-fuse && cargo run --release  -- -f $(FS_IMG_FAT32) -s ../user/src/bin/ -t ../preliminary/ -e ../user/target/riscv64gc-unknown-none-elf/release/
	@echo "finish pack fs-img"

fs-img: $(APPS)
	@cd ../user && make build TEST=$(TEST)
	@rm -f $(FS_IMG)
	@cd ../easy-fs-fuse && cargo run --release -- -s ../user/src/bin/ -t ../user/target/riscv64gc-unknown-none-elf/release/


# kernel_test:
# 	@cd ../user && make build TEST=$(TEST)
# 	@echo Platform: $(BOARD)
# 	@cp src/linker-$(BOARD).ld src/linker.ld
# 	@cargo test --release
# 	@rm src/linker.ld

clean:
	@cargo clean

disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

disasm-vim: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)

run: run-inner

run-fat32: run-inner-fat32

run-inner: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-smp $(CPUS) \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
        -drive file=$(FS_IMG),if=none,format=raw,id=x0 \
        -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 


run-inner-fat32: build-fat32
	@echo "start to run kernel in qemu..."
	@echo $(KERNEL_BIN)
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-smp $(CPUS) \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
        -drive file=$(FS_IMG_FAT32),if=none,format=raw,id=x0 \
        -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 


debug: build-pre
	@tmux new-session -d \
		"qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-drive file=$(FS_IMG_PRE),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-smp $(CPUS) \
		-s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

gdbserver: build-pre-fat32
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-smp $(CPUS) \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
        -drive file=$(FS_IMG_FAT32),if=none,format=raw,id=x0 \
        -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0  \
		-s -S



gdbclient:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

.PHONY: build env kernel clean disasm disasm-vim run-inner gdbserver gdbclient