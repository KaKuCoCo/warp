# Warp Cloud 與官方 Agent 介面移除維護

## 目前分支脈絡

- Working branch：`strip-warp-cloud-agent`
- Base branch：`ime-stable-20260603-pr10122`
- 目前官方 stable base：`v0.2026.06.03.09.49.stable_00`

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
5. 重新檢查 page-level routing，確保 search、command palette 與 deeplinks 不能進入
   hidden pages。
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
- `Features`，但清理 Agent-only settings。
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

隱藏 Agent / Warp AI-specific controls：

- `AutoOpenCodeReviewPaneWidget`
- agent 相關 desktop notification options
- `AtContextMenuInTerminalModeWidget`
- `SlashCommandsInTerminalModeWidget`
- `OutlineCodebaseSymbolsForAtContextMenuWidget`
- `ShowTerminalZeroStateBlockWidget`
- `DefaultSessionModeWidget` 內 agent-related parts

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
- Search、command palette 與 deeplinks 不能進入 hidden pages。
- Third-party CLI agents settings page 可進入。
- MCP Servers settings page 可進入。
- 基本 terminal workflow 正常。
