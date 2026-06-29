# Warp Cloud 與官方 Agent 介面移除維護

## 目前分支脈絡

- Working branch：`local/feature/warp-cloud-agent-removal`
- 歷史建議分支：`strip-warp-cloud-agent`，此分支基於舊 IME 基底，勿直接 merge 回目前 `master`。
- Base branch：目前從 `master` 開新分支；IME 與 Windows build 文件已在 `master` 整合。
- 目前官方 stable base：`v0.2026.06.03.09.49.stable_00`

## 2026-06-28 重新盤點

本盤點基於 `master` 的 `1be263b3` 之後開出的
`local/feature/warp-cloud-agent-removal`。

## 2026-06-28 第一階段實作

本階段沒有物理刪除 account/cloud/billing/team/official Agent models，而是在
settings 可見入口加強隱藏與 fallback：

- `app/src/settings_view/mod.rs`
  - 新增 `is_local_warp_cloud_ui_disabled()` 作為本 patch 的集中 grep anchor。
  - `SettingsSection::ai_subpages()` 在本地模式只回傳 `ThirdPartyCLIAgents`。
  - `SettingsView::new` 的 local sidebar 只保留：
    - `Agents` umbrella：`ThirdPartyCLIAgents`
    - `MCPServers`
    - `Code`
    - `Appearance`
    - `Features`
    - `Keybindings`
    - `Warpify`
    - `Privacy`
    - `About`
  - `filtered_pages()` / `should_render_page()` 會排除 hidden backing pages。
  - `initial_page` 與 `set_and_refresh_current_page_internal()` 會把 hidden entrypoints
    redirect 到 `ThirdPartyCLIAgents`、`MCPServers` 或 `Appearance`。
  - `open_mcp_servers_page()` 在本地模式直接開 `MCPServers`，不再經過
    `AgentMCPServers`。
- `app/src/settings_view/ai_page.rs`
  - local 模式下 `AISettingsPageView::build_page()` 與 `set_active_subpage()` 都正規化為
    `ThirdPartyCLIAgents`，只渲染 `CLIAgentWidget`。
  - local 模式下 `on_page_selected()` 不刷新 Warp AI usage。
- `app/src/settings_view/privacy_page.rs`
  - local 模式保留 `SecretRedactionWidget` 與既有 flag gate 的 `NetworkLogWidget`。
  - 移除 analytics、crash reports、cloud conversation storage、data management 與
    privacy policy widget 的 settings surface。
  - command palette toggle bindings 只保留 secret redaction。
- `app/src/settings_view/features_page.rs`
  - 恢復 upstream controls，不做額外 local-only 隱藏。
  - `auto_open_code_review_pane_on_first_agent_change`、agent task completion
    notifications、in-app agent notifications 都不是 official Warp Agent 專用入口。
- `app/src/workspace/mod.rs`
  - command palette / editable bindings 維持 upstream 行為，不做額外 local-only disable。
- `app/src/uri/mod.rs`
  - settings deeplink 維持 upstream 行為，不做額外 local-only redirect 或 disable。
- `app/src/local_control/handlers/app_state.rs`
  - `surface.settings.open` 維持 upstream 行為；仍保留 upstream 原本對 `WarpDrive`
    的限制。
- `app/src/settings_view/mod_tests.rs`
  - 更新 local sidebar/search/fallback 測試。

驗證狀態：

- `cargo fmt`：通過。
- `git diff --check`：通過。
- `cargo check -p warp --bin warp-oss --features release_bundle,gui --target x86_64-pc-windows-msvc`：
  2026-06-28 修復本機 MSVC Build Tools / Windows SDK 後通過。
- `cargo test -p warp settings_view::tests:: --no-default-features --features gui --target x86_64-pc-windows-msvc`：
  2026-06-28 通過，49 passed。

### Settings navigation

主要入口在 `app/src/settings_view/mod.rs`：

