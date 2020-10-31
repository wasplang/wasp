build:
	cd wasp && cargo build
	cd examples/helloworld && make
	cd examples/canvas && make
	cd examples/testing && make
	cd examples/dynamic_dispatch && make
	cd examples/simplest && make
	cd compiler && make
serve:
	http-server -p 8080
publish-std:
	cd compiler/vendor/std && git add . && git commit -m 'publishing' && git push
publish:
	git add . && git commit -m 'publishing' && git push
	cargo publish
