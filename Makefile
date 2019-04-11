all:
	cargo build --release

install:
	install -m 644 include/plamo.h /usr/include
	install target/release/libplamo.so /usr/lib

clean:
	cargo clean
