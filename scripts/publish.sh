#!/bin/bash

### How to publish the framework
#
# Prerequisites:
# - You need rights to publish on crates.io
# - You need an API access token (you obtain it from https://crates.io/me)
# - You need to call `cargo login <token>` in a console, follow the instructions on crates.io for this.
#
# Steps:
#
# 1. Have a look at commits on GitHub, everything that changed since the last release must be published.
# Be mindful that hotfixes need to be backwards compatible, minor releases do not.
# We always publish all `framework/*` crates together.
# We always publish `klever-codec` and `klever-codec-derive` together.
# `framework/*` depend on both `klever-codec` and `klever-chain-scenario-format`,
# so if you have a minor release on the latter, you also need a minor release on `framework/*`.
# See the Changelog for more details.
#
# 2. Mass replace previous version -> new version.
# Be careful to not accidentally replace some of the other dependencies we have.
#
# 3. Write release name, date and description in `CHANGELOG.md`.
#
# 4. Run `cargo test`, to make sure nothing was broken and all dependencies still work fine.
#
# 5. Commit changes. The name of the commit should be the released crates and versions, same as the changelog title,
# e.g. `sc 0.39.0, codec 0.17.0, chain-vm 0.1.0, chain-scenario-format 0.19.0, sdk 0.1.0`.
# The branch doesn't need to be published for the following steps to work.
#
# 6. Make sure that the contract upgrade tool is still sound.
# At the very least add the new version to `VERSIONS` and change `DEFAULT_LAST_VERSION` in
# `./klever-vm-sdk-rs/framework/meta/src/sc_upgrade/upgrade_versions.rs`
#
# 7. Run this script, `./publish.sh`.
# You can comment out the crates you are not publishing. The script will stop otherwise when it cannot publish them.
#
# 8. Search for `klever` on `crates.io` and check that the new versions appear for all crates.
# If any of the crates was not published, check what went wrong and try again.
#
# 9. Create tag.
# `git tag -s -a vX.X.X -m 'very short description of the release'`
# `git push origin vX.X.X`
#
# 10. Go to https://github.com/klever-io/klever-vm-sdk-rs/tags
# Click on the new tag.
# Click `Create release from tag`.
# The title should be the released crates and versions, same as in the changelog and the commit message.
# The description should be copied from CHANGELOG.md, as is.
#
# 11. Run `sc-meta all update`. This will update the `Cargo.lock` files.
#
# 12. Create pull request on GitHub. The faster it gets merged in master, the better.
#
# 13. (optional) Test the new framework on one of the contracts that are not in the same repo, e.g. DNS, DEX, etc.
#
# 14. Post in Slack to `release-announcements`.
#
# 15. Write a release announcement in Confluence.
#

# Don't use set -e because we use non-zero returns for flow control
set +e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Parse command line arguments
DEBUG=""
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Publishes all packages to crates.io in dependency order."
            echo ""
            echo "Options:"
            echo "  -d, --debug      Enable debug output"
            echo "  -h, --help       Show this help message"
            echo ""
            echo "Environment variables:"
            echo "  CRATES_TOKEN     The crates.io API token (required)"
            echo "  DEBUG            Set to enable debug output"
            echo ""
            echo "Before running this script:"
            echo "  1. Ensure all version numbers are updated"
            echo "  2. Update CHANGELOG.md"
            echo "  3. Commit all changes"
            echo "  4. Run tests with 'cargo test'"
            exit 0
            ;;
        -d|--debug)
            DEBUG=1
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use -h or --help for usage information"
            exit 1
            ;;
    esac
done

# Check for CRATES_TOKEN
if [[ -z "$CRATES_TOKEN" ]]; then
    print_error "CRATES_TOKEN environment variable is not set!"
    print_info "You need to set CRATES_TOKEN to publish to crates.io"
    print_info "Get your token from https://crates.io/me"
    exit 1
fi

