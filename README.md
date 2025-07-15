# 🛠️ Release CLI

A no-fuss CLI tool for bumping versions and tagging Git releases — with scoped tag support if you need it.

## ✨ What It Does

- Bumps versions using [semver](https://semver.org/)
- Creates Git tags like `1.2.3` or `frontend/1.2.3`
- Handles Git stuff (pull, tag, push)
- Keeps you safe with checks (like making sure you're on `main` or `master`)
- Lets you dry-run before doing anything real

## 🚀 Quick Start

Build it:

```bash
cargo build --release
```

Run it from target/release/release

```bash
# Bump patch version
release bump --part patch

# Bump minor for a specific scope
release bump frontend --part minor

# Push tag to remote
release bump --part patch --push

# Preview without applying
release bump --part major --dry-run

# List all releases
release list --all-scopes

# Get latest release for a scope
release list backend --latest
```

## 🏷️ Tag Format

 - `1.0.0` – unscoped
 - `frontend@2.1.0` – scoped
 - `frontend@2.1.0` – scoped
 - `lib/libname@2.1.0` – scoped

Use scopes to split up releases.

## 💡 Tips

 - By default, releases must happen from `main` or `master`
 - Use --allow-non-main if you really know what you're doing
 - Tool pulls latest changes before tagging

That's it. Just run release bump, push, and you're good.
