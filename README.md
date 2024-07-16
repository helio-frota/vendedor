# vendedor

basic test with cargo vendor.

<https://doc.rust-lang.org/cargo/commands/cargo-vendor.html>

## Steps

Clone

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
Clearing cache...

Cargo cache '/home/heliofrota/.cargo':

Total:                           392.56 MB
  17 installed binaries:         216.95 MB
  Registry:                      153.25 MB
    Registry index:               34.19 MB
    831 crate archives:          119.06 MB
    0 crate source checkouts:         0  B
  Git db:                         22.35 MB
    9 bare git repos:             22.35 MB
    0 git repo checkouts:             0  B
```

Build this project again

```shell
cargo clean ; cargo build
```

Run `cargo cache --autoclean` the output will be similar to previous, but we have modifications
because it checks out colored_json and zero:

```shell
➜  vendedor git:(main) ✗ cargo cache --autoclean
Clearing cache...

Cargo cache '/home/heliofrota/.cargo':

Total:                                394.56 MB => 392.56 MB
  17 installed binaries:                           216.95 MB
  Registry:                           154.86 MB => 153.25 MB
    Registry index:                                 34.19 MB
    831 crate archives:                            119.06 MB
    6 => 0 crate source checkouts:           1.61 MB => 0  B
  Git db:                               22.74 MB => 22.35 MB
    9 bare git repos:                               22.35 MB
    2 => 0 git repo checkouts:             389.45 KB => 0  B <-------------------

Size changed 394.56 MB => 392.56 MB (-2.00 MB, -0.5%)
```

```shell
➜  vendedor git:(main) ✗ ls ~/.cargo/git/checkouts/
colored_json-49209ff4acde481c/  zero-3d3eea17a85fd2bc/
```
