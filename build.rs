use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn generate_bindings(tflite_src_dir: PathBuf) {
    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut cmd = Command::new("bindgen");
    cmd.arg("wrapper.h");
    cmd.arg("--verbose");

    // --rust-target \
    // 1.33 \
    cmd.arg("--no-derive-default");
    cmd.arg("--ctypes-prefix=cty");
    // --generate \
    // functions,types,vars,methods,constructors,destructors \

    cmd.arg("--use-core");
    cmd.arg("--blacklist-type");
    cmd.arg("__va_list");
    cmd.arg("--output");
    cmd.arg(outdir.join("bindings.rs"));

    cmd.arg("--");
    cmd.arg("-x");
    cmd.arg("c++");

    cmd.arg("-target");
    cmd.arg(env::var("TARGET").unwrap());

    let sysroot_dir = tflite_src_dir.join(
        "/tensorflow/lite/experimental/micro/tools/make/downloads/gcc_embedded/arm-none-eabi",
    );

    cmd.arg(format!("-I{}", sysroot_dir.join("include").display()));
    cmd.arg(format!(
        "-I{}",
        sysroot_dir.join("include/c++/7.3.1").display()
    ));
    cmd.arg(format!(
        "-I{}",
        sysroot_dir.join("c++/7.3.1/arm-none-eabi").display()
    ));

    cmd.arg(format!("-I{}", tflite_src_dir.display()));

    // -Itools/tensorflow-1.14.0-rc0/tensorflow/lite/experimental/micro/tools/make/third_party/gemmlowp/fixedpoint \
    // -Itools/tensorflow-1.14.0-rc0/tensorflow/lite/experimental/micro/tools/make/third_party/flatbuffers/include \
    // -Itools/tensorflow-1.14.0-rc0/tensorflow/lite/experimental/micro/tools/make/third_party/stm32_bare_lib/include \
    // -Itools/tensorflow-1.14.0-rc0/tensorflow/lite/experimental/micro/tools/make/third_party/kissfft/kiss_fft \

    cmd.arg("-DGEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK");
    cmd.arg("-DTF_LITE_STATIC_MEMORY");
    cmd.arg("-DTF_LITE_MCU_DEBUG_LOG");
    cmd.arg("-DNDEBUG");
    cmd.arg("-g");
    cmd.arg("-O3");
    cmd.arg("-std=c++11");
    cmd.arg("-fno-rtti");
    cmd.arg("-fmessage-length=0");
    cmd.arg("-fno-exceptions");
    cmd.arg("-fno-unwind-tables");
    cmd.arg("-fno-builtin");
    cmd.arg("-ffunction-sections");
    cmd.arg("-fdata-sections");
    cmd.arg("-funsigned-char");
    cmd.arg("-MMD");
    cmd.arg("-mcpu=cortex-m3");
    cmd.arg("-mthumb");
    cmd.arg("-std=gnu++11");
    cmd.arg("-Wvla");
    cmd.arg("-Wall");
    cmd.arg("-Wextra");
    cmd.arg("-Wno-unused-parameter");
    cmd.arg("-Wno-missing-field-initializers");
    cmd.arg("-Wno-write-strings");
    cmd.arg("-Wno-sign-compare");
    cmd.arg("-fno-delete-null-pointer-checks");
    cmd.arg("-fomit-frame-pointer");
    cmd.arg("-fpermissive");
    cmd.arg("-nostdlib");
    cmd.arg("-g");
    cmd.arg("-Os");
    cmd.arg("-DARM_MATH_CM3");
    cmd.arg("-DARM_CMSIS_NN_M3");

    let cmsis_dir =
        tflite_src_dir.join("/tensorflow/lite/experimental/micro/tools/make/downloads/cmsis/CMSIS");

    cmd.arg(format!("-I{}", cmsis_dir.join("Core/Include").display()));
    cmd.arg(format!(
        "-I{}",
        cmsis_dir.join("CMSIS/NN/Include").display()
    ));
    cmd.arg(format!(
        "-I{}",
        cmsis_dir.join("CMSIS/DSP/Include").display()
    ));

    cmd.output().expect("failed to build tensorflow");
}

fn main() {
    let tensorflow_path = env::var("TENSORFLOW_PATH").expect(
        "You need to set the environment variable 'TENSORFLOW_PATH' to point to a checkout of https://github.com/tensorflow/tensorflow");

    let tflite_src_dir = PathBuf::from(tensorflow_path);

    Command::new("gmake")
        .arg("all")
        .arg("-f")
        .arg("tensorflow/lite/experimental/micro/tools/make/Makefile")
        .arg("TARGET=bluepill")
        .current_dir(&tflite_src_dir)
        .status()
        .expect("failed to build tensorflow");

    println!("cargo:rustc-link-lib=static=tensorflow-microlite");
    println!(
        "cargo:rustc-link-search={}",
        Path::new(&tflite_src_dir)
            .join("tensorflow/lite/experimental/micro/tools/make/gen/bluepill_cortex-m3/lib")
            .display()
    );

    generate_bindings(tflite_src_dir);
}
