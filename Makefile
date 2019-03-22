build:
	cargo build
	cd examples/helloworld && make
	cd examples/canvas && make
	cd examples/testing && make
	cd examples/wasmer && make
	cd examples/dynamic_dispatch && make
	cd compiler && make
serve:
	http-server -p 8080