- `SettingsSection` 目前包含所有需要處理的 section：`Account`、
  `BillingAndUsage`、`Teams`、`Referrals`、`SharedBlocks`、`WarpDrive`、
  `WarpAgent`、`AgentProfiles`、`AgentMCPServers`、`Knowledge`、
  `ThirdPartyCLIAgents`、`CloudEnvironments`、`OzCloudAPIKeys`。
- `SettingsSection::ai_subpages()` 目前順序是 `WarpAgent`、`AgentProfiles`、
  `AgentMCPServers`、`Knowledge`、`ThirdPartyCLIAgents`。第一階段應改成只保留
  `ThirdPartyCLIAgents`，並決定 `AgentMCPServers` 是否只透過獨立
  `MCPServers` page 保留。
- `SettingsView::new` 目前會無條件建立所有 settings page handles，並在
  `nav_items` 中加入 Account、Agents umbrella、Billing、Code umbrella、
  Cloud platform umbrella、Teams、Referrals、SharedBlocks、WarpDrive、Privacy 等。
  第一階段可先從 `nav_items` 強隱藏，不急著停止建立 backing views。
- `initial_page` 與 `set_and_refresh_current_page_internal` 目前會把
  `SettingsSection::AI` fallback 到 `WarpAgent`。本地模式要改成 fallback 到
  `ThirdPartyCLIAgents` 或其他保留頁，避免舊 deeplink/restore 進入官方 Agent。
- `should_render_page` 與 `set_and_refresh_current_page_internal()` 是 hidden page 的最後
  防線；command palette、deeplink、local-control 可維持 upstream 入口行為，但不能讓
  hidden section 最終渲染出來。

### Search 與 keyboard navigation

`app/src/settings_view/mod.rs` 的 `handle_search_editor_event` 會逐一掃
`SettingsSection::ai_subpages()` 和 `SettingsSection::code_subpages()`，並用
`subpage_filter` 控制搜尋結果。隱藏官方 Agent subpages 時，必須同步更新：

- `ai_subpages()`。
- search filter 走訪邏輯是否仍會掃 hidden subpages。
- collapsed umbrella navigation 測試。

`app/src/settings_view/mod_tests.rs` 有多個 nav/search tests 假設 Agents umbrella
包含 `WarpAgent`、`AgentProfiles`、`AgentMCPServers`、`Knowledge`、
`ThirdPartyCLIAgents`，實作後必須更新。

### Deeplink / command palette / local-control

本 patch 目前不再對這三種入口做額外 local-only 封鎖。原因是使用者希望保留
upstream 入口行為；hidden page 的實際渲染仍由
`SettingsView::set_and_refresh_current_page_internal()` 與
`should_render_page()` 的 fallback 決定。

- `app/src/uri/mod.rs`
  - `warp://settings/teams`
  - `warp://settings/billing_and_usage`
  - `warp://settings/environments`
  - `warp://settings/platform`
  - `warp://settings/warp_agent`
  - `settings_section_for_simple_subpage()` 維持 upstream mapping。
- `app/src/workspace/mod.rs`
  - command palette / editable bindings 目前註冊 Account、Shared Blocks、Teams、
    AI、Billing、Referrals、Environments、MCP Servers 等 settings actions。
  - 本地模式不在 command palette 層移除或 disable 這些 actions。
- `app/src/local_control/handlers/app_state.rs`
  - `surface.settings.open` 會用 `SettingsSection::from_str` 解析任意 page。
  - 目前只保留 upstream 原本對 `WarpDrive` 的限制。
- `app/src/root_view.rs` 和 `app/src/workspace/view.rs`
  - root/workspace action 會直接 dispatch `WorkspaceAction::ShowSettingsPage(section)`。
  - 本地 fallback 放在 `SettingsView::set_and_refresh_current_page_internal`，
    作為所有入口的最後防線。

### AI / Agents page

`app/src/settings_view/ai_page.rs` 目前狀態：

