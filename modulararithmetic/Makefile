RUSTC = rustc
RUSTFMT = rustfmt

RS_FILES = main.rs

all: main

main: $(RS_FILES)
	$(RUSTC) $(RS_FILES)

release: $(RS_FILES)
	$(RUSTC) -O $(RS_FILES)

format:
	$(RUSTFMT) $(RS_FILES)

.PHONY: format

