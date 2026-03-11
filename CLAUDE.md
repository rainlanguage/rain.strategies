# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A collection of on-chain trading strategies written in Rain Language for the Raindex orderbook protocol. Each `.rain` file in `src/` is a self-contained strategy combining YAML configuration (above `---`) and Rain Language source code (below `---`).

## Architecture

### File Structure

- `settings.yaml` — Shared configuration: networks (base, polygon, arbitrum), RPCs, subgraph endpoints, orderbook addresses, rainlang contract addresses, token lists, and local DB sync settings.
- `src/*.rain` — Strategy files. Each contains YAML front matter (orders, scenarios, deployments, GUI config) followed by `---` then Rain Language source.
- `src/*.md` — Strategy documentation referenced by GUI description URLs.
- `registry` — Lists the settings URL and all strategy URLs pinned to a specific git commit hash. Must be updated when strategies or settings change.
- `token-lists/` — Custom token list JSON files.

### Rain File Format

Each `.rain` file has two sections separated by `---`:

1. **YAML section** (version 5 schema):
   - `version:` — Schema version (currently 5)
   - `orders:` — Order definitions referencing orderbooks and token pairs
   - `scenarios:` — Simulation/test scenarios with bindings
   - `deployments:` — Maps orders to scenarios for deployment
   - `gui:` — UI configuration with fields, presets, and token selectors
   - Optional: `orderbooks:`, `rainlangs:` — Strategy-specific overrides of settings.yaml

2. **Rain Language section**: Source code with `#`-prefixed named expressions (e.g., `#calculate-io`, `#handle-io`, `#handle-add-order`).

### Key Conventions

- Strategies reference shared config from `settings.yaml` by name (e.g., `orderbook: base` references `orderbooks.base` in settings).
- Strategy-specific orderbooks/rainlangs override shared settings when defined locally (see `canary.rain`, `fixed-limit.rain`).
- The `registry` file pins all URLs to a specific commit hash — update it after merging changes to main.
- Rain Language uses `using-words-from` to import subparser contracts, and `call<'function-name>()` for internal function calls.

### Supported Networks

Base, Polygon, Arbitrum — each with their own RPC endpoints, orderbook contracts, and rainlang addresses defined in `settings.yaml`.
