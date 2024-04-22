# Solana Makefile

# Variables
CARGO = cargo
SRC_DIR = src
TARGET_DIR = target

# Targets
.PHONY: all build clean test doc

all: build

build: clean-deploy
	$(CARGO) build-bpf

clean:
	$(CARGO) clean
	rm -rf $(TARGET_DIR)

clean-deploy:
	rm -rf $(TARGET_DIR)/deploy

test:
	$(CARGO) test

doc:
	$(CARGO) doc

integration-test:
	cargo test-bpf

airdrop:
	solana airdrop 5

deploy: airdrop
	solana program deploy ./target/deploy/solana_gov.so