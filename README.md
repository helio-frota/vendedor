# vendedor

basic test with cargo vendor.

<https://doc.rust-lang.org/cargo/commands/cargo-vendor.html>

## Steps

* Basic code using clap as dependency declared on Cargo.toml

Clone and run:

```shell
➜  vendedor git:(main) ✗ cargo run potato
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/vendedor potato`
otatop
```

### Install and run as described on <https://crates.io/crates/cargo-vendor>

```shell
cargo install cargo-vendor
```

> Vendor and redirect the necessary vendor configs to a config file.
>
> <https://doc.rust-lang.org/cargo/commands/cargo-vendor.html#examples>

```shell
mkdir .cargo
cargo vendor > .cargo/config.toml
```

### A way to validate

> <https://github.com/rust-lang/cargo/issues/3289#issuecomment-485122034>

```shell
cargo install cargo-cache
cargo cache --autoclean
cargo cache --autoclean
```

Yes we need to run `cargo cache --autoclean` 2x to have output like this:

```shell
cargo cache --autoclean
Clearing cache...

Cargo cache '/home/heliofrota/.cargo':

Total:                             1.37 GB
  43 installed binaries:         535.03 MB
  Registry:                      710.33 MB
    Registry index:               82.10 MB
    4185 crate archives:         628.23 MB
    0 crate source checkouts:         0  B
  Git db:                        123.60 MB
    28 bare git repos:           123.60 MB
    0 git repo checkouts:             0  B
```

Build this project again

```shell
cargo clean ; cargo build
```

Run `cargo cache --autoclean` the output will be similar to previous...

Comment the content of `.cargo/config.toml`

```toml
# [source.crates-io]
# replace-with = "vendored-sources"

# [source.vendored-sources]
# directory = "vendor"
```

Build again

```shell
cargo clean ; cargo build
```

Now when `cargo cache --autoclean` shows a different output

```shell
cargo cache --autoclean
Clearing cache...

Cargo cache '/home/heliofrota/.cargo':

Total:                                      1.45 GB => 1.37 GB
  43 installed binaries:                             535.03 MB
  Registry:                             796.16 MB => 710.33 MB
    Registry index:                                   82.10 MB
    4185 crate archives:                             628.23 MB
    195 => 0 crate source checkouts:          85.83 MB => 0  B
  Git db:                                            123.60 MB
    28 bare git repos:                               123.60 MB
    0 git repo checkouts:                                 0  B

Size changed 1.45 GB => 1.37 GB (-85.83 MB, -5.89%)    <--------------------
```
