# AGENTS.md

## Project

**Expedition** — Tauri 2.x desktop/Android app for outdoor packing checklists. UI is French. Rust backend + pre-built vanilla JS frontend.

## Commands

All Rust commands run from `src-tauri/`:

```bash
cd src-tauri

# Dev build & run
cargo run

# Release build
cargo build --release

# Check / lint
cargo check
cargo clippy

# Tests
cargo test

# Tauri dev (requires tauri-cli)
cargo tauri dev

# Tauri production bundle
cargo tauri build

# Android build
cargo tauri android build
```

## Architecture

- **Frontend**: pre-built vanilla JS/HTML/CSS in `../dist/` (source not tracked; only compiled output). Uses `window.__TAURI__.core.invoke(...)` for IPC.
- **Backend**: Rust in `src-tauri/src/`
  - `main.rs` — binary entrypoint, calls `expedition_lib::run()`
  - `lib.rs` — data structs (`AppData`, `Activity`, `GearItem`), `Default` impl with hardcoded initial data, Tauri app init
  - `commands.rs` — two `#[tauri::command]` handlers: `load_data`, `save_data`
- **Data flow**: entire `AppData` state loaded on startup from `data.json` (platform app data dir), saved as a whole on any change. No incremental persistence.
- **Crate type**: `staticlib` + `cdylib` to support desktop and mobile targets.

## Key Config

- `tauri.conf.json` — app id `fr.claneys.expedition`, window 600×900px fixed, frontend dist at `../dist`, `withGlobalTauri: true`
- `capabilities/main.json` — `core:default` permission for the `main` window
- `Cargo.toml` — depends on `tauri = "2"`, `tauri-plugin-shell = "2"`, `tauri-plugin-log = "2"`

## Data Model

```
AppData {
    activities: Vec<Activity>,
    specific_gear: HashMap<String, Vec<GearItem>>,
    commun_gear: Vec<GearItem>,
}

Activity { id, name, icon }
GearItem { name, qty, sub }
```

`sub` values: `"vêtements"`, `"sécurité"`, `"confort"`, `"technique"`.

## Notes

- The `dist/` directory contains the built frontend. Do not expect a modern JS build pipeline or framework source in this repo.
- `load_data` returns `AppData::default()` if `data.json` does not exist yet.
- Android build artifacts and `target/` are present; the project has been built for Android (`aarch64-linux-android`, `armv7-linux-androideabi`, etc.).
