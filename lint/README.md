# testname_lints

テスト関数名をわかりやすくするための [dylint](https://github.com/trailofbits/dylint) カスタム lint 群。
`#[test]` が付いた関数の名前だけを対象に検査する。

## lint 一覧

| lint | 既定レベル | 内容 |
| --- | --- | --- |
| `no_test_prefix` | warn | テスト名が `test_` プレフィックスで始まらないことを要求する(`#[test]` で既にテストと分かるため冗長)。 |
| `test_name_min_words` | warn | テスト名がアンダースコア区切りで 3 単語以上であることを要求する(対象・条件・期待を表現するため)。 |
| `require_behavior_keyword` | warn | テスト名に振る舞いを表す語(`should` / `when` / `returns` / `given` / `is` / `has` など)が含まれることを要求する。 |

例:

```rust
#[test]
fn test_quit() {}            // 3 つすべて発火
#[test]
fn quit_app() {}             // min_words / behavior が発火
#[test]
fn should_quit_on_q_key() {} // どれも発火しない(良い例)
```

## 個別の切り替え・レベル変更

各 lint は独立して有効/無効・レベル(`warn` / `deny`(error)/ `allow`)を切り替えられる。
対象クレート側の属性、または `cargo dylint` 実行時の `RUSTFLAGS` で指定する。

```rust
// クレートルート(lib.rs / main.rs)で個別に変更する例
#![deny(test_name_min_words)]        // この lint だけ error に昇格
#![allow(require_behavior_keyword)]  // この lint だけ無効化
```

```sh
# 実行時にまとめて指定する例
RUSTFLAGS="-D test_name_min_words -A require_behavior_keyword" \
  cargo dylint --lib-path lint/target/release --all -- --tests
```

## ビルドと検証

ツール(`dylint-link`)とタスクは `lint/` 内の `mise.toml` で完結している。

```sh
cd lint
mise run build   # cdylib をビルド
mise run test    # UI テストで各 lint が実際に発火することを検証
```

検証は `dylint_testing` による UI テスト(`tests/ui.rs` + `ui/main.rs` / `ui/main.stderr`)で行う。
これらの lint は `--test` でコンパイルされた `#[test]` 関数のみを認識するため、入力には
`// compile-flags: --test` ヘッダを付けている。

## 既知の制約

- 実プロジェクトへ適用する `cargo dylint` CLI(`cargo-dylint`)は `openssl-sys` のビルドを伴う。
  本環境は x86_64 エミュレーション下で利用可能な openssl が ARM のみのためインストールできない。
  検知の検証は上記 `mise run test`(`cargo-dylint` 非依存)で完結する。
- `clippy_utils` と nightly ツールチェーン(`rust-toolchain.toml`)はバージョンが密結合のため、
  どちらかを更新する際は対応する rev / nightly を合わせること。
