# settings-zh-tw

## 摘要

- 目標：將本地 fork 中可見的 Settings 內容翻譯為繁體中文，降低日常使用時的英文介面負擔。
- 使用者可見行為：Settings 右側內容、對話框、說明文字、按鈕與驗證訊息多數改為繁體中文；左側 Settings navigation 保持英文，避免短標籤在目前字型 fallback 下顯示為方塊。
- 目前狀態：進行中，已涵蓋 Appearance、Features、Privacy、Code、Keyboard shortcuts、Warpify、Scripting、Warp Drive、MCP、third-party CLI agents、Environments 與 Oz Cloud API keys 相關 Settings 內容。
- 負責人 / 維護者：KaKuCoCo local fork。

## 範圍

包含：

- Settings 可見頁面的標題、說明文字、欄位 label、按鈕、提示、錯誤與確認對話框。
- `settings_page.rs` 中以 `localize_settings_text` 集中處理的通用 Settings 元件字串。
- Windows UI font fallback 修補，優先使用 `Microsoft JhengHei UI` 顯示繁中文字。

不包含：

- 左側 Settings navigation menu。此區目前保留英文。
- app 全域介面翻譯。
- 完整 i18n resource / locale framework。
- 已被 `warp-cloud-agent-removal` 隱藏的官方 billing、team、Warp login、Warp cloud 與官方 Warp Agent 大型頁面完整翻譯。

## Patch 策略

- Upstream base 假設：套用於 `warp-cloud-agent-removal` 之後，先決定哪些 Settings 頁會保留可見，再翻譯可見文字。
- 主要修改檔案或模組：
  - `app/src/appearance.rs`
  - `app/src/settings_view/settings_page.rs`
  - `app/src/settings_view/*_page.rs`
  - `app/src/settings_view/mcp_servers/*.rs`
  - `app/src/settings_view/platform/*.rs`
- 搜尋錨點：
  - `LOCAL-PATCH(settings-zh-tw)`
  - `localize_settings_text`
  - `settings_zh_tw_text`
  - `sidebar_label`
- Persisted data / schema 變更：無。
- 相容性備註：此 patch 以字串替換與局部 render helper 為主，不新增正式 i18n framework。未來 upstream 若加入 i18n，應優先改用官方 locale resource。

## Build 與測試

必要檢查：

```powershell
cargo fmt --check
git diff --check
cargo build -p warp --bin warp-oss --features release_bundle,gui --target x86_64-pc-windows-msvc
```

手動驗證：

- App 可啟動。
- Settings sidebar 仍為英文且不顯示方塊字。
- Appearance、Privacy、Code、MCP Servers、Third party CLI agents 等可見 Settings 頁面右側內容顯示繁體中文。
- 中文文字不顯示 missing-glyph 方塊。
- Third-party CLI agents settings 仍可進入，支援 `claude`、`codex` 與 `gemini`。
- MCP settings 仍可進入。

## 維護

rebase 到新版官方 stable tag 時：

1. 先套用 IME、Windows build bundle 與 `warp-cloud-agent-removal` patches。
2. 重新套用 `settings-zh-tw`。
3. 搜尋 `localize_settings_text` 與 `settings_zh_tw_text`，補上 upstream 新增的 Settings 通用字串。
4. 搜尋 `LOCAL-PATCH(settings-zh-tw)`，確認 Windows UI font 優先順序仍適用。
5. 搜尋新增或改名的 `app/src/settings_view/*` 檔案，盤點可見 Settings 頁是否有新增英文文案。
6. 重新 build 並手動檢查 Settings 中文顯示。
