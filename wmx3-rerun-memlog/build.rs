use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // WMX3 APIライブラリのパスを指定
    println!("cargo:rustc-link-search=C:\\Program Files\\SoftServo\\WMX3\\Lib\\");

    // 使用するWMX3 APIライブラリを指定
    println!("cargo:rustc-link-lib=WMX3Api");
    println!("cargo:rustc-link-lib=CoreMotionApi");
    println!("cargo:rustc-link-lib=LogApi");
    println!("cargo:rustc-link-lib=IMDll");

    // WMX3 APIを使う場合、VS2015以降のコンパイラでは以下のライブラリが必要
    println!("cargo:rustc-link-lib=legacy_stdio_definitions");
    println!("cargo:rustc-link-lib=legacy_stdio_wide_specifiers");

    // .h, .cppの変更を検出させる
    println!("cargo:rerun-if-changed=./src/ffi/wmx.cpp");
    println!("cargo:rerun-if-changed=./src/ffi/wmx.h");

    // C++コンパイル
    cc::Build::new()
        .file("./src/ffi/wmx.cpp")
        .include("./src/ffi/wmx.h")
        .compile("wmxffi");

    // bindgenでFFI自動化
    let bindings = bindgen::Builder::default()
        .header("./src/ffi/wmx.h")
        .clang_arg("-Iffi")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // $OUT_DIR/bindings.rs にFFIコードを書き込み
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // IMDll.dllを.exeと同じディレクトリにコピーする
    let src = "C:\\Program Files\\SoftServo\\WMX3\\Lib\\IMDll.dll";
    let dest = out_path.join("IMDll.dll");
    fs::copy(src, &dest).unwrap();
}
