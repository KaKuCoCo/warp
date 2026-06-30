# KaKuCoCo Warp Fork 維護指南

這個 repository 是個人維護的 Warp fork。主要目標是能快速基於官方最新
stable release tag，重新套用本地客製 patch，並建出可使用的 Windows
安裝檔 (`WarpOssSetup.exe`)。

下方 upstream 開發指南仍適用於一般 Rust、UI、測試與風格規範。本節針對此個人
fork 的維護、分支、發版與本地 patch 文件規範，優先於 upstream 的貢獻流程。

## 維護目標

- 追蹤 `warpdotdev/warp` 的官方 stable release。
- 每次官方 stable 更新後，以最低衝突成本重新套用本地 patches。
- 主要產物是 Windows OSS channel 安裝檔：
  `WarpOssSetup.exe`。
- 每個本地客製功能都必須有足夠文件，讓另一個 session 能在 upstream 衝突後快速修正。
- 本地改動應保持小範圍且可搜尋。patch 專用 gate 附近優先使用具名 helper
  或 `LOCAL-PATCH(<slug>)` 註解。

目前重要的本地 patches 包含 Windows IME 修補，以及規劃中的 Warp login、
Warp cloud、billing、team、官方 Warp Agent 介面移除/隱藏。這些只是本地
patch 的例子，不是此 fork 的唯一目標。

## 分支規則

- `master`：最新已驗證、可發版的個人 stable build。
- `base/stable/<yyyymmdd>`：從官方 stable tag 建立的乾淨基底分支。
- `local/upgrade/<official-tag>`：更新到新版官方 stable tag 時使用的分支。
- `local/feature/<slug>`：單一客製功能或維護功能分支。

不要在同一個 feature branch 混入無關的本地 patches。如果某個改動同時更新
官方基底並修正本地 patch 衝突，兩者都要記錄到
`specs/local-patches/CHANGELOG.md`。

## Worktree 規則

- 每個 upgrade 或 feature branch 都使用獨立 worktree。
- 預設 worktree root：`$WARP_WORKTREES`。
- 預設路徑格式：`$WARP_WORKTREES/<branch-slug>`。
- 本機開發建議讓所有 worktree 共用同一個 Cargo target directory，避免每個
  worktree 各自重建大型 Rust artifacts。預設建議路徑：
  `D:\programming\warp-cargo-target`。
- 分支 merge 且不再需要後，移除 worktree 並執行
  `git worktree prune`。

開始編輯前先檢查：

```powershell
git status --short --branch
git worktree list
```

開始本機 build 前，確認目前 shell 已設定共用 target：

```powershell
$env:CARGO_TARGET_DIR="D:\programming\warp-cargo-target"
```

若要長期套用到新的 PowerShell session，可設定使用者環境變數：

```powershell
[Environment]::SetEnvironmentVariable("CARGO_TARGET_DIR", "D:\programming\warp-cargo-target", "User")
```

同一個 `CARGO_TARGET_DIR` 不建議同時給多個 worktree 平行執行大型 build。若需要
平行 build，請暫時改用不同 target directory，避免 lock contention 或 artifacts
互相覆蓋導致重編。

除非使用者明確要求，不要刪除或 reset 其他 worktree 裡的使用者改動。

## 本地 Patch 文件

所有本地 patches 都放在 `specs/local-patches/`。

必要文件：

- `specs/local-patches/README.md`：patch index、目前基底、目前 local release、patch 套用順序。
- `specs/local-patches/CHANGELOG.md`：個人 fork changelog。
- `specs/local-patches/templates/FEATURE.md`：新 patch 文件模板。
- `specs/local-patches/features/<feature-slug>/README.md`：使用者可見目標、行為、範圍與主要實作筆記。
- `specs/local-patches/features/<feature-slug>/MAINTENANCE.md`：重套步驟、grep anchors、衝突熱點與驗證清單。

每個本地 feature branch merge 回 `master` 前，都必須新增或更新對應 feature 文件。

## 官方 Stable 更新流程

使用最新一個 tag 包含 `.stable_` 且 `prerelease` 為 `false` 的 GitHub Release。

1. Fetch 官方 tags。
2. Checkout 最新官方 stable tag。
3. 建立或更新 `base/stable/<yyyymmdd>`。
4. 建立 `local/upgrade/<official-tag>`。
5. 依 `specs/local-patches/README.md` 中列出的順序重新套用本地 patches。
6. 解決衝突。
7. 如果 anchors 或衝突點改變，更新受影響的
   `features/<feature-slug>/MAINTENANCE.md`。
8. 執行必要檢查與 Windows build。
9. 將已驗證的 upgrade merge 回 `master`。
10. 更新 `specs/local-patches/CHANGELOG.md`。

