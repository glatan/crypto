# CI

GitLab Runnerの設定について

## Variables

標準では以下のターゲットのテストのみ実行する

- stable-\*-linux-\*
- stable-wasm32-*

### ステージ制御

- DISABLE_STABLE_TEST: true
  - stable_testステージをスキップする
- ENABLE_NIGHTLY_TEST: true
  - nightly_testステージを実行する
- ENABLE_TIER3_BUILD: true
  - tier3_buildステージを実行する
- ENABLE_TIER3_TEST: true
  - tier3_testステージを実行する

### ターゲット制御

- DISABLE_LINUX_TEST: true
  - \*-linux-\*ターゲットのテストジョブをスキップする
- DISABLE_WASM32_TEST: true
  - wasm32-*ターゲットのテストジョブをスキップする
- ENABLE_WINDOWS_MSVC_TEST: true
  - *-pc-windows-msvcターゲットのテストジョブを実行する
- ENABLE_WINDOWS_GNU_TEST: true
  - *-pc-windows-gnuターゲットのテストジョブを実行する
- ENABLE_WINDOWS_GNULLVM_TEST: true
  - *-pc-windows-gnullvmターゲットのテストジョブを実行する

## 実行環境

### Runner

#### \*-linux-\*, wasm32-*

saas-linux-small-amd64

https://docs.gitlab.com/ee/ci/runners/hosted_runners/linux.html#machine-types-available-for-linux---x86-64

#### \*-pc-windows-\*

saas-windows-medium-amd64

https://docs.gitlab.com/ee/ci/runners/hosted_runners/windows.html#machine-types-available-for-windows
