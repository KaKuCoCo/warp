# 本地 Patch Changelog

此 changelog 追蹤 KaKuCoCo 個人 Warp fork，不追蹤 upstream Warp。

## Unreleased

- 在 `local/feature/warp-cloud-agent-removal` 上重新盤點目前 `master` 的
  settings navigation、search/deeplink/command palette 入口、AI/Privacy/Features
  widgets 與測試衝突點。
- 新增 root `AGENTS.md`，記錄 fork 維護、分支、worktree、版號、build、測試與
  changelog 規則。
- 新增 `specs/local-patches/` 維護結構與 feature template。
- 將 Warp cloud / official Agent 移除計畫遷移到
  `features/warp-cloud-agent-removal/`。
- 更新 Windows local build workflow，release tag 由官方 stable tag 加 local
  suffix 組成。

## v0.2026.06.03.09.49.stable_00-ime-pr10122.2

- 從官方 stable `v0.2026.06.03.09.49.stable_00` 加上 Warp PR `#10122` 建出可用的
  Windows OSS package。
- 修正前一版 release packaging 問題，改為包含完整 installer 與 portable runtime
  payload，而不是只包含單一 debug executable。

## v0.2026.06.03.09.49.stable_00-ime-pr10122.1

- 不可用 release。此版本只包了單一 debug executable，不應使用。
