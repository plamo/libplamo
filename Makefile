all:
	cargo build --release

install:
	install include/plamo.h /usr/include
	install target/release/libplamo.so /usr/lib
