# error-codegen-poc

## As CLI
Example usage:


``` shell
cargo run -- --root-definitions example.json --backend doc-html --output=example-output/doc
cargo run -- --root-definitions example.json --backend rust --verbose --output=example-output/zksync-error
cargo run -- --root-definitions https://raw.githubusercontent.com/sayon/error-codegen-poc/refs/heads/main/example.json --backend rust --verbose --output=example-output/zksync-error
```


## As build dependency

Check the example: https://github.com/sayon/error-codegen-users

Packages may provide error descriptions for others to use, use the error hierarchy, or both.

### Publishing
To make your error description public for others:

1. Suppose you write a piece of error description database in `resources/errors.json`.
2. Modify your `Cargo.toml` as follows:


```toml
[package]
...

include=["resources/errors.json"]

[package.metadata.zksync_error_codegen]
json_files = ["resources/errors.json"]

```

3. Now other packages that depend on your package will be able to use errors that you have defined in `errors.json` in their Rust code!



### Using

To use others' error descriptions:

1. Your project has to be a workspace.
2. Create a dummy crate `zksync_error` in the root of the workspace, as shown here: https://github.com/sayon/error-codegen-users/tree/main/consumer_workspace/zksync_error
3. Make sure you depend on all crates that contain relevant parts of error hierarchy.
4. For the error users in your workspace, modify their `Cargo.toml`:
   - Add dependency from `zksync_error` 

    ```toml
    [dependencies]
    zksync_error =  { path = "../zksync_error" }
    ```
   - Add build dependency:

    ```toml
    [build-dependencies]
    zksync-error-codegen = { git = "https://github.com/sayon/error-codegen-poc", branch = "cargo-dep-control" }
    ```
   - Add `build.rs` with these contents:

   ``` rust
    fn main() {
        println!("cargo:rerun-if-changed=resources/errors.json");
        zksync_error_codegen::default_load_and_generate("provider_root")
    }
   ```
5. Execute `cargo build` in the workspace root. First invocation may fail -- this is alright. Use `cargo build -vv` to see the log.

Here, `"provider_root"` is the name of the crate hosting the root `errors.json` file: [example](https://github.com/sayon/error-codegen-users/tree/main/provider_root).

Notice that the [root json](https://github.com/sayon/error-codegen-users/blob/main/provider_root/resources/errors.json) file is quite verbose but it is linked to other json files holding smaller portions of error hierarchy description (components or domains).

Links can be absolute paths in the filesystem, URLs, but we recommend using [special "cargo" links](https://github.com/sayon/error-codegen-users/blob/a44478f3b146f5f9da6b4e1f117f30eafd3c78dd/provider_root/resources/errors.json#L162):

`cargo://<crate name>@@<filename with extension>`.

Links are resolved in a context containing fields `json_files` of `[package.metadata.zksync_error_codegen]` for all dependencies of the user crate.
