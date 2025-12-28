#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# PHASE 0: CLEANUP AND PREPARE
# ═══════════════════════════════════════════════════════════════════════════════
#
# Part of: claude-flow-hir-clean.yaml
# Purpose: Clean slate for HIR implementation
#
# ═══════════════════════════════════════════════════════════════════════════════

set -e

echo "═══════════════════════════════════════════════════════════════════════════════"
echo "              PHASE 0: CLEANUP AND PREPARE FOR HIR v0.4.0"
echo "═══════════════════════════════════════════════════════════════════════════════"
echo ""

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 1: Ensure we're in the right place
# ─────────────────────────────────────────────────────────────────────────────────
if [ ! -f "Cargo.toml" ] || ! grep -q "metadol\|dol" Cargo.toml; then
    echo "Error: Run from univrs-dol root directory"
    exit 1
fi

echo "Step 1: Checking current state..."
CURRENT_BRANCH=$(git branch --show-current)
echo "  Current branch: $CURRENT_BRANCH"

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 2: Stash any uncommitted changes
# ─────────────────────────────────────────────────────────────────────────────────
echo ""
echo "Step 2: Saving uncommitted changes..."
if [ -n "$(git status --porcelain)" ]; then
    git stash push -m "Pre-HIR-clean-$(date +%Y%m%d-%H%M%S)"
    echo "  ✓ Changes stashed"
else
    echo "  ✓ Working directory clean"
fi

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 3: Switch to main
# ─────────────────────────────────────────────────────────────────────────────────
echo ""
echo "Step 3: Switching to main..."
git checkout main
git pull origin main 2>/dev/null || true
echo "  ✓ On main branch"

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 4: Delete problematic feature branches
# ─────────────────────────────────────────────────────────────────────────────────
echo ""
echo "Step 4: Cleaning up old branches..."
for branch in "feature/HIR-dolv3" "feature/hir-v0.3.0"; do
    if git show-ref --verify --quiet "refs/heads/$branch" 2>/dev/null; then
        git branch -D "$branch"
        echo "  ✓ Deleted $branch"
    else
        echo "  - $branch doesn't exist (ok)"
    fi
done

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 5: Verify main compiles
# ─────────────────────────────────────────────────────────────────────────────────
echo ""
echo "Step 5: Verifying main compiles..."
ERRORS=$(cargo check 2>&1 | grep "^error\[" | wc -l)
if [ "$ERRORS" -gt 0 ]; then
    echo "  ⚠ Main has $ERRORS errors!"
    echo "  These need to be fixed before proceeding."
    echo ""
    cargo check 2>&1 | grep "^error\[" | head -10
    echo ""
    echo "Fix main branch first, then re-run this script."
    exit 1
else
    echo "  ✓ Main compiles cleanly"
fi

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 6: Run tests on main
# ─────────────────────────────────────────────────────────────────────────────────
echo ""
echo "Step 6: Running tests on main..."
if cargo test --lib --quiet 2>/dev/null; then
    echo "  ✓ Tests pass"
else
    echo "  ⚠ Some tests fail on main"
    echo "  Continuing anyway - fix in feature branch"
fi

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 7: Create clean feature branch
# ─────────────────────────────────────────────────────────────────────────────────
echo ""
echo "Step 7: Creating feature branch..."
git checkout -b feature/hir-clean-v0.4.0
echo "  ✓ Created feature/hir-clean-v0.4.0"

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 8: Create directory structure
# ─────────────────────────────────────────────────────────────────────────────────
echo ""
echo "Step 8: Creating directory structure..."
mkdir -p docs/hir
mkdir -p tests/hir
mkdir -p archive/hir-v0.3.0-abandoned
echo "  ✓ Directories created"

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 9: Archive any .disabled files from main
# ─────────────────────────────────────────────────────────────────────────────────
echo ""
echo "Step 9: Archiving disabled files..."
DISABLED_COUNT=0
for f in $(find src -name "*.disabled" 2>/dev/null); do
    mv "$f" archive/hir-v0.3.0-abandoned/
    DISABLED_COUNT=$((DISABLED_COUNT + 1))
done
if [ "$DISABLED_COUNT" -gt 0 ]; then
    echo "  ✓ Archived $DISABLED_COUNT .disabled files"
else
    echo "  ✓ No .disabled files found"
fi

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 10: Copy claude-flow task file
# ─────────────────────────────────────────────────────────────────────────────────
echo ""
echo "Step 10: Setting up claude-flow..."
mkdir -p claude-flow
if [ -f "$HOME/Downloads/claude-flow-hir-clean.yaml" ]; then
    cp "$HOME/Downloads/claude-flow-hir-clean.yaml" claude-flow/
    echo "  ✓ Copied claude-flow task file"
else
    echo "  ⚠ claude-flow-hir-clean.yaml not found in Downloads"
    echo "    Please copy it manually to claude-flow/"
fi

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 11: Initial commit
# ─────────────────────────────────────────────────────────────────────────────────
echo ""
echo "Step 11: Creating initial commit..."
git add .
git commit -m "chore: Phase 0 - clean slate for HIR v0.4.0

- Create feature/hir-clean-v0.4.0 branch
- Archive old disabled files
- Set up directory structure
- Add claude-flow task definition

Part of: claude-flow-hir-clean.yaml
Phase: 0 - Cleanup and Prepare"

# ─────────────────────────────────────────────────────────────────────────────────
# SUMMARY
# ─────────────────────────────────────────────────────────────────────────────────
echo ""
echo "═══════════════════════════════════════════════════════════════════════════════"
echo "                          PHASE 0 COMPLETE"
echo "═══════════════════════════════════════════════════════════════════════════════"
echo ""
echo "Branch: feature/hir-clean-v0.4.0"
echo "Status: Ready for Phase 1 (HIR Design Document)"
echo ""
echo "Directory structure:"
echo "  docs/hir/           ← Phase 1: HIR-SPECIFICATION.md goes here"
echo "  tests/hir/          ← Phase 2+: HIR tests go here"
echo "  claude-flow/        ← Task tracking"
echo "  archive/            ← Old code preserved"
echo ""
echo "Next steps:"
echo "  1. Review claude-flow/claude-flow-hir-clean.yaml"
echo "  2. Start Phase 1: Design HIR specification document"
echo "  3. Get design approved before writing any code"
echo ""
echo "To continue: Start new Claude conversation with Phase 1 context"
echo ""
