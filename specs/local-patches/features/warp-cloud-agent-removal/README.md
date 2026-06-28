# 移除 Warp Cloud 與官方 Agent 介面

## 摘要

此本地 patch 會從個人 fork 移除或隱藏 Warp account、Warp cloud、billing、
teams、referrals、官方 Warp Agent，以及 Warp-managed cloud agent 相關介面。

目標不是立即刪除所有底層 model。第一階段先強力隱藏使用者可見入口，同時維持
build 穩定性，並讓未來更新官方 stable 時更容易 rebase。

## 使用者可見行為

移除或隱藏：

- Warp login、sign up、logout、account plan、billing、settings sync、referral
  CTA 與 account data management。
- Billing 與 usage settings。
- Teams settings 與 team billing/member management。
- Referrals。
- Shared blocks 與 Warp cloud sharing。
- Warp Drive cloud workflows、notebooks、prompts 與 environment variables。
- Cloud platform pages：
  - `CloudEnvironments`
  - `OzCloudAPIKeys`
- 官方 Warp Agent pages：
  - `WarpAgent`
  - `AgentProfiles`
  - `Knowledge`
- Privacy 內的 cloud / official service controls。
- Features 內的 Agent / Warp AI 專用 controls。

保留：

- Third-party CLI agents settings，用於 `claude`、`codex` 與 `gemini`。
- 獨立的 MCP Servers settings。
- Appearance。
- Keybindings。
- Warpify。
- About。
- 非 agent 的 terminal features。
- Privacy 內的 `SecretRedactionWidget`。

## Patch 策略

- 優先隱藏 settings navigation 與 page entry points。
- 第一階段保留底層 data models 與 persisted settings schemas。
- 新增集中 helper，例如 `is_local_warp_cloud_ui_disabled()`，所有 local-only
  隱藏行為都經由此 helper。
- 避免散落 magic `cfg` checks 或不相關 feature flags。
- 在 helper 或 local-only gate 附近加上簡短
  `LOCAL-PATCH(warp-cloud-agent-removal)` 註解，讓未來 rebase 可快速找到 patch。

## 主要實作區域

- `app/src/settings_view/mod.rs`
  - `is_local_warp_cloud_ui_disabled()`
  - `SettingsView::new`
  - `SettingsSection::ai_subpages()`
  - hidden / deep-linked pages 的 navigation fallback
- `app/src/settings_view/ai_page.rs`
  - `AISettingsPageView::build_page`
  - 保留 `CLIAgentWidget`
- `app/src/settings_view/privacy_page.rs`
  - 移除 cloud/account/telemetry widgets，同時保留有用的 local privacy controls
- `app/src/settings_view/features_page.rs`
  - 隱藏 agent / Warp AI-specific controls
- `app/src/workspace/mod.rs`
  - command palette / editable bindings 入口收斂
- `app/src/uri/mod.rs`
  - settings deeplink 收斂
- `app/src/local_control/handlers/app_state.rs`
  - `surface.settings.open` 禁止 hidden pages
- `app/src/settings_view/mod_tests.rs`
  - local sidebar / search / fallback 行為測試
- 候選的整頁 settings：
  - `billing_and_usage_page.rs`
  - `billing_and_usage_page_v2.rs`
  - `teams_page.rs`
  - `referrals_page.rs`
  - `show_blocks_view.rs`
  - `warp_drive_page.rs`
  - `environments_page.rs`
  - `platform_page.rs`

第一階段採用 navigation 與 page-level filtering；上述整頁檔案仍保留，避免物理刪除造成
官方 stable 更新時的大量衝突。

## Build 與測試

必要檢查：

```powershell
cargo check -p warp --bin warp-oss --features release_bundle,gui --target x86_64-pc-windows-msvc
```

手動驗證：

- App 可啟動。
- Settings sidebar 不再顯示 Warp login/account、billing、teams、referrals、
  shared blocks、Warp Drive、cloud platform 或官方 Warp Agent pages。
- Search、command palette 與 deeplinks 不能導向 hidden pages。
- Third-party CLI agents settings page 仍可進入。
- Third-party CLI agent toolbar settings 仍可設定。
- MCP Servers settings 仍可進入。
- 基本 terminal workflow 正常。
- Windows 中文 IME candidate/preedit 行為仍正常。
