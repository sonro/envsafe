#!/usr/bin/env sh
set -e

MSRV="1.80.0"

echo "fmt check"
cargo fmt --all --check

echo "clippy"
cargo clippy -q --all-features --all-targets --workspace -- -D warnings

echo "build docs"
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc -q --no-deps --workspace --all-features --document-private-items

echo "build tests"
cargo test -q --no-run

echo "running tests..."
cargo test -q > /dev/null

echo "build tests on msrv"
RUSTUP_TOOLCHAIN="$MSRV" cargo test -q --all-features --no-run 

echo "running tests on msrv..."
RUSTUP_TOOLCHAIN="$MSRV" cargo test -q --all-features > /dev/null

echo ">>>>>> ALL OK <<<<<<<<"
