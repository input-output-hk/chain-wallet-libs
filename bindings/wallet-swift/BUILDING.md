## Current platform (debug mode)

Build `wallet-c`:

    cargo build -p jormungandrwallet

Build the Swift library:

    swift build -v -Xcc -I./../bindings/wallet-c/ -Xlinker -lpthread -Xlinker -l./../../target/debug/libjormungandrwallet.a

## iOS

Make sure that you have installed:

* XCode.
* Rust with `x86_64-apple-ios` and `aarch64-apple-ios` targets.

To build run `./build-framework.sh`

