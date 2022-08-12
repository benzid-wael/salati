lint:
	cargo clippy

test:
	cargo test

doc:
	cargo rustdoc
	open target/doc/salati/all.html

build-web:
	wasm-pack build --target web

build-crate:
	cargo build

build: lint test build-crate build-web
