all:
	@rm -rf kernel/.cargo
	@cp -r kernel/cargo-submit kernel/.cargo
	@rm -rf user/.cargo
	@cp -r user/cargo-submit user/.cargo
	@cp bootloader/rustsbi-qemu.bin sbi-qemu
	@cd kernel && make kernel-bin
	@cp kernel/target/riscv64gc-unknown-none-elf/release/kernel.bin kernel-qemu
