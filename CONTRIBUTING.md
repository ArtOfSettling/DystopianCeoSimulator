# Contributing to Rust Terminal Game

Thanks for your interest in contributing!

We welcome bug reports, feature requests, questions, and pull requests.

## How to Contribute

### 1. Fork the Repository

Use GitHub's "Fork" button to create your own copy of the repo.

Then clone your fork locally:

```bash
git clone git@github.com:ArtOfSettling/the_cure.git
cd YOUR_FORK
```

### 2. Set Up Your Environment

Make sure Rust is installed:

```bash
rustup update
cargo build --workspace
```

### 3. Make Changes
Write your code, make your changes and run cargo fmt.

If you're adding a new renderer or input backend:
- Add your module under renderer/ or input/
- Implement the appropriate trait defined in renderer-api or input-api

### 4. Submit a Pull Request

Push your branch and open a pull request to the main branch.

Please include:
- A summary of what you changed
- Any discussion or motivation
- Screenshots if it's a UI improvement

## Code Style

- Use rustfmt for formatting.
- Use clippy to lint:
```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features
```
