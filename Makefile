build-proto:
	RUST_LOG=info,trace,warn,error cargo run --bin proto-build

build-proto-debug:
	RUST_LOG=trace,info,debug,warn,error cargo run --bin proto-build