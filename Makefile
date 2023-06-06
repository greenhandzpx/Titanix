all:
	@rm -rf os/.cargo
	@cp -r os/cargo-submit os/.cargo
	@rm -rf user/.cargo
	@cp -r user/cargo-submit user/.cargo
	@cp bootloader/rustsbi-qemu.bin sbi-qemu
	@cd os && make kernel-bin
	@cp os/target/riscv64gc-unknown-none-elf/release/os.bin kernel-qemu
