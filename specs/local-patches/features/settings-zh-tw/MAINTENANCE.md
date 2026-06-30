# settings-zh-tw Maintenance

## 重套步驟

1. 確認已先套用 `warp-cloud-agent-removal`，讓 Settings 可見範圍固定。
2. 套用 `app/src/appearance.rs` 的 Windows UI font 優先順序修補。
3. 套用 `app/src/settings_view/settings_page.rs` 的集中翻譯 helper。
4. 套用各 Settings page 的直接字串翻譯。
5. 保持 `SettingsSection::sidebar_label()` 的英文 navigation label，不要把左側 menu 改回繁中。

## Grep Anchors

```powershell
rg -n "LOCAL-PATCH\(settings-zh-tw\)|localize_settings_text|settings_zh_tw_text|sidebar_label" app/src specs/local-patches
rg -n "\"[A-Za-z][^\"]*\"" app/src/settings_view
```

## 衝突熱點

- `app/src/appearance.rs`
  - Upstream 可能調整 Windows UI font 載入邏輯。保留 `Microsoft JhengHei UI` 優先，`Segoe UI` fallback。
- `app/src/settings_view/settings_page.rs`
  - 通用 Settings 元件 render helper 常被 upstream 調整；衝突時優先維持翻譯集中入口。
- `app/src/settings_view/mod.rs`
  - `SettingsSection` display name 與 sidebar navigation 可能改動。左側 sidebar 應使用英文 `sidebar_label()`。
- `app/src/settings_view/code_page.rs`
  - Code indexing 相關 UI 變動頻繁，尤其 LSP / repository indexing rows。
- `app/src/settings_view/ai_page.rs`
  - Third-party CLI agents 與 custom inference 文案多，upstream 新增 providers 時需補翻譯。
- `app/src/settings_view/mcp_servers/*.rs`
  - MCP 安裝、更新、刪除確認文案常跟功能變動一起改。

## 驗證清單

- `cargo fmt --check`
- `git diff --check`
- `cargo build -p warp --bin warp-oss --features release_bundle,gui --target x86_64-pc-windows-msvc`
- 手動開啟 Settings，確認：
  - Sidebar menu 是英文。
  - 右側內容顯示繁體中文。
  - 沒有 missing-glyph 方塊字。
  - MCP Servers 與 Third party CLI agents 頁面仍可進入。
