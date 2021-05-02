TARGET		:=	riscv64gc-unknown-none-elf
MODE		:=	release
KERNEL_ELF	:=	target/$(TARGET)/$(MODE)/os
KERNEL_BIN	:=	$(KERNEL_ELF).bin
DISASM_TMO	:=	target/$(TARGET)/$(MODE)/asm

# Board
BOARD		?=	qemu
SBI			?=	rustsbi
BOOTLOADER	:=	../bootloader/$(SBI)-$(BOARD).bin
K210_BOOTLOADER_SIZE := 131072

# KERNEL ENTRY
ifeq ($(BOARD), qemu)
	KERNEL_ENTRY_PA	:=	0x80200000
else
	KERNEL_ENTRY_PA	:=	0x80020000
endif

# Run K210
K210-SERIALPORT	=	/dev/tty.usbserial-10
K210-BURNER		=	../tools/kflash.py

# Binutils
OBJDUMP		:=	rust-objdump --arch-name=riscv64
OBJCOPY		:=	rust-objcopy --binary-architecture=riscv64

build: $(KERNEL_BIN)

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils --vers ~0.2
	rustup component add rust-src
	rustup component add llvm-tools-preview

kernel:
	@echo Platform: $(BOARD)
	@cp src/linker-$(BOARD).ld src/linker.ld
	@cargo build --release
	@rm src/linker.ld

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

run: run-inner

run-inner: 
ifeq ($(BOARD), qemu)
	@qemu-system-riscv64	\
		-machine virt		\
		-nographic			\
		-bios $(BOOTLOADER)	\
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)
else
	(which $(K210-BURNER)) || (cd .. && git clone https://github.com/sipeed/kflash.py.git && mv kflash.py tools)
	@cp $(BOOTLOADER) $(BOOTLOADER).copy
	@dd if=$(KERNEL_BIN) of=$(BOOTLOADER).copy bs=$(K210_BOOTLOADER_SIZE) seek=1
	@mv $(BOOTLOADER).copy $(KERNEL_BIN)
	@sudo chmod 777 $(K210-SERIALPORT)
	python3 $(K210-BURNER) -p $(K210-SERIALPORT) -b 1500000 $(KERNEL_BIN)
	python3 -m serial.tools.miniterm --eol LF --dtr 0 --rts 0 --filter direct $(K210-SERIALPORT) 115200
endif

debug: 
	@tmux new-session -d	\
		"qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S" && \
		tmux split-window -h "gdb-multiarch -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234' -ex 'set disassemble-next-line on' " &&	\
		tmux -2 attach-session -d

clean:
	@cargo clean

.PHONY:	build env run-inner
