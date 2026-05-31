// compile-flags: --test
// dylint_testing 用の UI テスト入力。
// 各 #[test] 関数名に対して 3 つの lint がどう発火するかを検証する。
// is_test_function は `--test` でコンパイルされた #[test] 関数のみ認識するため、
// 上の compile-flags ヘッダで --test を付与している。
#![allow(dead_code, unused)]

// test_ プレフィックス + 2 単語 + 振る舞い語なし → 3 lint すべて発火。
#[test]
fn test_quit() {}

// 2 単語 + 振る舞い語なし → test_name_min_words / require_behavior_keyword が発火。
#[test]
fn quit_app() {}

// すべて満たす良い例 → どの lint も発火しない。
#[test]
fn should_quit_on_q_key() {}

fn main() {}
