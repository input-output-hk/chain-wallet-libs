#!/usr/bin/env python3

from pathlib import Path
import subprocess
import sys
import shutil

libname = "libjormungandrwallet.a"
root_directory = Path("../../target/")

library_header_src = Path("../wallet-c/wallet.h")
library_header_dst = Path("src/ios/LibWallet.h")


__AARCH64_TARGET = {"aarch64-apple-ios": ("arm64", "stable")}
__AARCH64_TARGET_SIM = {"aarch64-apple-ios-sim": ("arm64", "nightly")}

def run(sim: bool):
    targets = {
        "x86_64-apple-ios": ("x86_64", "stable"),
        **(__AARCH64_TARGET_SIM if sim else __AARCH64_TARGET),
    }

    lipo_args = ["lipo", "-create", "-output", f"./src/ios/{libname}"]

    for rust_target, (apple_target, toolchain) in targets.items():
        extra_cargo_options = []
        if toolchain == "nightly":
            extra_cargo_options = [f"+{toolchain}", "-Z", "build-std"]
        out = subprocess.run(
            [
                "cargo",
                *extra_cargo_options,
                "rustc",
                "--release",
                "--target",
                rust_target,
                "-p" "jormungandrwallet",
                "--",
                "-C",
                "lto",
            ]
        )
        if out.returncode != 0:
            print("couldn't build for target: ", rust_target)
            sys.exit(1)
        lipo_args += [
            "-arch",
            apple_target,
            str(root_directory / rust_target / "release" / libname),
        ]

    out = subprocess.run(lipo_args)
    if out.returncode != 0:
        print("couldn't build universal lib")
        sys.exit(1)

    shutil.copy(library_header_src, library_header_dst)


if __name__ == "__main__":
    # Set flag for emulator compatible lib
    sim = "--sim" in sys.argv
    run(sim)
