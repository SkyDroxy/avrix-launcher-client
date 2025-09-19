# Avrix Launcher

Standalone Tauri-based launcher for Avrix. This repository contains the Vue 3 + Vite frontend and the Tauri (Rust) backend.

## Quick start
- Install Node.js (LTS or current) and Rust toolchain
- Install pnpm: `npm i -g pnpm`
- Install dependencies: `pnpm i`
- Dev mode: `pnpm tauri:dev`
- Build: `pnpm tauri:build`

## Structure
- `src/` – Vue 3 app (Tailwind v4)
- `src-tauri/` – Tauri 2 backend (Rust)
- `scripts/` – helper scripts

## Notes
- The build helper `scripts/copy-exe.cjs` is safe to run standalone and will skip copying if the legacy monorepo installer path is missing.

## License
MIT or project license (update as needed)
