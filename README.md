# error-codegen-poc

Example usage:


``` shell
cargo run -- --definitions example.json --backend doc-html --output=example-output/doc
cargo run -- --definitions example.json --backend rust --verbose --output=example-output/zksync-error
cargo run -- --definitions https://raw.githubusercontent.com/sayon/error-codegen-poc/refs/heads/main/example.json --backend rust --verbose --output=example-output/zksync-error
```
