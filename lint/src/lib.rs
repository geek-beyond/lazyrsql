#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_hir_and_then;
use clippy_utils::is_test_function;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{Body, FnDecl, ItemKind, Node};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, impl_lint_pass};
use rustc_span::Span;

/// test_name_min_words が要求する最小単語数（アンダースコア区切り）。
const MIN_WORDS: usize = 3;

/// 振る舞いを表すと見なすキーワード一覧。
const BEHAVIOR_KEYWORDS: &[&str] = &[
    "should", "when", "returns", "return", "given", "is", "has", "have", "if", "with", "without",
    "on", "fails", "fail", "succeeds", "succeed", "handles", "handle", "does", "not",
];

declare_lint! {
    /// ### What it does
    /// `#[test]` 関数名に振る舞いを表す語（should/when/returns/given/is/has など）が
    /// 含まれているかを検査する。
    ///
    /// ### Why is this bad?
    /// テスト名が何を検証しているか読み取れず、可読性が下がるため。
    ///
    /// ### Example
    /// ```rust
    /// #[test]
    /// fn quit_q_key() {}
    /// ```
    /// Use instead:
    /// ```rust
    /// #[test]
    /// fn should_quit_on_q_key() {}
    /// ```
    pub REQUIRE_BEHAVIOR_KEYWORD,
    Warn,
    "test function name should contain a behavior keyword (should/when/returns/...)"
}

declare_lint! {
    /// ### What it does
    /// `#[test]` 関数名がアンダースコア区切りで 3 単語以上であることを要求する。
    ///
    /// ### Why is this bad?
    /// 単語数が少ないテスト名は対象・条件・期待を表現しきれないため。
    ///
    /// ### Example
    /// ```rust
    /// #[test]
    /// fn quit_works() {}
    /// ```
    /// Use instead:
    /// ```rust
    /// #[test]
    /// fn should_quit_on_q() {}
    /// ```
    pub TEST_NAME_MIN_WORDS,
    Warn,
    "test function name should have at least three underscore-separated words"
}

declare_lint! {
    /// ### What it does
    /// `#[test]` 関数名が `test_` プレフィックスで始まっていないことを要求する。
    ///
    /// ### Why is this bad?
    /// `#[test]` 属性で既にテストと分かるため `test_` は冗長で、振る舞い記述の妨げになる。
    ///
    /// ### Example
    /// ```rust
    /// #[test]
    /// fn test_should_quit() {}
    /// ```
    /// Use instead:
    /// ```rust
    /// #[test]
    /// fn should_quit() {}
    /// ```
    pub NO_TEST_PREFIX,
    Warn,
    "test function name should not start with the `test_` prefix"
}

pub struct TestNameLints;

impl_lint_pass!(TestNameLints => [
    REQUIRE_BEHAVIOR_KEYWORD,
    TEST_NAME_MIN_WORDS,
    NO_TEST_PREFIX,
]);

impl<'tcx> LateLintPass<'tcx> for TestNameLints {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _kind: FnKind<'tcx>,
        _decl: &'tcx FnDecl<'tcx>,
        _body: &'tcx Body<'tcx>,
        _span: Span,
        def_id: LocalDefId,
    ) {
        // #[test] 属性が付いた関数のみを対象にする。
        if !is_test_function(cx.tcx, def_id) {
            return;
        }

        // 関数名と診断用 span を HIR から取得する。
        let hir_id = cx.tcx.local_def_id_to_hir_id(def_id);
        let Node::Item(item) = cx.tcx.hir_node(hir_id) else {
            return;
        };
        let ItemKind::Fn { ident, .. } = item.kind else {
            return;
        };
        let name = ident.name.as_str();
        let name_span = ident.span;

        // 1. no_test_prefix
        if name.starts_with("test_") {
            span_lint_hir_and_then(
                cx,
                NO_TEST_PREFIX,
                hir_id,
                name_span,
                "test function name should not start with the `test_` prefix",
                |diag| {
                    // `test_` 除去後が空(名前が `test_` のみ)の場合は提案を出さない。
                    if let Some(stripped) = name.strip_prefix("test_").filter(|s| !s.is_empty()) {
                        diag.help(format!("consider renaming to `{stripped}`"));
                    }
                },
            );
        }

        // アンダースコア区切りの語に一度だけ分割し、以降の判定で共有する。
        let words: Vec<&str> = name.split('_').filter(|w| !w.is_empty()).collect();

        // 2. test_name_min_words
        let word_count = words.len();
        if word_count < MIN_WORDS {
            span_lint_hir_and_then(
                cx,
                TEST_NAME_MIN_WORDS,
                hir_id,
                name_span,
                format!(
                    "test function name has {word_count} word(s); at least {MIN_WORDS} are required"
                ),
                |diag| {
                    diag.help("use underscore-separated words like `should_<subject>_<behavior>`");
                },
            );
        }

        // 3. require_behavior_keyword
        let has_behavior = words.iter().any(|w| {
            BEHAVIOR_KEYWORDS
                .iter()
                .any(|kw| w.eq_ignore_ascii_case(kw))
        });
        if !has_behavior {
            span_lint_hir_and_then(
                cx,
                REQUIRE_BEHAVIOR_KEYWORD,
                hir_id,
                name_span,
                "test function name should contain a behavior keyword",
                |diag| {
                    diag.help(format!("add one of: {}", BEHAVIOR_KEYWORDS.join(", ")));
                },
            );
        }
    }
}

dylint_linting::dylint_library!();

#[expect(clippy::no_mangle_with_rust_abi)]
#[unsafe(no_mangle)]
pub fn register_lints(_sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    lint_store.register_lints(&[
        REQUIRE_BEHAVIOR_KEYWORD,
        TEST_NAME_MIN_WORDS,
        NO_TEST_PREFIX,
    ]);
    lint_store.register_late_pass(|_| Box::new(TestNameLints));
}
