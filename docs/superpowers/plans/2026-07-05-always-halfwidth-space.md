# Always Halfwidth Space Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make Karukan emit ASCII halfwidth spaces whenever it inserts or commits a space character, while keeping Space as the conversion/candidate key.

**Architecture:** The shared Rust IME engine owns space-width behavior, so change `karukan-im` only. macOS and fcitx5 already pass `Keysym::SPACE` through to the engine and will inherit the behavior.

**Tech Stack:** Rust 2024 workspace, `cargo test -p karukan-im`, `cargo fmt`.

---

### Task 1: Update Engine Space Behavior

**Files:**
- Modify: `karukan-im/src/core/engine/tests/basic.rs`
- Modify: `karukan-im/src/core/engine/tests/live_conversion.rs`
- Modify: `karukan-im/src/core/engine/input.rs`
- Modify: `karukan-im/README.md`

- [x] **Step 1: Write failing tests**

Change the Empty-state tests in `karukan-im/src/core/engine/tests/basic.rs` to expect `" "` instead of `"\u{3000}"`, and rename the test names/comments from fullwidth to halfwidth. Change the Ctrl+Space tests in `karukan-im/src/core/engine/tests/live_conversion.rs` to expect `" "` and `"あ "`.

- [x] **Step 2: Verify RED**

Run:

```bash
cargo test -p karukan-im space_in_empty_hiragana_commits_halfwidth_space
cargo test -p karukan-im ctrl_space
```

Expected: tests fail because production code still emits `"\u{3000}"`.

- [x] **Step 3: Implement minimal code**

In `karukan-im/src/core/engine/input.rs`, replace the Empty-state `"\u{3000}"` commits/inserts and the Ctrl+Space helper with ASCII `" "`. Rename `input_fullwidth_space` to `input_halfwidth_space` and update comments.

- [x] **Step 4: Update documentation**

In `karukan-im/README.md`, change `Ctrl+Space | 全角スペースを入力` to halfwidth-space wording.

- [x] **Step 5: Verify GREEN**

Run:

```bash
cargo test -p karukan-im
cargo fmt --check
git status --short
```

Expected: tests pass, formatting is clean, and only intended files changed.

- [x] **Step 6: Commit**

Stage only this feature's files and commit:

```bash
git add docs/superpowers/plans/2026-07-05-always-halfwidth-space.md karukan-im/src/core/engine/tests/basic.rs karukan-im/src/core/engine/tests/live_conversion.rs karukan-im/src/core/engine/input.rs karukan-im/README.md
git commit -m "Make IME spaces halfwidth"
```
