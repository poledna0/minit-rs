build:
	RUSTFLAGS="-C target-feature=+crt-static -C link-arg=-static" \
	cargo build --release --target x86_64-unknown-linux-musl

install:
	cp target/x86_64-unknown-linux-musl/release/minit-rs /init
	chmod +x /init

run:
	/init
