#
# Test and build holo-hosting-app Project
#
# This Makefile is primarily instructional; you can simply enter the Nix environment for
# holochain-rust development (supplied by holo=nixpkgs; see pkgs.nix) via `nix-shell` and run `hc
# test` directly, or build a target directly (see default.nix), eg. `nix-build -A holo-hosting-app`
# TODO: ensure buildDNA works for multi-zome DNAs
# 
SHELL		= bash
DNANAME		= holo-hosting-app
DNA		= dist/$(DNANAME).dna.json

# External targets; Uses a nix-shell environment to obtain Holochain runtimes, run tests, etc.
.PHONY: all
all: nix-test

# nix-test, nix-install, ...
nix-%:
	nix-shell --pure --run "make $*"

# Internal targets; require a Nix environment in order to be deterministic.
# - Uses the version of `hc`, `holochain` on the system PATH.
# - Normally called from within a Nix environment, eg. run `nix-shell` from within holo-hosting-app
.PHONY:		rebuild install build test test-unit test-e2e
rebuild:	clean build

install:	build

build:		$(DNA)

# Build the DNA; Specifying a custom --output requires the path to exist
# However, if the name of the directory within which `hc` is run matches the
# DNA's name, then this name is used by default, and the output directory is
# created automatically.
$(DNA):
	hc package

test:		test-unit test-e2e

# test-unit -- Run Rust unit tests via Cargo
test-unit:
	RUST_BACKTRACE=1 cargo test \
	  	--manifest-path zomes/host/code/Cargo.toml \
	    -- --nocapture
	RUST_BACKTRACE=1 cargo test \
	  	--manifest-path zomes/whoami/code/Cargo.toml \
	    -- --nocapture
	RUST_BACKTRACE=1 cargo test \
	  	--manifest-path zomes/provider/code/Cargo.toml \
	    -- --nocapture

# End-to-end test of DNA.  Runs a sim2h_server on localhost:9000; the default expected by `hc test`
test-e2e:	$(DNA)
	@echo "Setting up Scenario test Javascript..."; \
	    ( cd test && npm install );
	@echo "Starting sim2h_server on localhost:9000 (may already be running)..."; \
	    sim2h_server -p 9000 &
	@echo "Starting Scenario tests..."; \
	    RUST_BACKTRACE=1 hc test \

#	        | test/node_modules/faucet/bin/cmd.js

# Generic targets; does not require a Nix environment
.PHONY: clean
clean:
	rm -rf \
	    dist \
	    test/node_modules \
	    .cargo \
	    target \
	    zomes/*/code/target