- `AISubpage` 包含 `WarpAgent`、`Profiles`、`Knowledge`、`ThirdPartyCLIAgents`。
- `AISettingsPageView::build_page(None)` 會組出完整 AI page，包含
  `GlobalAIWidget`、`UsageWidget`、`ActiveAIWidget`、`AgentsWidget`、
  `AIInputWidget`、`MCPServersWidget`、`AIFactWidget`、`VoiceWidget`、
  `CloudHandoffWidget`、`CLIAgentWidget`、`ApiKeysWidget`、`AwsBedrockWidget`、
  `AgentAttributionWidget`、`CloudAgentComputerUseWidget`。
- `Some(AISubpage::WarpAgent)` 仍會顯示官方 Agent/AI 相關 widgets。
- `Some(AISubpage::ThirdPartyCLIAgents)` 只加入 `CLIAgentWidget`，是本地模式應保留的
  主要頁面。
- 若 `AgentMCPServers` 保留在 Agents umbrella，它實際映射到獨立 `MCPServers`
  backing page；若要讓 Agents 只顯示 third-party CLI agents，MCP 應只保留獨立
  `MCP Servers` sidebar page。

### Privacy page

`app/src/settings_view/privacy_page.rs` 的 `build_page()` 目前順序：

- 保留候選：`SecretRedactionWidget`，以及視需求保留 `NetworkLogWidget`。
- 隱藏候選：`AppAnalyticsWidget`、`CrashReportsWidget`、
  `CloudConversationStorageWidget`、`DataManagementWidget`。
- `PrivacyPolicyWidget` 屬品牌/外部政策連結，可保留或後續再決定。

### Features page

`app/src/settings_view/features_page.rs` 目前仍有多個 AI/Agent 控制。這些設定沒有直接
暴露 Warp login、billing、team、cloud platform 或 official Agent settings page，
且部分會被 third-party CLI agents 使用，因此 local 模式目前全部保留：

- `DefaultSessionModeWidget`。
- `AutoOpenCodeReviewPaneWidget`。
- `DesktopNotificationsWidget` 中的 agent task completed notification。
- `DesktopNotificationsWidget` 中的 needs-attention notifications。
- `DesktopNotificationsWidget` 中的 in-app agent notifications。
- `AtContextMenuInTerminalModeWidget`：AI context menu。
- `SlashCommandsInTerminalModeWidget`：依賴 `AISettings::is_any_ai_enabled`。
- `OutlineCodebaseSymbolsForAtContextMenuWidget`：AI context codebase outline。
- `ShowTerminalInputMessageLineWidget`。
- `ShowTerminalZeroStateBlockWidget`：由 `FeatureFlag::AgentView` / AI 設定控制。

非 agent terminal 功能仍應保留。

### Account / billing / teams / cloud pages

需要從 sidebar/search/deeplink/command palette 強隱藏：

- `app/src/settings_view/main_page.rs`
  - `AccountWidget`、`SettingsSyncWidget`、`EarnRewardsWidget`、
    `IapCredentialsWidget`、`LogoutWidget`。
  - `VersionInfoWidget` 可留在 About 或 diagnostics，但第一階段不需要保留 Account page。
- `billing_and_usage_page.rs`、`billing_and_usage_page_v2.rs`、
  `billing_and_usage_dispatch.rs`。
- `teams_page.rs`。
- `referrals_page.rs`。
- `show_blocks_view.rs`。
- `warp_drive_page.rs`。
- `environments_page.rs`。
- `platform_page.rs`。

### 額外內部入口風險

多個 AI/terminal path 會直接打開 `WarpAgent` 或 `BillingAndUsage`，例如
`app/src/ai/blocklist/**`、`app/src/terminal/input.rs`、`app/src/terminal/view.rs`、
`app/src/terminal/profile_model_selector.rs`。第一階段不必逐一刪除所有 call sites，
但 `SettingsView` 層必須集中 redirect hidden sections，否則這些入口仍可打開 hidden page。