未來自動化應每天檢查官方 releases。發現新版 stable 時，只建立 issue 或 draft
upgrade branch；在人工驗證前不得自動發版。

## 版號規則

本地 release 使用官方 stable tag 加個人後綴：

```text
<official-stable-tag>-kakucoco.<N>
```

範例：

```text
v0.2026.06.03.09.49.stable_00-kakucoco.1
```

同一個官方 stable tag 重 build 時遞增 `<N>`。換到新版官方 stable tag 後從
`.1` 重新開始。

舊的 local releases 可能使用 `-ime-pr10122.<N>` 等歷史後綴。保留這些 tags
作為歷史紀錄，但新 release 統一使用 `-kakucoco.<N>`。

## Build 與測試規則

feature 或 upgrade merge 到 `master` 前，至少執行：

```powershell
cargo check -p warp --bin warp-oss --features release_bundle,gui --target x86_64-pc-windows-msvc
```

release 驗證時，執行 Windows GitHub Actions build，並確認產出：

- 主要 artifact：`WarpOssSetup.exe`
- 可選 artifact：`WarpOssPortable.zip`

每個 release candidate 都要做 smoke test：

- App 可啟動。
- Windows 中文 IME marked/preedit text 仍正常。
- 基本 terminal workflow 正常。
- 主要本地客製功能仍正常。
- 對於 cloud/agent removal patch，Settings 不得露出 Warp login、billing、
  teams、cloud 或官方 Warp Agent 介面。
- Third-party CLI agents settings 仍可進入，支援 `claude`、
  `codex` 與 `gemini`。
- MCP settings 仍可進入，除非後續 patch 明確改變此行為。

## Changelog 規則

每個 merge 回 `master` 的 feature 或 upgrade 都必須更新
`specs/local-patches/CHANGELOG.md`，除非該分支明確標記
`CHANGELOG-NONE`。

changelog entries 應記錄：

- 官方 stable 基底變更。
- 本地 patch 新增、移除與行為變更。
- Build workflow 變更。
- 未來維護者需要知道的驗證備註。

未來 CI 應在有 code changes、但未更新 `specs/local-patches/CHANGELOG.md` 且未宣告
`CHANGELOG-NONE` 時，讓 PR 或 branch merge 失敗。

# Upstream AGENTS.md

This file provides guidance when working with code in this repository.

## Development Commands

### Build and Run
- `cargo run` - Build and run Warp locally
- `cargo bundle --bin warp` - Bundle the main app

### Running with local warp-server
To connect Warp client to a local warp-server instance:

```bash
# Connect to server on default port 8080
cargo run --features with_local_server

# Connect to server on custom port (e.g., 8082)
SERVER_ROOT_URL=http://localhost:8082 WS_SERVER_URL=ws://localhost:8082/graphql/v2 cargo run --features with_local_server
```

Environment variables:
- `SERVER_ROOT_URL` - HTTP endpoint (default: `http://localhost:8080`)
- `WS_SERVER_URL` - WebSocket endpoint (default: `ws://localhost:8080/graphql/v2`)

### Testing
- `cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2` - Run tests with nextest
- `cargo nextest run -p warp_completer --features v2` - Run completer tests with v2 features
- `cargo test --doc` - Run doc tests
- `cargo test` - Run standard tests for individual packages

### Linting and Formatting
- `./script/presubmit` - Run all presubmit checks (fmt, clippy, tests)
- `./script/format` - Format code
- `cargo clippy --workspace --all-targets --all-features --tests -- -D warnings` - Run clippy
- `./script/run-clang-format.py -r --extensions 'c,h,cpp,m' ./crates/warpui/src/ ./app/src/` - Format C/C++/Obj-C code
- `find . -name "*.wgsl" -exec wgslfmt --check {} +` - Check WGSL shader formatting

### Platform Setup
- `./script/bootstrap` - Platform-specific setup plus common agent skill installation from `skills-lock.json`; prompts for project/global when an install or update is needed unless a target flag or environment override is provided.
- `./script/bootstrap --skip-common-skills` - Platform setup without installing or updating common agent skills.
- `./script/bootstrap --install-common-skills` - Explicitly install common agent skills from `skills-lock.json`; this is the default behavior.
- `./script/bootstrap --install-common-skills-in-repo` - Platform setup plus common agent skill installation in this checkout's `.agents/skills`.
- `./script/bootstrap --install-common-skills-globally` - Platform setup plus common agent skill installation in `~/.agents/skills`.
- `../common-skills/scripts/install_common_skills --repo-root "$PWD" --project --if-needed` - Install or refresh shared agent skills in this checkout's `.agents/skills`.
- `../common-skills/scripts/install_common_skills --repo-root "$PWD" --global --if-needed` - Install or refresh shared agent skills in `~/.agents/skills`.
- `../common-skills/scripts/remove_common_skills --repo-root "$PWD"` - Remove shared agent skills listed in `skills-lock.json` from this checkout's `.agents/skills`.
- `../common-skills/scripts/remove_common_skills --repo-root "$PWD" --global` - Remove shared agent skills listed in `skills-lock.json` from `~/.agents/skills`.
- `../common-skills/scripts/remove_common_skills --repo-root "$PWD" --clear-lock` - Remove shared agent skills from this checkout and delete `skills-lock.json`.
- `./script/install_cargo_build_deps` - Install Cargo build dependencies
- `./script/install_cargo_test_deps` - Install Cargo test dependencies

