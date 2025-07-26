# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rain Strategies is a collection of trading strategies implemented in the Rain language for decentralized orderbook trading. The repository contains strategy files written in `.rain` format that define algorithmic trading behaviors on multiple blockchain networks.

## Repository Structure

- **`src/`**: Contains all trading strategy implementations
  - `.rain` files: Strategy implementations in Rain language
  - `.md` files: Strategy documentation and explanations
- **`settings.yaml`**: Network configuration for supported blockchains (RPC endpoints, chain IDs, subgraph URLs)
- **`token-lists/`**: Token configuration files for different networks
- **`registry`**: Version pinning file that maps strategy names to specific Git commit URLs

## Key Trading Strategies

The repository implements several core trading strategies:

1. **Auction DCA** (`auction-dca.rain`): Dollar-cost averaging through exponential price decay auctions
2. **Grid Trading** (`grid.rain`): Fixed-level grid trading with automatic recharging
3. **Dynamic Spread** (`dynamic-spread.rain`): Adaptive spread trading based on market conditions
4. **Fixed Limit** (`fixed-limit.rain`): Simple limit order implementation
5. **Canary** (`canary.rain`): Network health monitoring strategy that runs on cooldown

## Network Configuration

The project supports multiple blockchain networks configured in `settings.yaml`:
- **Arbitrum** (currently active): Main deployment network
- Other networks (Base, BSC, Polygon, Ethereum, Flare, Linea) are commented out

Each network configuration includes:
- RPC endpoints
- Chain ID and network ID
- Native currency
- Subgraph endpoints for data indexing
- Metaboard URLs for metadata
- Orderbook contract addresses
- Deployer contract addresses

## Rain Language Specifics

Strategy files use Rain language version 2 and follow this structure:
- `version: 2`: Rain language version declaration
- `scenarios`: Network-specific configurations
- `bindings`: Variable bindings for strategy parameters
- `runs`: Number of execution cycles

## Development Context

This is a strategy configuration repository rather than a traditional software project. There are no:
- Build scripts or compilation steps
- Test suites or testing frameworks
- Linting or code formatting tools
- Package managers or dependencies

## Working with Strategies

When modifying strategies:
1. Strategy files contain network-specific scenarios with different contract addresses
2. Each strategy has accompanying `.md` documentation explaining the trading logic
3. Bindings reference orderbook inputs/outputs using `${order.inputs.0.token.address}` syntax
4. Contract addresses (raindex-subparser) vary by network and must match deployed contracts

## Network Deployment

Currently only Arbitrum network is actively configured. Other networks are commented out in `settings.yaml` but can be enabled by uncommenting and updating contract addresses as needed.