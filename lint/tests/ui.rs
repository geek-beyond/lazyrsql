// dylint_testing による UI テスト。
// ui/ ディレクトリ内の入力をコンパイルし、各 lint が期待どおり発火するかを
// 隣接する .stderr ファイルと突き合わせて検証する。
#[test]
fn ui() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
    );
}
