# Cbc-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

`Cbc-src` crate is a *-src crate. This links Cbc libraries to executable build by cargo, but does not provide Rust bindings. [Cbc] is built with [CoinUtils] ([CoinUtils-src]), [Osi] ([Osi-src]), [Cgl] ([Cgl-src]) and [Clp] ([Clp-src]).

By this package, you don't need to worry about installing CBC in the system, and it can be built for **all platforms**.

Cbc (Coin-or branch and cut) is an open-source mixed integer linear programming solver written in C++. It can be used as a callable library or using a stand-alone executable. It can be used in a wide variety of ways through various modeling systems, packages, etc.

## Usage

1. Add the following to your `Cargo.toml`:

    ```toml
    [dependencies]
    cbc-src = "\*"
    ```

2. Add the following to your `lib.rs`:

    ```toml
    extern crate cbc_src;
    ```

This package does not provide bindings. Please use [coincbc-sys], [coinclp-sys] to use Cbc, Clp, e.g.

```toml
[dependencies]
coincbc-sys = { version = "\*" }
```

## Configuration

### Features

The following Cargo features are supported:

* `default` to `osicbc` and `cbcsolver` feature;
* `osicbc` to build with `Osi` supported. However the `Osi` is always built, by which the `Cbc` call the `Clp` solver and `Cgl` library;
* `cbcsolver` to build `CbcSolver` lib and crate the api for `Rust`. If you do not use `Cbc` directly, you can disable this feature to reduce the build time;

### Environment

The package build from the source and link statically by default. It also provide the following environment variables to allow users to link to system library customly:

* `CARGO_COINUTILS_STATIC` to link to CoinUtils statically;
* `CARGO_COINUTILS_SYSTEM` to link to CoinUtils system library;
* `CARGO_OSI_STATIC` to link to Osi statically;
* `CARGO_OSI_SYSTEM` to link to Osi system library;
* `CARGO_CLP_STATIC` to link to Clp statically;
* `CARGO_CLP_SYSTEM` to link to Clp system library;
* `CARGO_CGL_STATIC` to link to Cgl statically;
* `CARGO_CGL_SYSTEM` to link to Cgl system library;
* `CARGO_CBC_STATIC` to link to Cbc statically;
* `CARGO_CBC_SYSTEM` to link to Cbc system library;

Set the environment variable to `1` to enable the feature. For example, to link to system library dynamically, set `CARGO_${LIB_NAME}_SYSTEM` to `1`; to link to system library statically, set both `CARGO_${LIB_NAME}_SYSTEM` and `CARGO_${LIB_NAME}_STATIC` to `1`.

## Windows and vcpkg

On Windows, if `${LIB_NAME}_SYSTEM` is set to `1`, `cbc-src` will use
[vcpkg] to find Cbc. Before building, you must have the correct Cbc
installed for your target triplet and kind of linking. For instance,
to link dynamically for the `x86_64-pc-windows-msvc` toolchain, install
 `cbc` for the `x64-windows` triplet:

```sh
vcpkg install cbc --triplet x64-windows
```

To link Cbc statically, install `cbc` for the `x64-windows-static-md` triplet:

```sh
vcpkg install cbc --triplet x64-windows-static-md
```

To link Cbc and C Runtime (CRT) statically, install `cbc` for the `x64-windows-static` triplet:

```sh
vcpkg install cbc --triplet x64-windows-static
```

and build with `+crt-static` option

```sh
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-pc-windows-msvc
```

Please see the ["Static and dynamic C runtimes" in The Rust reference](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes) for detail.

## Cross Compilation

you can compile it for the other target by providing the `--target` option to
`cargo build`.

| Target                               |  supported  |
|--------------------------------------|:-----------:|
| `arm-unknown-linux-gnueabi`          | ✓   |
| `arm-unknown-linux-gnueabihf`        | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `aarch64-unknown-linux-gnu`          | ✓   |
| `aarch64-unknown-linux-musl`         | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-pc-windows-msvc`             | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |
| `x86_64-unknown-linux-musl`          | ✓   |
| others                               | not test   |

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[CoinUtils]: https://github.com/coin-or/CoinUtils
[Osi]: https://github.com/coin-or/Osi
[Cgl]: https://github.com/coin-or/Cgl
[Clp]: https://github.com/coin-or/Clp
[Cbc]: https://github.com/coin-or/Cbc

[CoinUtils-src]: https://github.com/Maroon502/coinutils-src
[Cgl-src]: https://github.com/Maroon502/cgl-src
[Clp-src]: https://github.com/Maroon502/clp-src
[Osi-src]: https://github.com/Maroon502/osi-src
[coincbc-sys]: https://github.com/Maroon502/coincbc-sys
[coinclp-sys]: https://github.com/Maroon502/coinclp-sys

[vcpkg]: https://github.com/Microsoft/vcpkg

[documentation-img]: https://docs.rs/cbc-src/badge.svg
[documentation-url]: https://docs.rs/cbc-src
[package-img]: https://img.shields.io/crates/v/cbc-src.svg
[package-url]: https://crates.io/crates/cbc-src
[license-img]: https://img.shields.io/crates/l/cbc-src.svg
[license-url]: https://github.com/Maroon502/cbc-src/blob/master/LICENSE.md
