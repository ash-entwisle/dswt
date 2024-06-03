# Yes, IK you can just use cargo, but I like make for the flexibility

run:
	cargo run

bench:
	cargo build --release
	time cargo run --release

test:
	cargo test

build:
	cargo build --release