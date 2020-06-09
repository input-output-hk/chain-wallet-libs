#!/bin/sh

echo_internal() {
    builtin echo "$*"
    set -x
}

alias echo="{ set +x; } 2> /dev/null; echo_internal"

set -ex

echo "generate XCode project"
swift package generate-xcodeproj

mkdir -p Lib

echo "iOS native build"
cargo rustc -p jormungandrwallet --target aarch64-apple-ios --release -- -C lto
cp ./../../target/aarch64-apple-ios/release/libjormungandrwallet.a ./Lib/libjormungandrwallet.a
xcodebuild -project ./JormungandrWallet.xcodeproj -xcconfig ./Config/Release.xcconfig -sdk iphoneos

echo "iOS simulator build"
cargo rustc -p jormungandrwallet --target x86_64-apple-ios --release -- -C lto
cp ./../../target/x86_64-apple-ios/release/libjormungandrwallet.a ./Lib/libjormungandrwallet.a
xcodebuild -project ./JormungandrWallet.xcodeproj -xcconfig ./Config/Release.xcconfig -sdk iphonesimulator

echo "compose universal framework"
cp -r ./build/Release-iphoneos/JormungandrWallet.framework/ ./build/JormungandrWallet.framework
cp ./build/Release-iphonesimulator/JormungandrWallet.framework/Modules/JormungandrWallet.swiftmodule/x86_64.swiftmodule ./build/JormungandrWallet.framework/Modules/
lipo -arch arm64 ./build/Release-iphoneos/JormungandrWallet.framework/JormungandrWallet -arch x86_64 ./build/Release-iphonesimulator/JormungandrWallet.framework/JormungandrWallet -output ./build/JormungandrWallet.framework/JormungandrWallet -create

echo "the framework is located in ./build/JormungandrWallet.framework"

