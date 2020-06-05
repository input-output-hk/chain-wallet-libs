## Current platform (debug mode)

Build `wallet-c`:

    cargo build -p jormungandrwallet

Build the Swift library:

    swift build -v -Xcc -I./../bindings/wallet-c/ -Xlinker -lpthread -Xlinker -l./../../target/debug/libjormungandrwallet.a
