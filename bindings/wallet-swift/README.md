This library is a Swift package that can be either build into an Apple
framework, imported into XCode directly or be used in a non-XCode environment as
a dependency of another Swift package.

## Usage

### As a Swift package

When using this library as an ordinary Swift package you need to link against
the binary (either static or dynamic) produced by

    cargo rustc -p wallet-uniffi --features builtin-bindgen

## Generating the code of this library

The `./Sources` contents are generated from `./../wallet-uniffi` by
`gen_bingings.sh` script you will find in that directory. You need to run it
every time UDL definitions are changed or when uniffi version is updated.
