#!/usr/bin/env python3

from pathlib import Path
import subprocess
import shutil
from directories import (
    repository_directory,
    rust_build_directory,
    plugin_directory,
)

libname = "libuniffi_jormungandr_wallet.a"

# Due to how packaging of Apple libraries work we need to:
#
# - Build all libraries.
# - Combine iOS simulator libraries for all architectures (e.g. arm64, x86_64) of the same platform
#   variant (e.g. macOS, iPhone, iOS simulator) into a fat binary using lipo.
# - Combine all fat binaries of all platform variants into a single xcframework.
#
# To simplify the process, all steps are explicitly specified in the targets description.
targets = {
    "ios-sim": ["aarch64-apple-ios-sim", "x86_64-apple-ios"],
    "iphone": ["aarch64-apple-ios"],
}


def run():
    plugin_ios_dir = plugin_directory / "src/ios"
    xcframework_path = (
        plugin_directory / "src/ios/libuniffi_jormungandr_wallet.xcframework"
    )
    xcframework_command = [
        "xcodebuild",
        "-create-xcframework",
        "-output",
        xcframework_path,
    ]
    for platform, rust_targets in targets.items():
        rust_targets = ",".join(rust_targets)
        subprocess.run(
            [
                "cargo",
                "lipo",
                "-p",
                "wallet-uniffi",
                "--features",
                "builtin-bindgen",
                "--release",
                "--targets",
                rust_targets,
            ],
            check=True,
        )
        libname_platform = f"{platform}-{libname}"
        library_src_path = rust_build_directory / "universal/release" / libname
        library_path = plugin_ios_dir / libname_platform
        shutil.copy(library_src_path, library_path)
        xcframework_command += ["-library", library_path]
    subprocess.run(xcframework_command, check=True)

    # The current version of Cordova cannot deal with Swift packages, so instead we install the
    # required files as a regular Swift source file and a bridging header.
    wallet_swift_dir = repository_directory / "bindings/wallet-swift/Sources"
    shutil.copy(
        wallet_swift_dir / "JormungandrWallet/JormungandrWallet.swift", plugin_ios_dir
    )
    shutil.copy(
        wallet_swift_dir / "JormungandrWalletFFI/JormungandrWalletFFI.h",
        plugin_ios_dir,
    )


if __name__ == "__main__":
    run()
