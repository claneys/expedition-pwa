# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Expedition** is a Tauri 2.x desktop (and Android) application for managing packing checklists for outdoor activities (ski touring, mountaineering, climbing, hiking, trekking). The UI is in French.

## Commands

All Rust commands run from `src-tauri/`:

```bash
# Build & run in development
cargo build
cargo run

# Release build
cargo build --release

# Run tests
cargo test

# Check without building
cargo check

# Lint
cargo clippy

# Tauri dev mode (requires tauri-cli)
cargo tauri dev

# Tauri production bundle
cargo tauri build

# Android build
cargo tauri android build
```

## Architecture

### Data Flow

The frontend (vanilla JS, pre-built in `../dist/`) communicates with the Rust backend exclusively via two Tauri IPC commands:

- `load_data` — reads `data.json` from the platform app data directory; returns `AppData::default()` if no file exists
- `save_data` — serializes the full `AppData` state to `data.json`

There is no incremental persistence: the entire state is loaded on startup and saved as a whole on any change.

### Rust Structure (`src/`)

| File | Role |
|------|------|
| `main.rs` | Binary entry point, delegates to `lib.rs::run()` |
| `lib.rs` | Data structs (`AppData`, `Activity`, `GearItem`), `Default` impl with hardcoded initial data, Tauri app initialization |
| `commands.rs` | `#[tauri::command]` handlers for `load_data` / `save_data` |

### Data Model

```
AppData {
    activities: Vec<Activity>          // ordered list of activities
    specific_gear: HashMap<String, Vec<GearItem>>  // keyed by activity id
    commun_gear: Vec<GearItem>         // shared across all activities
}

Activity { id: String, name: String, icon: String }
GearItem  { name: String, qty: u32, sub: String }
```

`sub` is the gear subcategory: `"vêtements"`, `"sécurité"`, `"confort"`, or `"technique"`.

### Frontend

The frontend is pre-built vanilla JS/HTML/CSS located in `../dist/`. The source is not tracked in this repository — only the compiled output is present. Calls to Tauri commands use `window.__TAURI__.core.invoke(...)`.

### Key Configuration

- `tauri.conf.json` — app identifier `fr.claneys.expedition`, window 600×900px fixed, frontend dist at `../dist`
- `capabilities/main.json` — window permissions
- `AndroidManifest.xml` — Android-specific config
- The library crate-type is `["staticlib", "cdylib"]` to support both desktop and mobile Tauri targets
