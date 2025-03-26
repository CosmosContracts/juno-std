build-proto:
	RUST_LOG=info,trace,warn,error RUST_BACKTRACE=full cargo run --bin proto-build

build-proto-debug:
	RUST_LOG=trace,info,debug,warn,error RUST_BACKTRACE=full cargo run --bin proto-build