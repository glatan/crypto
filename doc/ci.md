# CI

GitLab Runnerの設定について

## Variables

標準では以下のターゲットのテストのみ実行する

- stable-\*-linux-\*
- stable-wasm32-*

## 実行環境

### Runner

#### \*-linux-\*, wasm32-*

saas-linux-small-amd64

https://docs.gitlab.com/ee/ci/runners/hosted_runners/linux.html#machine-types-available-for-linux---x86-64

#### \*-pc-windows-\*

saas-windows-medium-amd64

https://docs.gitlab.com/ee/ci/runners/hosted_runners/windows.html#machine-types-available-for-windows
