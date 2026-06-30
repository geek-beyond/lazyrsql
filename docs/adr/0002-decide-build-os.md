# 2. decide_build_os

Date: 2026-06-17

## Status

Accepted

## Context

CIのビルドを実装しているときに、ビルド対象に`Ubuntu(Linux)`が含まれていた。

しかし、現状LinuxのOSのマシンを持っている人がいない、そのビルドの必要性について議論になった。

## Decision

対応するOSは`Ubuntu(Linux)`を除く、`Windows`、`MacOS`のみとする

## Consequences

現状開発メンバーには`Windows`と`MacOS`のユーザーしかいないので、`Ubuntu(Linux)`の対応は見送る
