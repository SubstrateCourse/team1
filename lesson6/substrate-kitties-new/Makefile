run:
	SKIP_WASM_BUILD= cargo run -- --dev -lruntime=debug

build:
	cargo build

check:
	SKIP_WASM_BUILD= cargo check

check-tests:
	SKIP_WASM_BUILD= cargo check --tests --all

test:
	cargo test --all

check-dummy:
	BUILD_DUMMY_WASM_BINARY= cargo check

build-native:
	SKIP_WASM_BUILD= cargo build

purge:
	SKIP_WASM_BUILD= cargo run -- purge-chain --dev -y

restart: purge run