## 重套步驟

1. 先重新套用 Windows IME 與 Windows build packaging patches，除非新版官方 stable
   tag 已經包含這些改動。
2. 搜尋 settings navigation construction：

   ```powershell
   rg -n "SettingsNavItem::Page|SettingsNavItem::Umbrella|ai_subpages|SettingsView::new" app/src/settings_view
   ```

3. 還原集中式 local gate，例如
   `is_local_warp_cloud_ui_disabled()`。
4. 重新隱藏 account、billing、teams、referrals、shared blocks、Warp Drive、
   cloud platform 與官方 Warp Agent pages 的 settings navigation entries。
5. 重新檢查 page-level routing，確保 sidebar/search/direct navigation 仍不渲染
   hidden pages；command palette、deeplinks、local-control 維持 upstream 入口行為。
6. 重新檢查 `AISettingsPageView::build_page`，此 patch 第一階段只保留
   third-party CLI agents。
7. 重新檢查 Features 與 Privacy pages 是否新增 upstream cloud、AI、Agent、Oz、
   account、billing 或 team controls。
8. 執行必要 build check 與 smoke tests。

## 搜尋錨點

每次官方 stable 更新後使用以下搜尋：

```powershell
rg -n "LOCAL-PATCH\\(warp-cloud-agent-removal\\)|is_local_warp_cloud_ui_disabled" app crates
rg -n "Account|BillingAndUsage|Teams|Referrals|SharedBlocks|WarpDrive|CloudEnvironments|OzCloudAPIKeys" app/src/settings_view
rg -n "WarpAgent|AgentProfiles|Knowledge|ThirdPartyCLIAgents|CLIAgentWidget|AISettingsPageView::build_page" app/src/settings_view/ai_page.rs app/src/settings_view/mod.rs
rg -n "analytics|crash|conversation|cloud|account|billing|team|agent|AI|Oz" app/src/settings_view/privacy_page.rs app/src/settings_view/features_page.rs
```

## 衝突熱點

- `app/src/settings_view/mod.rs` 內的 settings sidebar 與 umbrella page construction。
- `app/src/settings_view/ai_page.rs` 內的 AI/Agents subpage list 與 AI page widget
  rendering。
- `app/src/settings_view/main_page.rs` 內的 Account page widgets。
- `app/src/settings_view/privacy_page.rs` 內的 Privacy widgets。
- `app/src/settings_view/features_page.rs` 內的 Agent/Warp AI feature controls。
- Billing、teams、referrals、shared blocks、Warp Drive、cloud environments 與 Oz
  cloud API keys 的 full settings pages。

## 詳細盤點

### Sidebar Sections

定義於 `app/src/settings_view/mod.rs` 的 `SettingsView::new`。

移除或隱藏：

- `Account`：account identity、sign up、logout、billing、settings sync、
  referral CTA、IAP credentials。
- `BillingAndUsage`：Warp usage 與 billing。
- `Cloud platform` umbrella：
  - `CloudEnvironments`：ambient/cloud agent runtime environments。
  - `OzCloudAPIKeys`：cloud agents 使用的 Warp/Oz cloud API keys。
- `Teams`：Warp team/workspace admin、team billing、invites、discovery。
- `Referrals`：Warp referral program。
- `SharedBlocks`：Warp shared blocks / cloud sharing。
- `WarpDrive`：Warp Drive workflows/notebooks/prompts/environment variables，account gated。

保留：

- `Appearance`
- `Features`
- `Keybindings`
- `Warpify`
- `Privacy`，但清理 cloud/official AI-only widgets。
- `About`
- `MCP Servers`

調整：

- `Agents` umbrella 第一階段只保留 `ThirdPartyCLIAgents`。
- MCP 保持可透過獨立 `MCP Servers` page 進入。

### Account Page

