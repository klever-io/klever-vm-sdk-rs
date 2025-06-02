apply-lint:
	cargo fmt --all
	cargo clippy --fix --allow-dirty

lint-check:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings