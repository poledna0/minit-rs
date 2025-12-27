
install:
	cp target/x86_64-unknown-linux-musl/release/minit-rs /init
	chmod +x /init

run:
	/init

build:
	RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-musl

