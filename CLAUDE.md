# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build                 # Build all crates
cargo run                   # Run the application
cargo clippy                # Lint
cargo fmt                   # Format code
just run [debug|trace]      # Run with logging (see justfile)
```

Development environment: `nix develop` or `direnv allow`

## Architecture

D&D 5e spell/encounter management desktop app using Iced GUI framework (v0.13.1).

### Crate Structure

```
initiative (binary) - Entry point, initializes logging and runs app
├── app             - Application state, update/view (Elm architecture)
│   └── types
├── views           - UI screens (index, root, spells)
│   ├── components
│   └── types
├── assets          - Embedded icons/fonts via rust-embed
└── logging         - Tracing subscriber setup

types               - Domain models: Spell, Message, View, FormMode
                      All spell properties are strongly-typed enums with strum derives
```

### Key Patterns

- **Elm architecture**: `Initiative` struct implements `update(&mut self, Message) -> Task<Message>` and `view(&self) -> Element<Message>`
- **Message-driven**: All state changes go through `types::Message`
- **Enum-heavy domain**: Spell properties (school, level, range, duration, etc.) are all enums using `strum::Display` + `strum::VariantArray`
- **Embedded assets**: Icons/fonts compiled into binary via `rust-embed`

## Workspace Lints (Denied)

`unsafe_code`, `dbg_macro`, `todo`, `allow_attributes`, `allow_attributes_without_reason`

Use `tracing` macros instead of `dbg!` or `println!`.
