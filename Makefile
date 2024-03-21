# Solana Makefile

# Variables
CARGO = cargo
SRC_DIR = src
TARGET_DIR = target

# Targets
.PHONY: all build clean test doc

all: build

build:
	$(CARGO) build-bpf

clean:
	$(CARGO) clean
	rm -rf $(TARGET_DIR)

test:
	$(CARGO) test

doc:
	$(CARGO) doc

integration-test:
	cargo test-bpf

deploy:
	cargo deploy