// swift-tools-version:5.2

import PackageDescription

let package = Package(
    name: "JormungandrWallet",
    products: [
        .library(
            name: "JormungandrWallet",
            targets: ["JormungandrWallet"]
        )
    ],
    targets: [
        .systemLibrary(name: "JormungandrWalletC"),
        .target(
            name: "JormungandrWallet",
            dependencies: ["JormungandrWalletC"]
        ),
    ]
)
