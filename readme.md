# tensorflow-lite-sys

## what
This is meant to be tensorflow-microlite-sys someday, currently its hardcoded for the bluepill example


## why?
Tensorflow lite for microcontrollers is actually kind of seperate experimental project inside tensorflow using -DTF_LITE_STATIC_MEMORY flag

[tensorflow-rust](https://github.com/tensorflow/rust) is the 'regular' tensorflow bindings only.

[tflite](https://github.com/boncheolgu/tflite-rs/blob/master/build.rs) is building the micro lite bindings but only fr std targets

## how
Rust.. is.. complicated. You need to manually:
* `cargo install bindgen`
* `wget -c https://codeload.github.com/tensorflow/tensorflow/tar.gz/v1.14.0-rc0 -O - | tar -xz`
* `TENSORFLOW_PATH=tensorflow-1.14.0-rc0 cargo run --example bluepill_micro_speech --release`
