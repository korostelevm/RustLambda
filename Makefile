build-ServiceApiFunction:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap $(ARTIFACTS_DIR)

deploy:
	sam package --template-file template.yaml --s3-bucket cold-lambda --s3-prefix RustLambda --output-template-file template-built.yaml
	sam deploy --template-file template-built.yaml --stack-name RustLambda-api --capabilities CAPABILITY_NAMED_IAM --no-fail-on-empty-changeset