# DOL Release Workflow

Automate DOL version releases with documentation updates across all three repositories.

## Usage

Run this skill after every major DOL 2.0 update with substantial feature updates or bug fixes.

```
/releases/dol-release <version> <release-name>
```

**Example:**
```
/releases/dol-release v0.2.1 "Macro Improvements"
```

## Parameters

- `$1` - Version number (e.g., `v0.2.1`, `v0.3.0`)
- `$2` - Release name/codename (e.g., `"Macro Improvements"`, `"Self-Hosting"`)

## Step-by-Step Process

### Phase 1: Gather Release Information

1. **Run Tests and Verify Build**
   ```bash
   cd ~/repos/univrs-dol
   cargo test --all
   cargo clippy -- -D warnings
   ```

2. **Count Tests by Category**
   ```bash
   cargo test 2>&1 | grep -E "test result|tests"
   ```

3. **Get Recent Commits Since Last Tag**
   ```bash
   git log $(git describe --tags --abbrev=0)..HEAD --oneline
   ```

4. **Check Current Version in Cargo.toml**
   ```bash
   grep "^version" Cargo.toml
   ```

### Phase 2: Update Main Repository (univrs-dol)

1. **Update Cargo.toml Version**
   - Edit `Cargo.toml` to bump version number

2. **Update README.md**
   - Update version badge
   - Update feature list if new features added
   - Update test count in README

3. **Create/Update CHANGELOG.md Entry**
   - Add new version section
   - List features, fixes, and breaking changes

4. **Commit Changes**
   ```bash
   cd ~/repos/univrs-dol
   git add -A
   git commit -m "Release $1 \"$2\""
   ```

5. **Create Git Tag**
   ```bash
   git tag -a $1 -m "DOL $1 - $2"
   ```

6. **Push Tag to GitHub**
   ```bash
   git push origin main
   git push origin $1
   ```

### Phase 3: Update Book Wiki (univrs-docs)

1. **Navigate to Wiki Repository**
   ```bash
   cd ~/repos/univrs-docs
   git pull origin main
   ```

2. **Update README.md Roadmap**
   - Update DOL version status in roadmap section
   - Mark completed features as `[x]`

3. **Update SUMMARY.md**
   - Add links to new documentation if applicable
   - Update version references

4. **Update vudo/dol2.0_roadmap_checkpoint.md**
   - Update document version
   - Update test counts
   - Mark completed roadmap items

5. **Create Release Notes (if not exists)**
   ```bash
   # Create releases/v$VERSION.md with release notes
   ```

6. **Commit and Push**
   ```bash
   git add -A
   git commit -m "docs: Update for DOL $1 release"
   git push origin main
   ```

### Phase 4: Update Marketing Site (learn.univrs.io)

1. **Navigate to Marketing Repository**
   ```bash
   cd ~/repos/learn.univrs.io
   git pull origin main
   ```

2. **Update DOL Reference Page**
   - File: `src/pages/dol/Reference.tsx`
   - Update version display
   - Add new feature documentation

3. **Update Any Version Badges**
   - Search for version strings and update

4. **Commit and Push**
   ```bash
   git add -A
   git commit -m "Update DOL to $1"
   git push origin main
   ```

### Phase 5: Create GitHub Release

1. **Create GitHub Release**
   ```bash
   cd ~/repos/univrs-dol
   gh release create $1 --title "DOL $1 - $2" --notes-file RELEASE_NOTES.md
   ```

   Or with inline notes:
   ```bash
   gh release create $1 --title "DOL $1 - $2" --generate-notes
   ```

## What Claude Code Does

1. Uses **Read** tool to examine current versions and documentation
2. Uses **Edit** tool to update version numbers and documentation
3. Uses **Write** tool to create new release notes
4. Uses **Bash** tool for git operations and testing
5. Uses **TodoWrite** tool to track progress through each phase

## Checklist

Before running this skill, ensure:

- [ ] All tests pass (`cargo test --all`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Changes are committed to main branch
- [ ] You have push access to all three repositories
- [ ] GitHub CLI (`gh`) is authenticated

## Repository Paths

| Repository | Path | Purpose |
|------------|------|---------|
| univrs-dol | `~/repos/univrs-dol` | Main DOL compiler |
| univrs-docs | `~/repos/univrs-docs` | Book/Wiki documentation |
| learn.univrs.io | `~/repos/learn.univrs.io` | Marketing website |

## Files Typically Updated

### univrs-dol
- `Cargo.toml` - version number
- `README.md` - badges, feature list
- `CHANGELOG.md` - release notes

### univrs-docs
- `README.md` - roadmap status
- `SUMMARY.md` - documentation links
- `vudo/dol2.0_roadmap_checkpoint.md` - test counts, status
- `releases/vX.Y.Z.md` - release notes

### learn.univrs.io
- `src/pages/dol/Reference.tsx` - version display
- Any version badge components

## Example Invocation

```
User: /releases/dol-release v0.2.1 "Macro Improvements"

Claude Code will:
1. Run cargo test to verify build
2. Update Cargo.toml version to 0.2.1
3. Update README.md with new version
4. Create CHANGELOG entry
5. Commit with message "Release v0.2.1 'Macro Improvements'"
6. Create and push git tag v0.2.1
7. Update univrs-docs wiki files
8. Update learn.univrs.io marketing site
9. Create GitHub release with auto-generated notes
```
