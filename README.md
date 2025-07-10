# Release CLI

A Rust command-line tool for managing semantic versioning and Git releases with support for scoped tags.

## Features

- **Semantic Versioning**: Supports major, minor, and patch version bumping following semver standards
- **Scoped Tags**: Organize releases by scope (e.g., `frontend/1.0.0`, `backend/2.1.0`)
- **Git Integration**: Automatically handles Git operations (pull, tag, push)
- **Safety Features**: Requires main branch for releases (configurable)
- **Dry Run Mode**: Preview changes before applying them

## Installation

Download the latest build binary or build from source:

```bash
cargo build --release
```

The binary will be available at `target/release/release`.

## Usage

### List Releases

List all releases or filter by scope:

```bash
# List all releases
release list --all-scopes

# List releases for a specific scope
release list frontend

# Show only the latest release
release list --latest

# Show latest release for a specific scope
release list frontend --latest
```

### Bump Version

Create a new release by bumping the version:

```bash
# Bump patch version (1.0.0 -> 1.0.1)
release bump --part patch

# Bump minor version (1.0.0 -> 1.1.0)
release bump --part minor

# Bump major version (1.0.0 -> 2.0.0)
release bump --part major

# Bump version for a specific scope
release bump frontend --part minor

# Push tags to remote after creating
release bump --part patch --push

# Preview changes without applying
release bump --part minor --dry-run

# Allow bumping from non-main branches
release bump --part patch --allow-non-main
```

## Tag Format

The tool supports two tag formats:

- **Unscoped**: `1.0.0`, `2.1.3`
- **Scoped**: `frontend/1.0.0`, `backend/2.1.3`

Scoped tags use the format `scope/version` where scope can be any string identifier.

## Safety Features

- Requires being on main branch (main/master) for releases unless `--allow-non-main` is used
- Performs `git pull --rebase` before creating tags to ensure up-to-date state
- Validates version format before proceeding
- Dry run mode for testing changes

## Examples

```bash
# Create first release
release bump --part minor  # Creates 0.1.0

# Create scoped release
release bump frontend --part major  # Creates frontend/1.0.0

# List all frontend releases
release list frontend

# Get latest release across all scopes
release list --all-scopes --latest

# Safe release with push
release bump --part patch --push --dry-run  # Preview first
release bump --part patch --push            # Then apply
```

## Contributing

This project uses Rust 2024 edition. Ensure you have a recent Rust toolchain installed.

```bash
# Run tests
cargo test

