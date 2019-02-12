# holochain-rust Makefile

SHELL = /bin/bash
CORE_RUST_VERSION ?= nightly-2019-01-24
TOOLS_RUST_VERSION ?= nightly-2019-01-24
CARGO = RUSTFLAGS="-Z external-macro-backtrace -D warnings" RUST_BACKTRACE=1 rustup run $(CORE_RUST_VERSION) cargo $(CARGO_ARGS)
CARGO_TOOLS = RUSTFLAGS="-Z external-macro-backtrace -D warnings" RUST_BACKTRACE=1 rustup run $(TOOLS_RUST_VERSION) cargo $(CARGO_ARGS)
CARGO_TARPULIN_INSTALL = RUSTFLAGS="--cfg procmacro2_semver_exempt -D warnings" RUST_BACKTRACE=1 cargo $(CARGO_ARGS) +$(CORE_RUST_VERSION)

# list all the "C" binding tests that have been written
C_BINDING_DIRS = $(sort $(dir $(wildcard c_binding_tests/*/)))

# list all the "C" binding test executables that should be produced
C_BINDING_TESTS = $(foreach dir,$(C_BINDING_DIRS),target/debug/c_binding_tests/$(shell basename $(dir))/test_executable)

# list all the extraneous files that will be generated when running tests
C_BINDING_CLEAN = $(foreach dir,$(C_BINDING_DIRS),$(dir)Makefile $(dir).qmake.stash)

# Check if Rust version is correct, and prompts to offer to change to the correct version.  Requires
# RUST_VERSION to be set, as appropriate for whatever target is being installed (defaults to
# CORE_RUST_VERSION; see install_rustup..., below).  We'll also export PATH to default location of
# Rust installation for use here in the Makefile, in case this is the first time rustup has been
# installed/run, and we don't have a rustup-modified .profile loaded yet.  If not connected to a
# terminal (stdin is a tty), or running under a Continuous Integration test (CI), defaults to
# automatically installing and changing the default Rust version (under the assumption that the
# invoker of the Makefile target knows what they want, under headless automated procedures like
# CI). Otherwise, entering "no<return>" rejects installing/changing the Rust version (and we assume
# you know what you're doing, eg. testing some new Rust toolchain version that you've installed)
export PATH := $(HOME)/.cargo/bin:$(PATH)
RUST_VERSION = $(CORE_RUST_VERSION)
.PHONY: version_rustup

version_rustup:
	@if which rustup >/dev/null; then \
	    echo -e "\033[0;93m## Current Rust version installed (need: '$(RUST_VERSION)'): ##\033[0m"; \
	    if ! rustup override list 2>/dev/null | grep "^$(PWD)\s*$(RUST_VERSION)"; then \
		rustup show; rustup override list; \
		echo -e "\033[0;93m## Change $(PWD) Rust version override to '$(RUST_VERSION)' ##\033[0m"; \
		[ -t 1 ] && [ -t 0 ] && [[ "$(CI)" == "" ]] && read -p "Continue? (Y/n) " yes; \
		if [[ "$${yes:0:1}" != "n" ]] && [[ "$${yes:0:1}" != "N" ]]; then \
		    echo -e "\033[0;93m## Selecting Rust version '$(RUST_VERSION)'... ##\033[0m"; \
		    rustup override set $(RUST_VERSION); \
		fi; \
	    fi; \
	fi


# Actual installation of Rust $(RUST_VERSION) via curl
.PHONY: curl_rustup
curl_rustup:
	@if ! which rustup >/dev/null; then \
	    echo -e "\033[0;93m## Installing Rust $(RUST_VERSION)... ##\033[0m"; \
	    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain $(RUST_VERSION) -y; \
	fi

# idempotent install rustup with the default toolchain set for Holochain core
# best for green fields Rust installation
.PHONY: install_rustup
install_rustup:		RUST_VERSION = $(CORE_RUST_VERSION)
install_rustup: version_rustup curl_rustup

# idempotent install rustup with the default toolchain set for tooling
# best for CI based on tools only.
.PHONY: install_rustup_tools
install_rustup_tools:	RUST_VERSION = $(TOOLS_RUST_VERSION)
install_rustup_tools: version_rustup curl_rustup

# idempotent installation of core toolchain.  Changes default toolchain to CORE_RUST_VERSION.
.PHONY: core_toolchain
core_toolchain: RUST_VERSION=$(CORE_RUST_VERSION)
core_toolchain: version_rustup install_rustup

# idempotent installation of tools toolchain.  Changes default toolchain to TOOLS_RUST_VERSION.
.PHONY: tools_toolchain
tools_toolchain: RUST_VERSION=$(TOOLS_RUST_VERSION)
tools_toolchain: version_rustup install_rustup_tools

# idempotent addition of wasm target in current (default: CORE_RUST_VERSION) toolchain
.PHONY: ensure_wasm_target
ensure_wasm_target: core_toolchain
	rustup target add wasm32-unknown-unknown

# idempotent installation of development tooling; RUST_VERSION defaults to TOOLS_RUST_VERSION
# Since the default toolchain has been changed (see: tools_toolchain, version_rustup), we can
# install the component without specifying which toolchain version to use)
.PHONY: install_rust_tools
install_rust_tools: tools_toolchain
	@if ! rustup component list | grep -q 'rustfmt-preview.*(installed)'; then \
		echo -e "\033[0;93m## Installing rustfmt (rust formatting) tools ##\033[0m"; \
		rustup component add rustfmt-preview; \
	fi
	@if ! rustup component list | grep -q 'clippy-preview.*(installed)'; then \
		echo -e "\033[0;93m## Installing clippy (rust linting) tools ##\033[0m"; \
		rustup component add clippy-preview; \
	fi
