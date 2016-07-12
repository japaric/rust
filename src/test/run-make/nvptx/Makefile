-include ../tools.mk

NVPTX = nvptx-unknown-unknown
NVPTX64 = nvptx64-unknown-unknown

all: smoke syncthreads thread_idx

smoke:
	$(RUSTC) --target $(NVPTX) --emit=asm $@.rs
	grep '\.address_size *32' $(TMPDIR)/$@.s
	$(RUSTC) --target $(NVPTX64) --emit=asm $@.rs
	grep '\.address_size *64' $(TMPDIR)/$@.s

syncthreads:
	$(RUSTC) --target $(NVPTX) --emit=llvm-ir $@.rs
	grep 'call.*syncthreads' $(TMPDIR)/$@.ll
	$(RUSTC) --target $(NVPTX64) --emit=llvm-ir $@.rs
	grep 'call.*syncthreads' $(TMPDIR)/$@.ll
	$(RUSTC) --target $(NVPTX) --emit=asm $@.rs
	grep 'bar\.sync' $(TMPDIR)/$@.s
	$(RUSTC) --target $(NVPTX64) --emit=asm $@.rs
	grep 'bar\.sync' $(TMPDIR)/$@.s

thread_idx:
	$(RUSTC) --target $(NVPTX) --emit=llvm-ir $@.rs
	grep 'call.*tid\.x' $(TMPDIR)/$@.ll
	$(RUSTC) --target $(NVPTX64) --emit=llvm-ir $@.rs
	grep 'call.*tid\.x' $(TMPDIR)/$@.ll
	$(RUSTC) --target $(NVPTX) --emit=asm $@.rs
	grep '%tid\.x' $(TMPDIR)/$@.s
	$(RUSTC) --target $(NVPTX64) --emit=asm $@.rs
	grep '%tid\.x' $(TMPDIR)/$@.s
