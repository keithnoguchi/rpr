# SPDX-License-Identifier: GPL-2.0
TARGET := checkers.wasm
.PHONY: all d dump c clean

all: dump
%.wasm: %.wat
	wat2wasm $^
d dump: $(TARGET)
	wasm-objdump -x $^
c clean:
	$(RM) $(TARGET)
