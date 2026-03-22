# 🐧 PenGo
> **Cargo for Novelists.**
> A structured, CLI-based writing tool for authors who think like engineers.

[![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**pengo** is a terminal-based tool designed to bring the **structure**, **version control**, and **workflow** of software development into the world of novel writing.

If you are tired of messy Word documents, love Markdown, and believe your novel deserves a proper project structure—welcome home.

---

## 📖 Why PenGo?

Most writing tools fall into two categories:

1. **Too Simple:** Plain text editors that lack project management.
2. **Too Complex:** Bloated GUI software (e.g. Scrivener) that locks your data in proprietary formats.

**PenGo** hits the sweet spot. It treats your novel as a **project**, your chapters as **source files**, and your characters and scenes as **managed assets**.

---

## ✨ Features (Planned)

- 🏗️ **Scaffolding** — Generate a standard directory structure with `pengo new`
- 📖 **Chapter Management** — Create and open the next chapter in your `$EDITOR` with `pengo chapter new`
- 💾 **Git Wrapper** — Save progress with `pengo save` without memorizing git commands
- 👤 **Asset Management** — Manage characters and scenes via `pengo character` and `pengo scene`
- 💡 **Idea Capture** — Instantly append a thought to `ideas.md` with `pengo idea`
- 📦 **Export** — Compile your manuscript into a single file with `pengo build`

---

## 🛠️ Installation

> PenGo is currently in early development.

```bash
git clone https://github.com/jedpoet/pengo.git
cd pengo
cargo install --path pengo-cli
```

---

## 🚀 Quick Start

### 1. Create a new novel

```bash
pengo new my-fantasy-novel
cd my-fantasy-novel
```

This creates the following structure:

```
my-fantasy-novel/
├── pengo.toml          # Project config (title, author, etc.)
├── outline.md          # Story structure and planning
├── ideas.md            # Loose thoughts, lines, inspiration
├── .git/               # Auto-initialized git repo
├── book/               # Your manuscript
│   └── ch001-opening.md
├── lore/               # World-building assets
│   ├── characters/
│   └── scenes/
└── drafts/             # Scraps and deleted content
```

### 2. Write the next chapter

```bash
pengo chapter new "The Encounter"
# Creates book/ch002-the-encounter.md and opens $EDITOR

# Short alias available:
pengo next "The Encounter"
```

### 3. Add a character

```bash
pengo character add "Alice"
# Creates lore/characters/alice.md with a metadata template
```

### 4. Capture an idea

```bash
pengo idea "What if Alice was the villain all along?"
# Appends the thought to ideas.md instantly
```

### 5. Save your progress

```bash
pengo save "Finished chapter 2"
# Equivalent to: git add . && git commit -m "Finished chapter 2"
```

---

## 🗂️ Command Reference

| Command | Alias | Description |
|---|---|---|
| `pengo new <name>` | | Create a new novel project |
| `pengo chapter new --volume [volume] [title]` | `pengo next --volume [volume] [title]` | Create and open the next chapter |
| `pengo chapter list --volume [volume]` | | List all chapters |
| `pengo character add <name>` | | Add a new character |
| `pengo character list` | | List all characters |
| `pengo scene add <name>` | | Add a new scene/location |
| `pengo scene list` | | List all scenes |
| `pengo idea "<text>"` | | Append a thought to ideas.md |
| `pengo save [message]` | | Commit current progress via git |
| `pengo status` | | Show project overview and word count |
| `pengo build` | | Export manuscript to a single file |

---

## ⚙️ Configuration (`pengo.toml`)

```toml
[book]
title = "The Rust Chronicles"
author = "Yu Yu"
version = "0.1.0"

[build]
output_format = ["epub", "pdf"]

[editor]
command = "nvim"   # Falls back to $EDITOR if not set
auto_open = true
```

---

## 🗺️ Roadmap

- [X] **Phase 1 — Core CLI**
  - [X] Project scaffolding (`pengo new`)
  - [X] Chapter management (`pengo chapter`)
  - [X] Character and scene management
  - [ ] Git wrapper (`pengo save`)
  - [X] Idea capture (`pengo idea`)

- [ ] **Phase 2 — Polish**
  - [ ] `pengo status` with word count and progress
  - [ ] `pengo build` export via pandoc (txt / epub / pdf)

- [ ] **Phase 2.5 - Editor Integrations**
  - [ ] Neovim Plugin (pengo.nvim)
  - [ ] Obsidian Plugin

- [ ] **Phase 3 — GUI**
  - Standalone GUI application powered by `pengo-core`
  - Targets non-developer writers

---

## 🏗️ Architecture

pengo is structured as a Cargo workspace to cleanly separate logic from interface:

```
pengo/
├── pengo-core/   # Core library — all logic lives here
├── pengo-cli/    # CLI frontend — thin wrapper over pengo-core
└── pengo-gui/    # GUI frontend (planned)
```

This means the CLI and GUI share the same underlying engine.

---

## 📄 License

This project is licensed under the MIT License.
