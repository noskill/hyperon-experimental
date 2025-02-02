![CI](https://github.com/trueagi-io/hyperon-experimental/actions/workflows/ci.yml/badge.svg)

# Overview

This is reimplementation of the C++ Hyperon prototype from scratch in a Rust
programming language. This project replaces the [previous
prototype](https://github.com/trueagi-io/hyperon/tree/master).
See [Python examples](./python/tests) to become familiar with Hyperon features.

# Prerequisites

Install latest stable Rust, see [Rust installation
page](https://www.rust-lang.org/tools/install).

# Hyperon library

Build and test the library:
```
cd ./lib
cargo build
cargo test
```

To enable logging during tests execute:
```
RUST_LOG=hyperon=debug cargo test
```

# C and Python API

Prerequisites:
```
cargo install cbindgen
python -m pip install conan
python -m pip install -e ./python[dev]
```

Setup build:
```
mkdir -p build
cd build
cmake ..
```

If `Conan` claims it cannot find out the version of the C compiler you can
workaround it by adding `compiler=` and `compiler.version=` into
`.conan/profiles/default`.

Build and run tests:
```
make
make check
```

To run release build use following instead of `cmake ..`:
```
cmake -DCMAKE_BUILD_TYPE=Release ..
```

# Setup IDE

See [Rust Language Server](https://github.com/rust-lang/rls) page.

In order to use clangd server generate compile commands using cmake var:
```
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=Y ..
```
