# The Cure -> Rust Terminal game + Learning playground.

A multi-package Rust game framework for building terminal-based simulations with modular rendering, input, and orchestration.

This workspace includes:
- `client`: The terminal-based game frontend.
- `server`: The game logic engine.
- `launcher`: Orchestrates launching both client and server.
- `shared`: Shared types and logic across crates.
- `renderer`: Modular rendering backends (e.g., Ratatui).
- `input`: Modular input backends (e.g., Crossterm).

## Overview

This project is designed for fast prototyping of terminal UI games, supporting features like:
- Modular renderer and input layers
- Async server-client architecture
- Launcher that spins up the full stack
- Game state driven by ECS and event sourcing

Ideal for simulations, management games, or roguelike prototypes.

---

## Getting Started

### 1. Install Rust

Make sure you have the latest stable version of Rust installed:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Then restart your shell and check:
```bash
rustup show
cargo --version
```

### 2. Clone the repo
```bash
git clone https://github.com/ArtOfSettling/DystopianCeoSimulator.git
cd DystopianCeoSimulator
```

### 3. Build the full workspace
```bash
cargo build --workspace
```

### 4. Run the launcher
```bash
cargo run -p launcher
```

## Workspace Layout
```
├── client/                  # Terminal-based UI client
├── server/                  # Game logic and simulation engine
├── launcher/                # Orchestrates launching client/server
├── shared/                  # Shared types and logic
├── renderer/
│   ├── renderer-api/        # Trait definitions for rendering
│   └── renderer-ratatui/    # Ratatui renderer implementation
├── input/
│   ├── input-api/           # Trait definitions for input
│   └── input-crossterm/     # Crossterm input implementation
```

## Contributing
Welcome contributions! Please see CONTRIBUTING.md for details.





