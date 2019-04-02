RUSTC = rustc
RUSTFMT = rustfmt

MAIN = main.rs

all: main

main: main.rs
	$(RUSTC) $(MAIN)

release:
	$(RUSTC) -O $(MAIN)

format:
	$(RUSTFMT) $(MAIN)
