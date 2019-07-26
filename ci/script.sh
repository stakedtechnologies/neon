#!/usr/bin/env bash

set -eux

# Enable warnings about unused extern crates
export RUSTFLAGS=" -W unused-extern-crates"

# Install rustup and the specified rust toolchain.
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=$RUST_TOOLCHAIN -y
# Load cargo environment. Specifically, put cargo into PATH.
source ~/.cargo/env

rustup install nightly-2019-05-21
rustup target add wasm32-unknown-unknown --toolchain nightly-2019-05-21

rustc --version
rustup --version
cargo --version

case $TARGET in
	"native")
		sudo apt-get -y update
		sudo apt-get install -y cmake pkg-config libssl-dev

		# Install prerequisites and build all wasm projects
		./scripts/init.sh

		cargo test --all --release --locked
		;;

	"wasm")

		# Install prerequisites and build all wasm projects
		sudo apt-get install -y curl jq tar
		curl https://raw.githubusercontent.com/substrate-developer-hub/substrate-contracts-workshop/master/scripts/install-wasm-tools.sh -sSf |bash -s
		cargo install pwasm-utils-cli --bin wasm-prune --force

		cd ./contracts/cash && ./build.sh && make test
		cd ../commitment && make test
		cd ../deposit && make test
		cd ../predicate && make test
		;;
esac
