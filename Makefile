build-ServiceApiFunction:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap $(ARTIFACTS_DIR)

deploy:
	sam deploy --no-fail-on-empty-changeset