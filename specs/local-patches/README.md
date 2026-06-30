# 本地 Patches

這個資料夾記錄 KaKuCoCo Warp fork 攜帶的個人 patches。維護目標是快速更新到
官方最新 stable Warp tag、重新套用這些 patches，並建出 Windows
`WarpOssSetup.exe`。

## 目前基底

- 官方 stable base：`v0.2026.06.03.09.49.stable_00`
- 目前 local branch：`strip-warp-cloud-agent`
- 目前 IME base branch：`ime-stable-20260603-pr10122`
- 目前已知可用 local release：
  `v0.2026.06.03.09.49.stable_00-ime-pr10122.2`
- 不可用的歷史 local release：
  `v0.2026.06.03.09.49.stable_00-ime-pr10122.1`

新 release 應使用以下版號格式：

```text
<official-stable-tag>-kakucoco.<N>
```

範例：

```text
v0.2026.06.03.09.49.stable_00-kakucoco.1
```

## Patch 套用順序

更新到新版官方 stable tag 時，依以下順序套用本地 patches：

1. Warp PR `#10122` 的 Windows IME marked text 支援，直到選定的官方 stable
   base 已包含此修補為止。
2. 產出可用 installer 與 portable package 所需的 Windows OSS release
   bundle/build 修補。
3. `warp-cloud-agent-removal`：隱藏或停用 Warp login、cloud、billing、team、
   referral、官方 Warp Agent 與 Warp-managed cloud agent 介面，同時保留
   third-party CLI agents。
4. `settings-zh-tw`：將可見 Settings 內容翻譯為繁體中文，並讓 Windows UI font
   優先使用可顯示繁中文字的系統字型。
5. 未來新增的 local patches，依此處列出的順序套用。

新增、移除或調整 patch 順序時，必須在同一個 branch 更新此清單。

## Feature 索引

| Feature | 狀態 | 文件 |
| --- | --- | --- |
| `warp-cloud-agent-removal` | 規劃中 / 進行中 | `features/warp-cloud-agent-removal/` |
| `settings-zh-tw` | 進行中 | `features/settings-zh-tw/` |

## Upgrade 檢查清單

1. 查詢 `warpdotdev/warp` 的 GitHub Releases。
2. 選擇最新一個 tag 包含 `.stable_` 且 `prerelease` 欄位為 `false` 的 release。
3. Fetch tags，並從該 tag 建立 `base/stable/<yyyymmdd>`。
4. 建立 `local/upgrade/<official-tag>`。
5. 依上方 patch 順序重新套用 patches。
6. 解決衝突，並更新每個受影響的 `MAINTENANCE.md`。
7. 執行：

   ```powershell
   cargo check -p warp --bin warp-oss --features release_bundle,gui --target x86_64-pc-windows-msvc
   ```

8. 執行 Windows build workflow，並確認 `WarpOssSetup.exe`。
9. 對 installer build 做 smoke test。
10. Merge 到 `master` 並更新 `CHANGELOG.md`。

## 自動化 TODO

- 新增每日 workflow，檢查官方 GitHub Releases 是否有新版 stable release。
- 發現新版 stable 時，讓 workflow 建立 issue 或 draft `local/upgrade/<official-tag>`
  branch。
- 在 patches、測試與 `WarpOssSetup.exe` 經人工驗證前，不自動發版。
