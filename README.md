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

```shell
mkdir .cargo
cargo vendor > .cargo/config.toml
```

>![NOTE]