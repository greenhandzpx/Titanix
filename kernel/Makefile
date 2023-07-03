# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/kernel
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm

PRELIMINARY = 0
FS_IMG_SRC := ../fs-img-src
FS_IMG := $(FS_IMG_SRC)/sdcard.img

TEST := 22
TEST_DIR := "../$(TEST)-testcases"

STRACE := 

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
OBJDUMP_PREFIX := ../objdump
OBJDUMP_PATH := $(OBJDUMP_PREFIX)/kernel.asm
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly
DISASM ?= -d

# Qemu opts
ifndef CPUS
CPUS := 2
endif
QFLAGS := 
QFLAGS += -m 128M
QFLAGS += -machine virt 
QFLAGS += -nographic
QFLAGS += -smp $(CPUS)
QFLAGS += -kernel $(KERNEL_BIN)
# uncomment the following to use rust-sbi
# QFLAGS += -bios $(BOOTLOADER) 
QFLAGS += -drive file=$(FS_IMG),if=none,format=raw,id=x0 
QFLAGS += -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 



build: env user-build $(KERNEL_BIN)
ifeq ("$(wildcard $(FS_IMG))","")
	@echo "$(TEST_DIR)"
	@echo "Cannot find the fs img. You should run: [sudo make fs-img] to build the fs img first. Note that you can add TEST=23 to build the 2023 img."
	@exit 1
endif


env:
	 (rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	 cargo install cargo-binutils
	 rustup component add rust-src
	 rustup component add llvm-tools-preview


$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@


kernel:
	@echo "build kernel..."
	# @cd ../user && make build TEST=$(TEST) 
	@echo Platform: $(BOARD)
	@cp src/linker64.ld src/linker.ld
	@cargo fmt
ifeq ($(STRACE), )
	@cargo build --release 
else
	@cargo build --release --features strace
endif
	@rm src/linker.ld
	@echo "build kernel finished"


user-build:
	@echo "build user..."
	@cd ../user && make build PRELIMINARY=$(PRELIMINARY)
	@cp ../22-testcases/busybox ../user/target/riscv64gc-unknown-none-elf/release/busybox
	@echo "build user finished"


fs-img: $(APPS)
	@rm -rf $(FS_IMG)
	@mkdir -p $(FS_IMG_SRC)
	@dd if=/dev/zero of=$(FS_IMG) count=62768 bs=1K
	@mkfs.vfat -F 32 $(FS_IMG)
ifeq ($(TEST), 0)
	@echo "make empty fatfs img"
else
	@mkdir -p mnt
	@sudo mount -t vfat -o user,umask=000,utf8=1 --source $(FS_IMG) --target mnt
	@cp -r $(TEST_DIR)/* mnt
	@sudo umount mnt
	@rm -rf mnt
	@sudo chmod -R 777 $(FS_IMG_SRC)
endif
	@echo "finish pack fs-img"



run: build
		@echo "start to run kernel in qemu..."
		@echo $(KERNEL_BIN)
		@qemu-system-riscv64 $(QFLAGS)


clean:
	@cargo clean
	@rm -rf $(FS_IMG)


objdump: 
	@mkdir -p $(OBJDUMP_PREFIX)
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(OBJDUMP_PATH)
	@echo "kernel.bin has been dumped to $(OBJDUMP_PATH)"


disasm-vim: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)


debug: build
	@tmux new-session -d \
		"qemu-system-riscv64 $(QFLAGS) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d


gdbserver: build
	@qemu-system-riscv64 $(QFLAGS) -s -S


gdbclient:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'


tftp-server: build
	sudo make -C ../tftp-server -f Makefile run


com:
	sudo picocom -b 115200 /dev/ttyUSB1


.PHONY: build env kernel clean disasm disasm-vim run-inner gdbserver gdbclient  tftp-server com
