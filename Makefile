ARCH ?= rv32

ifeq ($(ARCH),rv32)
	TARGET		:= riscv32imac-unknown-none-elf
	QEMU		:= qemu-system-riscv32
	FIRMWARE 	?= opensbi-riscv32-generic-fw_dynamic.bin
else ifeq ($(ARCH),rv64)
	TARGET 		:= riscv64gc-unknown-none-elf
	QEMU 		:= qemu-system-riscv64
	FIRMWARE 	?= default
else
	$(error ARCH must be rv32 or rv64)
endif

KERNEL := target/$(TARGET)/release/rust_kernel
QEMU_BASE := -machine virt -nographic -serial mon:stdio

OBJDUMP := $(shell command -v llvm-objdump 2>/dev/null || \
                  command -v rust-objdump 2>/dev/null || \
                  command -v riscv64-unknown-elf-objdump 2>/dev/null || \
                  command -v riscv32-unknown-elf-objdump 2>/dev/null || \
                  echo "")

SIZEBIN := $(shell command -v llvm-size 2>/dev/null || \
                  command -v rust-size 2>/dev/null || \
                  command -v riscv64-unknown-elf-size 2>/dev/null || \
                  command -v riscv32-unknown-elf-size 2>/dev/null || \
                  command -v size 2>/dev/null || \
                  echo "")

.PHONY: help build run debug gdb objdump size clean setup-mac setup-rust

help:
	@echo "Usage:"
	@echo "  make run 			# 빌드 후 QEMU 실행"
	@echo "  make debug			# GDB 디버깅용(-S -s)으로 실행"
	@echo "  make objdump		# Disasm 보기(llvm-objdump 필요)"
	@echo "  make size			# 바이너리 사이즈"
	@echo "  make clean			# 정리"
	@echo "  make setup-mac		# macOS 도구 설치(llvm, qemu)"
	@echo "  make setup-rust	# rust-objdump 사용 준비(cargo-binutils)"

build:
	cargo build --release

run: build
	@BIOS_OPT=""; \
	if [ "$(ARCH)" = "rv32" ]; then \
		if [ -f "$(FIRMWARE)" ]; then \
			BIOS_OPT="-bios $(FIRMWARE)"; \
		else \
			BIOS_OPT="-bios default"; \
		fi; \
	else \
		BIOS_OPT="-bios default"; \
	fi; \
	$(QEMU) $(QEMU_BASE) $$BIOS_OPT -kernel $(KERNEL)


debug: build
	@BIOS_OPT=""; \
	if [ "$(ARCH)" = "rv32" ]; then \
		if [ -f "$(FIRMWARE)" ]; then \
			BIOS_OPT="-bios $(FIRMWARE)"; \
		else \
			BIOS_OPT="-bios default"; \
		fi; \
	else \
		BIOS_OPT="-bios default"; \
	fi; \
	$(QEMU) $(QEMU_BASE) -S -s $$BIOS_OPT -kernel $(KERNEL)

gdb:
	@echo "Example (lldb): 	lldb -o 'gdb-remote 1234' $(KERNEL)"
	@echo "Example (gdb ):	riscv64-unknown-elf-gdb -ex 'target remote :1234' $(KERNEL)"

objdump:
	@if [ -z "$(OBJDUMP)" ]; then \
		echo "No objdump found. Run: 'make setup-mac' or 'make setup-rust'"; \
		exit 1; \
	fi
	@echo "Using: $(OBJDUMP)"
	@$(OBJDUMP) -d $(KERNEL) | less

size:
	@if [ -z "$(SIZEBIN)" ]; then \
		echo "No size tool found. Run: 'make setup-mac' or install a cross size tool"; exit 1; \
	fi
	@echo "Using: $(SIZEBIN)"
	@$(SIZEBIN) $(KERNEL)

clean:
	cargo clean

setup-mac:
	@which brew >/dev/null || (echo "Homebrew not found"; exit 1)
	brew install llvm qemu
	@echo "Tip: add LLVM to PATH if needed: 'export PATH=$$(brew --prefix llvm)/bin:$$PATH'"

setup-rust:
	rustup component add llvm-tools-preview
	cargo install cargo-binutils
	@echo "Now 'rust-objdump' and 'rust-size' are available."
