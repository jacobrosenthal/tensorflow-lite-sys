# tensorflow-microlite-bluepill-sys

This is meant to be tensorflow-microlite-sys someday, but today is not that day. We need the tensorflow sources, and I either have to:
* check them in here (no)
* use git submodules (Ive never seen it work with cargo)
* git clone/wget them in the build.rs (can't, build dependencies mix std and no_std )

So instead, I have manually:

```
wget https://codeload.github.com/tensorflow/tensorflow/tar.gz/v1.14.0-rc0
tar -xvzf v1.14.0-rc0.tar.gz' 'cd tensorflow-1.14.0-rc0
make all -f tensorflow/lite/experimental/micro/tools/make/Makefile TARGET=bluepill
cp tensorflow/lite/experimental/micro/tools/make/gen/bluepill_cortex-m3/lib/libtensorflow-microlite.a ..
```
