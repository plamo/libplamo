all:
	cargo build --release

install:
	install -m 644 include/libplamo.h /usr/include
	install target/release/libplamo.so /usr/lib

uninstall:
	-rm /usr/include/libplamo.h
	-rm /usr/lib/libplamo.so

clean:
	cargo clean