檔案：`app/src/settings_view/main_page.rs`

隱藏：

- `AccountWidget`
- `SettingsSyncWidget`
- `EarnRewardsWidget`
- `IapCredentialsWidget`
- `LogoutWidget`

如果此頁仍保留給 diagnostics，可保留：

- `VersionInfoWidget`

第一階段偏好從 navigation 隱藏 `SettingsSection::Account`，而不是把它重設計成
diagnostics page。

### AI / Agents Page

檔案：`app/src/settings_view/ai_page.rs`

保留：

- `CLIAgentWidget`，此 widget 控制 third-party CLI agent toolbar settings。

隱藏：

- `GlobalAIWidget`
- `UsageWidget`
- `ActiveAIWidget`
- `AgentsWidget`
- `AIInputWidget`
- AI page 內的 `MCPServersWidget`
- `AIFactWidget`
- `VoiceWidget`
- `CloudHandoffWidget`
- `CloudAgentComputerUseWidget`
- `AgentAttributionWidget`
- `OtherAIWidget`
- `AwsBedrockWidget`
- `ApiKeysWidget`，除非未來 local patch 明確保留 local BYOK model support。

### Privacy Page

檔案：`app/src/settings_view/privacy_page.rs`

保留：

- `SecretRedactionWidget`
- `NetworkLogWidget`，如果仍有用且仍由既有 context flags gate。

隱藏：

- `AppAnalyticsWidget`
- `CrashReportsWidget`
- `CloudConversationStorageWidget`
- `DataManagementWidget`

### Code Page

檔案：`app/src/settings_view/code_page.rs`

可能隱藏：

- `CodeIndexing`，因為它主要服務 Warp Agent/AI context。

變更前需檢查：

- `EditorAndCodeReview`，因為部分 editor behavior 在沒有官方 Warp Agent 時仍可能有用。

### Features Page

檔案：`app/src/settings_view/features_page.rs`

目前保留：

- `AtContextMenuInTerminalModeWidget`
- `SlashCommandsInTerminalModeWidget`
- `OutlineCodebaseSymbolsForAtContextMenuWidget`
- `ShowTerminalInputMessageLineWidget`
- `ShowTerminalZeroStateBlockWidget`
- `DefaultSessionModeWidget`
- needs-attention desktop notification option
- `AutoOpenCodeReviewPaneWidget`
- agent task completion desktop notification option
- in-app agent notification option

檢查後保留 non-agent terminal controls。

### Warp Drive

檔案：`app/src/settings_view/warp_drive_page.rs`

隱藏整個 page。如果仍保留任何 AI widgets，也要從 AI settings 移除任何
"Warp Drive as agent context" UI。

### Cloud Platform

檔案：

- `app/src/settings_view/environments_page.rs`
- `app/src/settings_view/platform_page.rs`

隱藏：

- `CloudEnvironments`
- `OzCloudAPIKeys`

### Billing / Teams / Referrals / Shared Blocks

檔案：

- `app/src/settings_view/billing_and_usage_page.rs`
- `app/src/settings_view/billing_and_usage_page_v2.rs`
- `app/src/settings_view/teams_page.rs`
- `app/src/settings_view/referrals_page.rs`
- `app/src/settings_view/show_blocks_view.rs`

隱藏整個 sections。這些功能依賴 Warp account、team、server state、billing
metadata 或 cloud sharing。

## 驗證清單

- `cargo check -p warp --bin warp-oss --features release_bundle,gui --target x86_64-pc-windows-msvc`
- App 可啟動。
- Windows 中文 IME candidate/preedit 正常。
- Settings sidebar 隱藏所有已移除 sections。
- Search 與 sidebar/direct navigation 不顯示 hidden pages；command palette、
  deeplinks、local-control 不做額外 local-only 封鎖。
- Third-party CLI agents settings page 可進入。
- MCP Servers settings page 可進入。
- 基本 terminal workflow 正常。
