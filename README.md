# Rust_mini_OS — Architecture & Boot Guide

> What runs where, how we boot, and how to debug on QEMU + RISC‑V with OpenSBI.

## 1) TL;DR
<p align="center">
<img width="917" height="406" alt="sketch" src="https://github.com/user-attachments/assets/5b843fbc-7e27-4d87-99e0-4cda0bc89e35" />
</p>

This project build an S‑mode (Supervisor) kernel in Rust.
OpenSBI (M‑mode firmware) boots first and hands control to us in S‑mode, exposing the SBI interface (putchar, set_timer, system_reset, …).
QEMU emulates the (virtual) RISC‑V hardware.
The kernel can either call OpenSBI via SBI or touch MMIO/CSRs directly (when allowed in S‑mode).

## 2) Build & Run(with Makefile)
```bash
# release build + execute
make run

# execute GDB(QEMU: -S -s)
make debug

# disassem(objdump)
make objdump

# check binary size
make size

# clean
make clean
```

## 3) References
- https://operating-system-in-1000-lines.vercel.app/ko/
- https://riscv.org/specifications/ratified/
- https://github.com/riscv-non-isa/riscv-sbi-doc?utm_source=chatgpt.com
- https://www.qemu.org/docs/master/system/target-riscv.html?utm_source=chatgpt.com
