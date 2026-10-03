# Flux Architecture Decisions

This document records the architectural decisions made for the Flux download manager.

## 1. Rust + Tauri 2 vs Electron / C++
**Decision**: Use Rust + Tauri 2.
**Rationale**: Rust provides high performance, zero-cost abstractions, and memory safety, essential for multi-connection downloads. Tauri 2 allows building a small, native-feeling shell without shipping a full Chromium instance (unlike Electron).

## 2. React 19 + Zustand vs Vue / Redux
**Decision**: Use React 19 + TypeScript with Zustand.
**Rationale**: React has the largest ecosystem. Zustand allows for fast, selector-based re-renders, fulfilling the requirement of max 1 re-render per 250ms.

## 3. SQLite vs JSON / LevelDB
**Decision**: Use SQLite (via `sqlx`).
**Rationale**: Downloads are highly structured and require transactional updates for segments and recovery. SQLite offers ACID properties and crash safety.

## 4. UI Theme Engine (Style Dictionary + Tailwind v4)
**Decision**: Use CSS variables generated via Style Dictionary.
**Rationale**: Fulfills the requirement of zero direct hex colors in components and <100ms theme switching time. 

## 5. Event Batching
**Decision**: Batch download progress events every 250ms.
**Rationale**: Frequent progress updates (per byte) will cause UI stutter (event storm). We only send diffs every 250ms.

## 6. Browser Extension Connection
**Decision**: Native Messaging as default, with WebSocket as a disabled-by-default fallback.
**Rationale**: Native Messaging provides a secure boundary. The WebSocket fallback is maintained for environments where Native Messaging cannot be used, but requires a 256-bit token authentication and strict origin checks to prevent unauthorized downloads. See `EXTENSION_SECURITY.md` for the full contract.

## Open Decisions & Assumptions
- **Native Window Effects**: Tauri 2 `window-vibrancy` will be used for Glass UI, but fallback to CSS opacity is needed for Linux WebKitGTK.
- **Default Theme**: To be decided between System or Dark.
- **Project Naming**: Assuming "Flux" is final for now.
