# changelog-rs

Trivial Rust-based CHANGELOG.md generation tool

## How to run

```shell
curl https://sh.rustup.rs -sSf | sh # This downloads the rust compiler, latest stable version. You can
                                    # omit this step if you already have a suitable verison installed.
cargo run <repo> <from-revision> <to-revision>
```

Future versions are likely to support a mode in which a full log betwen all tagged versions are included. Stay tuned.