`skills-lock.json` is the standard project lock file managed by `npx skills`. `warpdotdev/common-skills/scripts/install_common_skills` requires an explicit install target before restoring: pass `--project`, pass `--global`, set `WARP_COMMON_SKILLS_INSTALL_TARGET`, or answer the interactive prompt from bootstrap. Non-interactive flows fail if no target is explicit. The installer creates `skills-lock.json` from `warpdotdev/common-skills` if it is missing, uses global as the recommended interactive default, errors if common skills are present in both project and global locations, prevents a global install pinned to one lock from being silently overwritten by another checkout pinned to a different lock, and verifies installed skills against the lock after successful install or skip paths. `script/run` and `script/bootstrap` execute this installer with `script/resolve_common_skills`, which uses `WARP_COMMON_SKILLS_SCRIPTS_DIR` only when explicitly set and otherwise runs the raw script from `warpdotdev/common-skills`. To test a remote common-skills branch, set `WARP_COMMON_SKILLS_REF=<branch>`. Cloud setup should use `common-skills/scripts/install_common_skills --repo-root <warp-checkout> --project --if-needed --non-interactive` or set `WARP_COMMON_SKILLS_INSTALL_TARGET=project` to avoid the prompt. To update the locked common skills, run `npx --yes skills@1.5.6 update -p -y` and commit the resulting `skills-lock.json` changes.

## Architecture Overview

This is a Rust-based terminal emulator with a custom UI framework called **WarpUI**.

### Key Components

**WarpUI Framework** (`ui/`):
- Custom UI framework with Entity-Component-Handle pattern
- Global `App` object owns all views/models (entities)
- Views hold `ViewHandle<T>` references to other views
- `AppContext` provides temporary access to handles during render/events
- Elements describe visual layout (Flutter-inspired)
- Actions system for event handling
- MouseStateHandle must be created once during construction, and then referenced/cloned anywhere we're using mouse input to track mouse changes. Inline `MouseStateHandle::default()` while rendering will cause no mouse interactions to work.

**Main App** (`app/`):
- Terminal emulation and shell management (`terminal/`)
- AI integration including Agent Mode (`ai/`)
- Cloud synchronization and Drive features (`drive/`)
- Authentication and user management (`auth/`)
- Settings and preferences (`settings/`)
- Workspace and session management (`workspace/`)

**Core Libraries**:
- `crates/warp_core/` - Core utilities and platform abstractions
- `crates/editor/` - Text editing functionality
- `crates/warpui/` and `crates/warpui_core/` - Custom UI framework
- `crates/ipc/` - Inter-process communication
- `crates/graphql/` - GraphQL client and schema

### Key Architectural Patterns

1. **Entity-Handle System**: Views reference other views via handles, not direct ownership
2. **Modular Structure**: Workspace contains multiple workspace configurations, each with terminals, notebooks, etc.
3. **Cross-Platform**: Native implementations for macOS, Windows, Linux, plus WASM target
4. **AI Integration**: Built-in AI assistant with context awareness and codebase indexing
5. **Cloud Sync**: Objects can be synchronized across devices via Warp Drive

### Development Guidelines

**Workspace Structure**:
- This is a Cargo workspace with 60+ member crates
- Main binary is in `app/`, UI framework in `crates/warpui/`
- Platform-specific code is conditionally compiled
- Integration tests are in `crates/integration/`

**Coding Style Preferences**:
- Avoid unnecessary type annotations, especially in closure params.
- Avoid using too many Rust path qualifiers and use imports for concision. Place import statements at the top of the file as per convention.
  An exception to this is inside cfg-guarded code branches. In those cases, you can either embed the import into the relevant scope or just use an absolute path for one-offs.
- If a function takes a context parameter (`AppContext`, `ViewContext`, or `ModelContext`), it should be named `ctx` and go last. The one exception is for
  functions that take a closure parameter, in which case the closure should be last.
