# 3. support_os_versions

Date: 2026-07-03

## Status

Accepted

## Context

CIのビルドの実装をしているときに、ビルド対象にUbuntu(Linux)を含めるべきかが議論になった。

結果的に、`Linux`を除く、`Windows`、`MacOS`のみの対応とすることになったが、

OS自体のバージョンをどこまでサポートするべきだという議論が出た。

そしてそのCPUのアーキテクチャのサポートもどこまで行うべきかという議論も出た。

## Decision

`Github Actions`の[runner images](https://github.com/actions/runner-images)がサポートしているOSのバージョンを対象とする。

CPUのアーキテクチャは`Windows`は`x64`、`MacOS`は`arm64`をサポートする。

## Consequences

ビルド環境に現状Github Actionsを採用しているため、その環境内におけるバージョンはGithub ActionsのサポートしているOSのバージョンに依存することになる。

 また、CPUのアーキテクチャは一般的に利用されているアーキテクチャをサポートすることにする。
