# Releases Commands

Commands for managing DOL releases and version documentation.

## Available Commands

| Command | Description |
|---------|-------------|
| `/releases/dol-release` | Full release workflow for DOL versions |

## Usage

### DOL Release

Run after major DOL updates with substantial feature additions or bug fixes:

```
/releases/dol-release v0.2.1 "Macro Improvements"
```

This command automates:
1. Building and testing the compiler
2. Updating version in `Cargo.toml`
3. Creating git tags
4. Updating documentation in `univrs-docs`
5. Updating marketing site at `learn.univrs.io`
6. Creating GitHub releases

## Prerequisites

- GitHub CLI (`gh`) authenticated
- Push access to all three repositories:
  - `~/repos/univrs-dol`
  - `~/repos/univrs-docs`
  - `~/repos/learn.univrs.io`

## Repository Map

```
Repositories Updated:
├── univrs-dol (main compiler)
│   ├── Cargo.toml
│   ├── README.md
│   └── CHANGELOG.md
├── univrs-docs (book/wiki)
│   ├── README.md
│   ├── SUMMARY.md
│   └── vudo/dol2.0_roadmap_checkpoint.md
└── learn.univrs.io (marketing)
    └── src/pages/dol/Reference.tsx
```
