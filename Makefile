build: clean codecheck test
	cargo doc
	cargo build
	RUSTFLAGS="-C target-cpu=native" cargo build --release

clean:
	cargo clean

codecheck:
	cargo clippy --all-features --tests

test:
	cargo test --release

fmt:
	cargo fmt

fix:
	cargo fix --allow-dirty --allow-staged

perft:
	RUSTFLAGS="-C target-cpu=native" cargo build --release && time target/release/chessgen-perft 8
