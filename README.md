# Cbc-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

The package provides Low-level bindings to the [Cbc] library. [Cbc] build with [CoinUtils] ([CoinUtils-src]), [Osi] ([Clp-src]), [Cgl] ([Cgl-src]) and [Clp] ([Clp-src]) support.
                                
Cbc (Coin-or branch and cut) is an open-source mixed integer linear programming solver written in C++. It can be used as a callable library or using a stand-alone executable. It can be used in a wide variety of ways through various modeling systems, packages, etc.

## Usage
Just add the following to your `Cargo.toml`:

```toml
[dependencies]
cbc-src = "0.1"
```

## Configuration

The following Cargo features are supported:

* `default` to `with_osi` feature;
* `with_clp` to build with Clp support;


The package build from the source and link statically by default. It also provide the following environment variables to allow users to link to system library customly:

* `CARGO_COINUTILS_STATIC` to link to CoinUtils statically;
* `CARGO_COINUTILS_SYSTEM` to link to CoinUtils system library;
* `CARGO_OSI_STATIC` to link to Osi statically;
* `CARGO_OSI_SYSTEM` to link to Osi system library;
* `CARGO_CLP_STATIC` to link to Clp statically if `with_clp` feature is enabled;
* `CARGO_CLP_SYSTEM` to link to Clp system library if `with_clp` feature is enabled;
* `CARGO_CGL_STATIC` to link to Cgl statically;
* `CARGO_CGL_SYSTEM` to link to Cgl system library;

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

```
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
| `armv7-linux-androideabi`            | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-pc-windows-gnu`              | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[CoinUtils]: https://github.com/coin-or/CoinUtils
[Osi]: https://github.com/coin-or/Osi
[Cgl]: https://github.com/coin-or/Cgl
[Clp]: https://github.com/coin-or/Clp
[Cbc]: https://github.com/coin-or/Cbc
[BCP]: https://github.com/coin-or/BCP

[CoinUtils-src]: https://github.com/Maroon502/coinutils-src
[Osi-src]: https://github.com/Maroon502/osi-src
[Cgl-src]: https://github.com/Maroon502/cgl-src
[Clp-src]: https://github.com/Maroon502/clp-src
[Cbc-src]: https://github.com/Maroon502/cbc-src
[BCP-src]: https://github.com/Maroon502/bcp-src

[vcpkg]: https://github.com/Microsoft/vcpkg

[documentation-img]: https://docs.rs/cbc-src/badge.svg
[documentation-url]: https://docs.rs/cbc-src
[package-img]: https://img.shields.io/crates/v/cbc-src.svg
[package-url]: https://crates.io/crates/cbc-src
[license-img]: https://img.shields.io/crates/l/cbc-src.svg
[license-url]: https://github.com/Maroon502/cbc-src/blob/master/LICENSE.md
