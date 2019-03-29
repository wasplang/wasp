build:
	cargo build
	cd examples/helloworld && make
	cd examples/canvas && make
	cd examples/testing && make
	cd examples/dynamic_dispatch && make
	cd examples/simplest && make
	cd compiler && make
serve:
	http-server -p 8080
