# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A collection of on-chain trading strategies written in Rain Language for the Raindex protocol. Each `.rain` file in `src/` is a self-contained strategy combining YAML configuration (above `---`) and Rain Language source code (below `---`).

## Architecture

### File Structure

- `settings.yaml` — Shared configuration: networks (base), RPCs, subgraph endpoints, raindex addresses, rainlang contract addresses, token lists, and local DB sync settings.
- `src/*.rain` — Strategy files. Each contains YAML front matter (orders, scenarios, deployments, GUI config) followed by `---` then Rain Language source.
- `src/*.md` — Strategy documentation referenced by GUI description URLs.
- `registry` — Lists the settings URL and all strategy URLs pinned to a specific git commit hash. Must be updated when referenced settings or strategy files change.
- `token-lists/` — Custom token list JSON files.

### Rain File Format

Each `.rain` file has two sections separated by `---`:

1. **YAML section** (version 6 schema):
   - `version:` — Schema version (currently 6)
   - `orders:` — Order definitions referencing raindexes and token pairs
   - `scenarios:` — Simulation/test scenarios with bindings
   - `deployments:` — Maps orders to scenarios for deployment
   - `gui:` — UI configuration with fields, presets, and token selectors
   - Optional: `raindexes:`, `rainlangs:` — Strategy-specific overrides of settings.yaml

2. **Rain Language section**: Source code with `#`-prefixed named expressions (e.g., `#calculate-io`, `#handle-io`, `#handle-add-order`).

### Key Conventions

- Strategies reference shared config from `settings.yaml` by name (e.g., `raindex: base` references `raindexes.base` in settings).
- Strategy-specific raindexes/rainlangs override shared settings when defined locally (see `canary.rain`, `fixed-limit.rain`).
- The `registry` file pins all URLs to a specific commit hash. See the registry hash workflow below.
- Rain Language uses `using-words-from` to import subparser contracts, and `call<'function-name>()` for internal function calls.

### Registry Hashes

Registry URLs should point to the latest commit that changed the referenced settings or strategy file content.

When a PR changes any file referenced by `registry`, make the settings/strategy changes first, then update `registry` in a follow-up commit using the content commit hash. Do not point registry URLs at the registry-update commit itself: a commit cannot contain its own final hash.

If a later commit changes any referenced settings or strategy file again, update `registry` again to point at that newer content commit.

### Supported Networks

Base — with its RPC endpoints, raindex contract, and rainlang address defined in `settings.yaml`.
