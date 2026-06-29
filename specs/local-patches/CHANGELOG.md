# 本地 Patch Changelog

此 changelog 追蹤 KaKuCoCo 個人 Warp fork，不追蹤 upstream Warp。

## Unreleased

- 在 `local/feature/warp-cloud-agent-removal` 第一階段實作 settings 強隱藏：
  - sidebar 移除 Account、Billing、Teams、Referrals、Shared Blocks、Warp Drive、
    Cloud platform 與官方 Warp Agent 子頁。
  - Agents umbrella 只保留 `ThirdPartyCLIAgents`，MCP 改為獨立 settings page。
  - search / direct navigation fallback 會將 hidden entrypoints 導到
    `ThirdPartyCLIAgents`、`MCPServers` 或 `Appearance`。
  - command palette、URI deeplink、local-control 不再額外封鎖 hidden settings 入口；
    是否能實際渲染仍交由 settings page-level fallback 控制。
  - AI settings local 模式只渲染 `CLIAgentWidget`，Privacy 移除官方 cloud / service
    controls，Features 僅隱藏強綁官方 Warp Agent 的 controls。
  - 修復本機 Windows MSVC test build 會因空 compile-time feature flag 列表無法推斷
    型別的問題，並補上 `ai_page_handle` clone 讓 `cargo check` 通過。
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
