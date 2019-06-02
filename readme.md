# tensorflow-lite-sys

## what
This is meant to be tensorflow-microlite-sys someday, but today is not that day. We need the tensorflow sources, and I either have to:
* check them in here (no)
* use git submodules (Ive never seen it work with cargo)
* git clone/wget them in the build.rs (can't, build dependencies mix std and no_std )
* [cargo-5730](https://github.com/auxoncorp/cargo-5730) workaround isnt ideal as the build-dependencies arent cached  seem to cache and these are rather large 
So instead for now I have a manual `cargo gen` (which requires make 3.82 or later) to generate the tensorflow lite bindings for bluepill only.

## why?
Tensorflow lite for microcontrollers is actually kind of seperate experimental project inside tensorflow using -DTF_LITE_STATIC_MEMORY flag

[tensorflow-rust](https://github.com/tensorflow/rust) is the 'regular' tensorflow std bindings.

[tflite](https://github.com/boncheolgu/tflite-rs/blob/master/build.rs) is building the micro lite bindings but only fr std targets

## how
Rust.. is.. complicated. You need to manually:
* `cargo install bindgen`
* `wget -c https://codeload.github.com/tensorflow/tensorflow/tar.gz/v1.14.0-rc0 -O - | tar -xz`
* `TENSORFLOW_PATH=tensorflow-1.14.0-rc0 cargo run --target thumbv7m-none-eabi --example micro_speech --release`
