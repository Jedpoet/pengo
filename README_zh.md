# 🐧 pengo

> **小說家的 Cargo。**
> 為具有工程師思維的作者所打造的結構化、基於 CLI 的寫作工具。

**pengo** 是一款基於終端機的工具，旨在將軟體開發的**結構**、**版本控制**和**工作流程**帶入小說寫作的世界。

如果你已經厭倦了凌亂的 Word 文件、熱愛 Markdown，並認為你的小說值得擁有一個良好的專案結構——歡迎回家。

---

## 📖 為什麼選擇 pengo？

大多數寫作工具可分為兩類：

1. **太過簡單：** 缺乏專案管理的純文字編輯器。
2. **太過複雜：** 臃腫的 GUI 軟體（例如 Scrivener），會將你的資料鎖定在專有格式中。

**pengo** 找到了完美的平衡點。它將你的小說視為**專案**，將你的章節視為**原始碼檔案**，並將你的角色和場景視為**受管資產**。

---

## ✨ 功能特色（計畫中）

* 🏗️ **建立鷹架 (Scaffolding)** — 使用 `pengo new` 產生標準的目錄結構
* 📖 **章節管理** — 使用 `pengo chapter new` 建立並在你的 `$EDITOR` 中開啟下一章
* 💾 **Git 封裝** — 使用 `pengo save` 儲存進度，無需死記 git 指令
* 👤 **資產管理** — 透過 `pengo character` 和 `pengo scene` 管理角色和場景
* 💡 **捕捉靈感** — 使用 `pengo idea` 瞬間將想法附加到 `ideas.md`
* 📦 **匯出** — 使用 `pengo build` 將你的手稿編譯成單一檔案

---

## 🛠️ 安裝

> pengo 目前處於早期開發階段。

```bash
git clone https://github.com/jedpoet/pengo.git
cd pengo
cargo install --path pengo-cli

```

---

## 🚀 快速開始

### 1. 建立一部新小說

```bash
pengo new my-fantasy-novel
cd my-fantasy-novel

```

這會建立以下結構：

```
my-fantasy-novel/
├── pengo.toml          # 專案設定（標題、作者等）
├── outline.md          # 故事結構與規劃
├── ideas.md            # 零散的想法、台詞、靈感
├── .git/               # 自動初始化的 git 儲存庫
├── book/               # 你的手稿
│   └── ch001-opening.md
├── lore/               # 世界觀設定資產
│   ├── characters/
│   └── scenes/
└── drafts/             # 廢稿與刪除的內容

```

### 2. 撰寫下一章

```bash
pengo chapter new "The Encounter"
# 建立 book/ch002-the-encounter.md 並開啟 $EDITOR

# 提供簡短別名：
pengo next "The Encounter"

```

### 3. 新增角色

```bash
pengo character add "Alice"
# 使用中繼資料 (metadata) 範本建立 lore/characters/alice.md

```

### 4. 捕捉靈感

```bash
pengo idea "What if Alice was the villain all along?"
# 立即將想法附加到 ideas.md

```

### 5. 儲存進度

```bash
pengo save "Finished chapter 2"
# 相當於：git add . && git commit -m "Finished chapter 2"

```

---

## 🗂️ 指令參考

| 指令 | 別名 | 描述 |
| --- | --- | --- |
| `pengo new <name>` |  | 建立新的小說專案 |
| `pengo chapter new [title]` | `pengo next` | 建立並開啟下一章 |
| `pengo chapter list` |  | 列出所有章節 |
| `pengo character add <name>` |  | 新增角色 |
| `pengo character list` |  | 列出所有角色 |
| `pengo scene add <name>` |  | 新增場景/地點 |
| `pengo scene list` |  | 列出所有場景 |
| `pengo idea "<text>"` |  | 將想法附加到 ideas.md |
| `pengo save [message]` |  | 透過 git 提交目前進度 |
| `pengo status` |  | 顯示專案總覽與字數統計 |
| `pengo build` |  | 將手稿匯出為單一檔案 |

---

## ⚙️ 設定 (`pengo.toml`)

```toml
[book]
title = "The Rust Chronicles"
author = "Yu Yu"
version = "0.1.0"

[build]
output_format = ["epub", "pdf"]

[editor]
command = "nvim"   # 若未設定，則退回使用 $EDITOR
auto_open = true

```

---

## 🗺️ 開發路線圖 (Roadmap)

* [X] **階段 1 — 核心 CLI**
    - [X] 專案鷹架建立 (`pengo new`)
    - [X] 章節管理 (`pengo chapter`)
    - [X] 角色與場景管理
    - [ ] Git 封裝 (`pengo save`)
    - [X] 靈感捕捉 (`pengo idea`)


* [ ] **階段 2 — 完善與打磨**
    - [ ] 包含字數統計與進度的 `pengo status`
    - [ ] 透過 pandoc 進行 `pengo build` 匯出 (txt / epub / pdf)

* [ ] **階段 2.5 - 與主流編輯器整合**
    - [ ] Neovim 插件 (pengo.nvim)
    - [ ] Obsidian 插件

* [ ] **階段 3 — GUI**
    - 由 `pengo-core` 驅動的獨立 GUI 應用程式
    - 針對非開發者的寫作者



---

## 🏗️ 架構

pengo 被建構為一個 Cargo 工作區 (workspace)，以乾淨地分離邏輯與介面：

```
pengo/
├── pengo-core/   # 核心函式庫 — 所有邏輯都在這裡
├── pengo-cli/    # CLI 前端 — pengo-core 之上的輕量封裝
└── pengo-gui/    # GUI 前端（計畫中）

```

這意味著 CLI 與 GUI 共享同一個底層引擎。

---

## 📄 授權條款

本專案採用 MIT 授權條款授權。

