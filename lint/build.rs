//! dylint ライブラリ(cdylib)は `@rpath/librustc_driver-*.dylib` を参照するため、
//! dylint-driver からロード(dlopen)できるよう、ビルド時にアクティブな nightly
//! ツールチェーンの sysroot/lib への rpath を埋め込む。
//!
//! `rustc --print sysroot` を使って計算するため、特定の環境にハードコードせず
//! どのマシン / CI 上でも正しい rpath が付与される。
use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
        .expect("failed to run `rustc --print sysroot`");
    let sysroot = String::from_utf8(output.stdout).expect("sysroot path is not valid UTF-8");
    let lib_dir = format!("{}/lib", sysroot.trim());

    // cdylib のリンク時に rpath を追加する(macOS/Linux の両方で有効)。
    println!("cargo:rustc-link-arg=-Wl,-rpath,{lib_dir}");
    println!("cargo:rerun-if-changed=build.rs");
}
