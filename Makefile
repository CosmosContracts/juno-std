build-proto:
	@RUST_LOG=trace,info,debug,warn,error cargo run --bin proto-build
