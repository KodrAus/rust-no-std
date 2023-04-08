# Building your no-std Rust library for a no-std target in CI [![rust](https://github.com/KodrAus/rust-no-std/actions/workflows/rust.yml/badge.svg)](https://github.com/KodrAus/rust-no-std/actions/workflows/rust.yml)

This example demonstrates how to make sure your Rust library will actually build for a no-std target in CI.

-----

The current design of Rust's standard library is split into a few layers, each building on the assumed platform capabilities of the one below. There's:

- [`std`](https://doc.rust-lang.org/std): the full standard library assumes the presence of threads, a filesystem, and networking. Targets for common operating systems, like `x86_64-pc-windows-msvc`, are supported by `std`. Some targets, like `wasm32-unknown-unknown` are supported by `std`, but some features are shimmed.
- [`alloc`](https://doc.rust-lang.org/alloc): the collections layer builds on the core by assuming runtime support for dynamic memory allocation.
- [`core`](https://doc.rust-lang.org/core): the core layer makes no (well, not very many) assumptions about the underlying platform. Just about any target that can run Rust code is supported by `core`.

So when you're designing your library you can make it maximally portable by targeting the lowest layer of the standard library that you can. That can become tricky if your library has dependencies though. You might want to target `core`, but a dependency might quietly pull in `std`. If you develop on a platform that supports `std` (which you probably are) then you might not even notice things don't work until a user reports a bug.

All is not lost though. We can check our libraries against a no-std target during CI so a regression in target support gets caught sooner rather than later. In this example, we do that by building for the `thumbv6m-none-eabi` target, which is only supported by `core`.
