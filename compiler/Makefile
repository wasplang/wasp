build:
	cd ../&& cargo build
	../target/debug/wasp build
watch:
	while true; do \
	    make build; \
	    inotifywait -qre close_write .; \
	done
serve:
	http-server -p 8080