# Define packages in dependency order (important!)
# The order matters because some packages depend on others
PACKAGES=(
    # Codec (codec-derive must be published before codec)
    "data/codec-derive"
    "data/codec"

    # Core dependencies (no internal dependencies)
    "vm"
    "sdk/core"
    "sdk/scenario-format"
    
    # Framework (in dependency order)
    "framework/derive"
    "framework/base"
    "framework/meta"
    "framework/scenario"
    "framework/wasm-adapter"
    
    # Modules (depends on framework)
    "contracts/modules"
)

# Function to get package name from Cargo.toml
get_package_name() {
    local cargo_path="$1/Cargo.toml"
    if [[ -f "$cargo_path" ]]; then
        grep "^name" "$cargo_path" | sed -n 's/.*"\([^"]*\)".*/\1/p' | head -1
    else
        echo "unknown"
    fi
}

# Function to get package version from Cargo.toml
get_package_version() {
    local cargo_path="$1/Cargo.toml"
    if [[ -f "$cargo_path" ]]; then
        grep "^version" "$cargo_path" | sed -n 's/.*"\([^"]*\)".*/\1/p' | head -1
    else
        echo "unknown"
    fi
}

# Function to check if a crate version exists on crates.io
check_crate_exists() {
    local package_name=$1
    local package_version=$2
    
    print_info "Checking if $package_name v$package_version exists on crates.io..."
    
    # Query crates.io API with timeout and connection timeout
    local response
    local curl_exit_code
    
    response=$(curl -s --max-time 10 --connect-timeout 5 \
        -H "User-Agent: klever-publish-script" \
        "https://crates.io/api/v1/crates/$package_name/$package_version" 2>&1)
    curl_exit_code=$?
    
    # Debug output if DEBUG is set
    if [[ -n "$DEBUG" ]]; then
        echo "[DEBUG] curl exit code: $curl_exit_code"
        echo "[DEBUG] Response length: ${#response}"
        echo "[DEBUG] Response (first 200 chars): ${response:0:200}"
    fi
    
    # Check if curl failed
    if [[ $curl_exit_code -ne 0 ]]; then
        print_warning "Failed to check crates.io (curl error $curl_exit_code), assuming package doesn't exist"
        return 1
    fi
    
    # Check if version exists (API returns 404 for non-existent versions)
    if echo "$response" | grep -q '"version"'; then
        return 0  # Version exists
    else
        return 1  # Version doesn't exist
    fi
}

# Function to publish a single package
publish_package() {
    local package_path=$1
    local package_name=$(get_package_name "$package_path")
    local package_version=$(get_package_version "$package_path")
    
    if [[ "$package_name" == "unknown" || -z "$package_name" ]]; then
        print_error "Could not find package name at $package_path"
        return 1
    fi
    
    if [[ "$package_version" == "unknown" || -z "$package_version" ]]; then
        print_error "Could not find package version for $package_name at $package_path"
        return 1
    fi
    
    # Check if this version already exists on crates.io
    if check_crate_exists "$package_name" "$package_version"; then
        print_warning "$package_name v$package_version already exists on crates.io, skipping..."
        if [[ -n "$DEBUG" ]]; then
            echo "[DEBUG] About to return 2 from publish_package"
        fi
        return 2  # Return special code for "already exists"
    fi
    
    print_info "Publishing $package_name v$package_version..."
    
    cargo publish -p "$package_name" --token "${CRATES_TOKEN}"
    
    local result=$?
    
    if [[ $result -eq 0 ]]; then
        print_success "$package_name v$package_version published successfully!"
        # Small delay between publishes to ensure crates.io indexes are updated
        print_info "Waiting 10 seconds for crates.io to update..."
        sleep 10
    else
        print_error "Failed to publish $package_name v$package_version"
        return 1
    fi
    
    return 0
}

# Main execution
print_info "Starting publish process..."
echo ""

# Check git status
if ! git diff-index --quiet HEAD --; then
    print_error "There are uncommitted changes in your repository!"
    print_info "Please commit all changes before publishing"
    exit 1
fi

print_info "Publishing packages to crates.io..."
echo ""

