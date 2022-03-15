// swift-tools-version:5.5.0
import PackageDescription

let package = Package(
    name: "JormungandrWallet",
    products: [
        .library(name: "JormungandrWallet", targets: ["JormungandrWallet"]),
    ],
    targets: [
        .target(
            name: "JormungandrWallet",
            dependencies: ["JormungandrWalletFFI"]
        ),
        .systemLibrary(name: "JormungandrWalletFFI")
    ]
)