- Always remove unused parameters completely rather than prefixing them with `_`. Update the function signature and all call sites accordingly.
- Prefer inline format arguments in macros like `println!`, `eprintln!`, and `format!` (for example, `eprintln!("{message}")` instead of `eprintln!("{}", message)`) to satisfy Clippy's `uninlined_format_args` lint.
- Do not pass `Itertools::format` results directly to logging macros (`log::*`, `safe_*`, etc.). `Itertools::format` produces a single-use formatter, while logging implementations may format a message more than once. Use a reusable `String` such as `iter.join(", ")` for logging arguments instead. Direct use in `format!` or `write!` is fine.
- Do not remove existing comments when making unrelated changes. Only remove or modify a comment if the logic it describes has changed.
- When adding a toggleable setting, also add the matching Command Palette enable/disable entry and any required context flags so the setting is discoverable outside Settings.

**Terminal Model Locking**:
- Be extremely careful when calling `model.lock()` on the terminal model (`TerminalModel`). Acquiring multiple locks on the same model from different call sites can cause a deadlock, resulting in a UI freeze (beach ball on macOS).
- Before adding a new `model.lock()` call, verify that no caller in the current call stack already holds the lock.
- Prefer passing already-locked model references down the call stack rather than acquiring new locks.
- If you must lock the model, keep the lock scope as short as possible and avoid calling other functions that might also attempt to lock.

**Testing**:
- Use `cargo nextest` for parallel test execution
- Integration tests use custom framework in `integration/`
- Tests should be run via presubmit script before submitting
- Unit tests should be placed in separate files using the naming convention `${filename}_tests.rs` or `mod_test.rs`
- Test files should be included at the end of their corresponding module with:
  ```rust
  #[cfg(test)]
  #[path = "filename_tests.rs"]  // or "mod_test.rs"
  mod tests;
  ```

**Pull Request Workflow**:
- **ALWAYS** run `./script/format` and `cargo clippy` (the versions specified in ./script/presubmit) before opening a PR or pushing updates to an existing PR branch
- Those commands must pass completely before creating or updating a pull request
- Specifically, ensure `./script/format` and `cargo clippy` checks pass
- If they fail, fix all issues before proceeding with the PR
- Do not create public pull requests or public issues that disclose a non-public security vulnerability. Refer users to `SECURITY.md` for the proper disclosure methods instead.
- This applies to:
  - Opening new pull requests
  - Pushing new commits to existing PR branches
  - Any branch updates that will be reviewed
 - When opening PRs, use the PR template at `.github/pull_request_template.md`
 - Add changelog entries when appropriate using the format at the bottom of the PR template. Use the following prefixes (without the `{{}}` brackets):
   - `CHANGELOG-NEW-FEATURE:` for new, relatively sizable features (use sparingly - these may get marketing/docs)
   - `CHANGELOG-IMPROVEMENT:` for new functionality of existing features
   - `CHANGELOG-BUG-FIX:` for fixes related to known bugs or regressions
   - `CHANGELOG-IMAGE:` for GCP-hosted image URLs
   - Leave changelog lines blank or remove them if no changelog entry is needed

**Database**:
- Uses Diesel ORM with SQLite
- Migrations in `crates/persistence/migrations/`
- Schema defined in `crates/persistence/src/schema.rs`

**GraphQL**:
- Schema and client code generation from `crates/warp_graphql_schema/api/schema.graphql`
- TypeScript types generated for frontend integration

### Feature Flags

Warp uses compile-time feature flags with a small runtime plumbing layer.

How to add a feature flag:
- Add a new variant to `warp_core/src/features.rs` in the `FeatureFlag` enum
- (Optional) Enable it by default for dogfood builds by listing it in `DOGFOOD_FLAGS`
- Gate code paths with `FeatureFlag::YourFlag.is_enabled()`
- For preview or release rollout, add to `PREVIEW_FLAGS` or `RELEASE_FLAGS` respectively (as appropriate)

Best practices:
- **Prefer runtime checks over cfg directives**: Prefer `FeatureFlag::YourFlag.is_enabled()` over `#[cfg(...)]` compile-time directives so flags can be toggled without recompilation and are easier to clean up later. Use `#[cfg(...)]` only when the code cannot compile without them (for example, platform-specific code or dependencies that do not exist when the feature is disabled).
- Keep flags high-level and product-focused rather than per-call-site
- Remove the flag and dead branches after launch has stabilized
- For UI sections that expose a new feature, hide the UI behind the same flag

Example:
```rust
#[derive(Sequence)]
pub enum FeatureFlag {
    YourNewFeature,
}

// Default-on for dogfood builds
pub const DOGFOOD_FLAGS: &[FeatureFlag] = &[
    FeatureFlag::YourNewFeature,
];

// Use in code
if FeatureFlag::YourNewFeature.is_enabled() {
    // gated behavior
}
```

### Exhaustive Matching

When adding/editing match statements, avoid using the wildcard _ when at all possible. Exhaustive matching is helpful for ensuring that all variants are handled, especially when adding new variants to enums in the future.
