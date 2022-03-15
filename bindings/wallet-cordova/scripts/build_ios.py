#!/usr/bin/env python3

from pathlib import Path
import subprocess
import shutil
import os
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


def run(release=True):
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

    native_libs = []

    for platform, rust_targets in targets.items():
        libname_platform = f"{platform}-{libname}"
        library_path = plugin_ios_dir / libname_platform

        native_libs.append(library_path)
        xcframework_command += ["-library", library_path]

        lipo_command = ["lipo", "-create", "-output", library_path]

        for rust_target in rust_targets:
            rustc_command = [
                "cargo",
                "rustc",
                "-p",
                "wallet-uniffi",
                "--features",
                "builtin-bindgen",
                "--target",
                rust_target,
            ]

            if release:
                rustc_command += ["--release", "--", "-C", "lto"]

            subprocess.run(rustc_command, check=True)

            debug_or_release = "release" if release else "debug"
            library_src_path = (
                rust_build_directory / rust_target / debug_or_release / libname
            )
            lipo_command.append(library_src_path)

        subprocess.run(lipo_command, check=True)

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

    # remove intermediary build artifacts
    for lib in native_libs:
        os.remove(lib)


if __name__ == "__main__":
    run()
