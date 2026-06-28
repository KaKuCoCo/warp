# <feature-slug>

## 摘要

- 目標：
- 使用者可見行為：
- 目前狀態：
- 負責人 / 維護者：

## 範圍

包含：

- 待補。

不包含：

- 待補。

## Patch 策略

- Upstream base 假設：
- 主要修改檔案或模組：
- 搜尋錨點：
  - `LOCAL-PATCH(<feature-slug>)`
- Persisted data / schema 變更：
- 相容性備註：

## Build 與測試

必要檢查：

```powershell
cargo check -p warp --bin warp-oss --features release_bundle,gui --target x86_64-pc-windows-msvc
```

手動驗證：

- App 可啟動。
- 基本 terminal workflow 正常。
- 此 feature 的專屬行為正常。

## 維護

rebase 到新版官方 stable tag 時：

1. 依 `specs/local-patches/README.md` 列出的順序，在前置 patches 之後重新套用此 patch。
2. 搜尋上方列出的錨點。
3. 重新檢查所有衝突熱點。
4. 執行必要檢查。
5. 如果 upstream code 移動，更新本文件與 feature `MAINTENANCE.md`。
