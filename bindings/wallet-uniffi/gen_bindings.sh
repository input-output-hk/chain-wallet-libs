#!/usr/bin/env sh

uniffi-bindgen generate -l kotlin ./src/lib.udl --config-path ./uniffi.toml -o ./codegen/kotlin

uniffi-bindgen generate -l swift ./src/lib.udl --config-path ./uniffi.toml -o ./codegen/swift
mkdir -p ./../wallet-swift/Sources/{JormungandrWallet,JormungandrWalletFFI}
mv -f ./codegen/swift/JormungandrWallet.swift ./../wallet-swift/Sources/JormungandrWallet/JormungandrWallet.swift
mv -f ./codegen/swift/JormungandrWalletFFI.h ./../wallet-swift/Sources/JormungandrWalletFFI/JormungandrWalletFFI.h
mv -f ./codegen/swift/JormungandrWalletFFI.modulemap ./../wallet-swift/Sources/JormungandrWalletFFI/module.modulemap
