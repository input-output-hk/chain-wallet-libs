# Getting started 

The javascript documentation for this module can be generated with jsdoc by
running:

```bash
npm install
npm run doc
```

The generated files can be found at the `doc` directory, and can be read by
opening `index.html` with a web browser.

At the moment the best source for examples are the javascript
[tests](tests/src/main.js).

# Development

## Getting started

For general documentation on Cordova's plugins development, the [official
documentation](https://cordova.apache.org/docs/en/11.x/guide/hybrid/plugins/index.html)
is the best place to start.

### Requirements

As a baseline, Node.js and the cordova cli are required. Since the process of
running the tests involves creating and application, the documentation at
[installing-the-cordova-cli](https://cordova.apache.org/docs/en/11.x/guide/cli/index.html#installing-the-cordova-cli)
can be consulted. Check out the [Android
documentation](https://cordova.apache.org/docs/en/11.x/guide/platforms/android/index.html)
and the [IOs
documentation](https://cordova.apache.org/docs/en/11.x/guide/platforms/ios/plugin.html)
for requirements specific to the platform you are going to be developing for.

#### Android

[https://github.com/cross-rs/cross](cross) is currently used for building the
native libraries for Android.

#### JCLI

Additionally, jcli is required to generate the genesis file that it is used in
the test-vectors, installation instructions can be found in the [jormungandr's
repository](https://github.com/input-output-hk/jormungandr). It's recommended
that the `jcli` version is built with the same version of `chain-libs` that is
used to build the plugin (which can be found in the Cargo.lock file), although
it's not strictly necessary as long as the genesis binary encoding is
compatible.

## Overview

The core code of the plugin is written in rust, and ffi is used to bridge that
to either Objective-C or Kotlin, depending on the platform.

The [wallet.js](www/wallet.js) file has the top level Javascript api for the
plugin users, which is mostly a one-to-one mapping to the API of the
wallet-core rust crate.

The iOS part of the plugin is backed by the [wallet-c](../wallet-c/wallet.h)
package, while the Android support is provided via the
[wallet-uniffi](../wallet-uniffi/src/lib.udl) package. Both are also thin
wrappers over **wallet-core**.

## Building and running the tests

The *tests* directory contains a Cordova plugin with js tests
[tests](tests/src/main.js), we use
[cordova-plugin-test-framework](https://github.com/apache/cordova-plugin-test-framework)
as a test harness. 

The [test.py](scripts/test.py) script can be used to build
the plugin and setup the test harness. For example, the following command will

- create a cordova application at the `~/cdvtest/hello` directory (the
  directory must not exist, the script will not overwrite it).
- install cordova-plugin-test-framework.
- build the native libraries for the android platform, and copy those to
  src/android/libs.
- build the wallet-uniffi kotlin bindings for the native library.
- install the plugin at this directory.
- install the plugin in the tests directory (after .
- run the application.

```bash
./test.py --platform android -d ~/cdvtest --cargo-build --run full
```

The `reload-plugin` and `reload-tests` commands can be used if only one of
those was modified, to avoid having to redo the whole process.
