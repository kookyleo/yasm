#!/bin/bash

# YASM Release Script
# This script helps create a new release for the YASM state machine library

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

error() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    error "Cargo.toml not found. Please run this script from the project root."
fi

# Check if git is clean
if [ -n "$(git status --porcelain)" ]; then
    error "Git working directory is not clean. Please commit or stash your changes."
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
info "Current version: $CURRENT_VERSION"

# Ask for new version
echo -n "Enter new version (current: $CURRENT_VERSION): "
read NEW_VERSION

if [ -z "$NEW_VERSION" ]; then
    error "Version cannot be empty"
fi

# Validate version format (basic semver check)
if ! echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$'; then
    error "Invalid version format. Please use semantic versioning (e.g., 1.0.0)"
fi

info "Preparing release for version $NEW_VERSION"

# Update version in Cargo.toml
info "Updating Cargo.toml version..."
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Update Cargo.lock
info "Updating Cargo.lock..."
cargo check

# Run tests
info "Running tests..."
cargo test

# Run clippy
info "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
info "Checking code formatting..."
cargo fmt --all -- --check

# Build documentation
info "Building documentation..."
cargo doc --no-deps --all-features

# Test examples
info "Testing examples..."
find examples -name "*.rs" -exec basename {} .rs \; | xargs -n1 cargo run --example > /dev/null

# Generate documentation
info "Generating project documentation..."
cargo run --example generate_docs

# Build release
info "Building release..."
cargo build --release

# Create git commit
info "Creating git commit..."
git add Cargo.toml Cargo.lock docs/
git commit -m "chore: bump version to $NEW_VERSION"

# Create git tag
info "Creating git tag..."
git tag -a "v$NEW_VERSION" -m "Release version $NEW_VERSION"

# Show what will be pushed
info "The following will be pushed:"
echo "  - Commit: $(git log --oneline -1)"
echo "  - Tag: v$NEW_VERSION"

# Ask for confirmation
echo -n "Push to origin? (y/N): "
read CONFIRM

if [ "$CONFIRM" = "y" ] || [ "$CONFIRM" = "Y" ]; then
    info "Pushing to origin..."
    git push origin main
    git push origin "v$NEW_VERSION"
    
    success "Release $NEW_VERSION has been created and pushed!"
    info "GitHub Actions will now:"
    echo "  1. Run tests and checks"
    echo "  2. Publish to crates.io"
    echo "  3. Create GitHub release"
    echo ""
    info "Monitor the progress at: https://github.com/kookyleo/yasm/actions"
else
    warning "Release created locally but not pushed."
    info "To push manually:"
    echo "  git push origin main"
    echo "  git push origin v$NEW_VERSION"
fi

success "Release script completed!" 