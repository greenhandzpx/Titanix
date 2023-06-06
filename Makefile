all:
	@cp bootloader/rustsbi-qemu.bin sbi-qemu
	@cd os && make kernel
	@cp os/target/riscv64gc-unknown-none-elf/release/os.bin kernel-qemu