# Track success/failure
failed_packages=()
succeeded_packages=()
skipped_packages=()

# Debug: show total packages
if [[ -n "$DEBUG" ]]; then
    echo "[DEBUG] Total packages to process: ${#PACKAGES[@]}"
    echo "[DEBUG] Packages: ${PACKAGES[@]}"
fi

for i in "${!PACKAGES[@]}"; do
    package="${PACKAGES[$i]}"
    
    if [[ -n "$DEBUG" ]]; then
        echo "[DEBUG] Processing package $((i+1))/${#PACKAGES[@]}: $package"
    fi
    
    print_info "Processing package: $package"
    
    if [[ -d "$package" ]]; then
        publish_package "$package"
        status=$?
        
        if [[ -n "$DEBUG" ]]; then
            echo "[DEBUG] publish_package returned status: $status"
        fi
        
        case $status in
            0)  # Successfully published
                succeeded_packages+=("$package")
                if [[ -n "$DEBUG" ]]; then
                    echo "[DEBUG] Added $package to succeeded list"
                fi
                ;;
            2)  # Already exists
                skipped_packages+=("$package")
                if [[ -n "$DEBUG" ]]; then
                    echo "[DEBUG] Added $package to skipped list"
                fi
                ;;
            *)  # Failed
                failed_packages+=("$package")
                print_error "Stopping due to failure. Packages not yet processed:"
                for remaining in "${PACKAGES[@]}"; do
                    if [[ ! " ${succeeded_packages[@]} " =~ " ${remaining} " ]] && 
                       [[ ! " ${failed_packages[@]} " =~ " ${remaining} " ]] && 
                       [[ ! " ${skipped_packages[@]} " =~ " ${remaining} " ]]; then
                        echo "  - $remaining"
                    fi
                done
                exit 1
                ;;
        esac
        
        if [[ -n "$DEBUG" ]]; then
            echo "[DEBUG] Continuing to next package..."
        fi
    else
        print_warning "Skipping non-existent package: $package"
    fi
    echo ""
done

print_info "Main loop completed. Succeeded: ${#succeeded_packages[@]}, Skipped: ${#skipped_packages[@]}, Failed: ${#failed_packages[@]}"

# Summary
echo ""
print_info "============================================"
print_info "Publishing Summary:"
print_info "============================================"

if [[ ${#succeeded_packages[@]} -gt 0 ]]; then
    print_success "Successfully published ${#succeeded_packages[@]} packages:"
    for package in "${succeeded_packages[@]}"; do
        package_name=$(get_package_name "$package")
        package_version=$(get_package_version "$package")
        echo "  ✓ $package_name v$package_version"
    done
fi

if [[ ${#skipped_packages[@]} -gt 0 ]]; then
    echo ""
    print_info "Skipped ${#skipped_packages[@]} packages (already published):"
    for package in "${skipped_packages[@]}"; do
        package_name=$(get_package_name "$package")
        package_version=$(get_package_version "$package")
        echo "  ⏭️  $package_name v$package_version"
    done
fi

if [[ ${#failed_packages[@]} -gt 0 ]]; then
    echo ""
    print_error "Failed to publish ${#failed_packages[@]} packages:"
    for package in "${failed_packages[@]}"; do
        package_name=$(get_package_name "$package")
        echo "  ✗ $package_name"
    done
    exit 1
fi

echo ""

if [[ ${#succeeded_packages[@]} -gt 0 ]]; then
    print_success "Publishing completed successfully! 🎉"
    echo ""
    print_info "Next steps:"
    print_info "1. Create and push git tag: git tag -s -a vX.X.X -m 'Release description'"
    print_info "2. Create GitHub release from the tag"
    print_info "3. Run 'sc-meta all update' to update Cargo.lock files"
    print_info "4. Create PR to merge changes"
elif [[ ${#skipped_packages[@]} -gt 0 && ${#failed_packages[@]} -eq 0 ]]; then
    print_info "All packages were already published to crates.io."
else
    print_error "No packages were published."
fi
