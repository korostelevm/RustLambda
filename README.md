# RustLambda




https://www.rust-lang.org/tools/install


```
brew install rustup-init
rustup-init
```

follow
https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/building-custom-runtimes.html
https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/

must have  SAM CLI, version 0.52.0
```

$ cargo build --release --target x86_64-unknown-linux-musl --verbose    
$ sam build

$ mkdir .cargo
$ echo '[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"' > .cargo/config

```