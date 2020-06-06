## Current platform (debug mode)

Build `wallet-c`:

    cargo build -p jormungandrwallet

Build the Swift library:

    swift build -v -Xcc -I./../bindings/wallet-c/ -Xlinker -lpthread -Xlinker -l./../../target/debug/libjormungandrwallet.a

## iOS

Build `wallet-c`:

    cargo rustc -p jormungandrwallet --target x86_64-apple-ios --release -- -C lto
    cargo rustc -p jormungandrwallet --target aarch64-apple-ios --release -- -C lto

Link universal library:

    mkdir Lib
    lipo -create -output ./Lib/libjormungandrwallet.a -arch x86_64 ./../../target/x86_64-apple-ios/release/libjormungandrwallet.a -arch arm64 ./../../target/aarch64-apple-ios/release/libjormungandrwallet.a

Generate XCode project:

    swift package generate-xcodeproj

Build the framework:

    xcodebuild -project ./JormungandrWallet.xcodeproj -xcconfig ./Config/Release.xcconfig

