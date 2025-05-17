#!/bin/bash

set -euo pipefail

# 1. Build and checks (customize as needed)

echo "Running cargo build --release..."
cargo build --release

echo "Running cargo fmt check..."
cargo fmt -- --check

echo "Running cargo clippy..."
cargo clippy --all --all-targets -- -D warnings

echo "All checks passed! Creating a pull request..."

# 2. Create a PR using GitHub CLI

# Detect current branch
branch=$(git rev-parse --abbrev-ref HEAD)

# Use main as base branch by default (customize if needed)
base_branch="main"

# Prompt for PR title and body or use defaults
read -rp "PR Title: " pr_title
pr_title=${pr_title:-"Automated build and lint checks"}

read -rp "PR Body: " pr_body
pr_body=${pr_body:-"This PR was created automatically after successful build and checks."}

# Create the PR
gh pr create --base "$base_branch" --head "$branch" --title "$pr_title" --body "$pr_body"

echo "Pull request created successfully!"
