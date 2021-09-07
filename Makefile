build: build-dynamic

.PHONY: build-static
build-static:
	cargo build --release
	mkdir -p lib bin
	cp target/release/libtheno.* lib/
	go build -o bin/gotheno gotheno.go 

# Wasn't able to get this to work on M1 mac
.PHONY: build-dynamic
build-dynamic:
	cargo build --release
	mkdir -p lib bin
	cp target/release/libtheno.* lib/
	go build -ldflags="-r ./lib" -o bin/godynotheno godynotheno.go 


.PHONY: clean
clean:
	rm -rf bin/gothermo bin/godynotheno lib/libtheno.a lib/libtheno.d lib/libtheno.dylib lib/libtheno.so