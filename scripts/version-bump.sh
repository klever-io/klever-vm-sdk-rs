#!/bin/bash

# Version Bump Helper Script for klever-vm-sdk-rs
# This script automates the version bumping process including:
# - Finding and replacing version strings
# - Generating changelog from git commits
# - Updating version files
# - Running tests
# - Creating commit

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script configuration
DRY_RUN=false
VERBOSE=false
SKIP_TESTS=false
SKIP_CHANGELOG=false
SKIP_UPDATE=false
SKIP_BUILD=false
AUTO_DETECT=false

# Version patterns
SC_VERSION=""
VM_VERSION=""
CODEC_VERSION=""
SDK_VERSION=""
SCENARIO_FORMAT_VERSION=""

# Current versions (will be detected)
CURRENT_SC_VERSION=""
CURRENT_VM_VERSION=""
CURRENT_CODEC_VERSION=""
CURRENT_SDK_VERSION=""
CURRENT_SCENARIO_FORMAT_VERSION=""

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Function to show usage
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Version bump helper for klever-vm-sdk-rs repository.

OPTIONS:
    -s, --sc-version VERSION         New version for klever-sc family (e.g., 0.45.0)
    -v, --vm-version VERSION         New version for klever-chain-vm (e.g., 0.6.0)
    -c, --codec-version VERSION      New version for klever-sc-codec family (e.g., 0.19.0)
    --sdk-version VERSION            New version for klever-vm-sdk (e.g., 0.2.0)
    --scenario-version VERSION       New version for klever-chain-scenario-format (e.g., 0.20.0)
    -a, --auto-detect                Auto-detect which modules need version bumps based on changes
    -d, --dry-run                    Show what would be changed without making changes
    --skip-tests                     Skip running tests after version bump
    --skip-changelog                 Skip changelog generation
    --skip-update                    Skip running meta-update-all.sh
    --skip-build                     Skip running meta-build-wasm.sh
    --verbose                        Show detailed output
    -h, --help                       Show this help message

EXAMPLES:
    # Auto-detect which modules need bumping (shows recommendations)
    $0 --auto-detect

    # Bump all versions
    $0 -s 0.45.0 -v 0.6.0 -c 0.19.0 --sdk-version 0.2.0

    # Bump only sc version (dry run)
    $0 -s 0.45.0 --dry-run

    # Bump sc and codec versions
    $0 -s 0.45.0 -c 0.19.0
EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -s|--sc-version)
            SC_VERSION="$2"
            shift 2
            ;;
        -v|--vm-version)
            VM_VERSION="$2"
            shift 2
            ;;
        -c|--codec-version)
            CODEC_VERSION="$2"
            shift 2
            ;;
        --sdk-version)
            SDK_VERSION="$2"
            shift 2
            ;;
        --scenario-version)
            SCENARIO_FORMAT_VERSION="$2"
            shift 2
            ;;
        -d|--dry-run)
            DRY_RUN=true
            shift
            ;;
        --skip-tests)
            SKIP_TESTS=true
            shift
            ;;
        --skip-changelog)
            SKIP_CHANGELOG=true
            shift
            ;;
        --skip-update)
            SKIP_UPDATE=true
            shift
            ;;
        --skip-build)
            SKIP_BUILD=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        -a|--auto-detect)
            AUTO_DETECT=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            ;;
    esac
done

# Function to validate version format
validate_version() {
    local version=$1
    if ! [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        print_error "Invalid version format: $version. Expected format: X.Y.Z"
    fi
}

# Function to get the last release tag
get_last_release_tag() {
    local last_tag=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
    if [[ -z "$last_tag" ]]; then
        # If no tags, use the initial commit
        last_tag=$(git rev-list --max-parents=0 HEAD)
    fi
    echo "$last_tag"
}

# Function to auto-detect which modules need version bumps
auto_detect_version_bumps() {
    local last_tag=$(get_last_release_tag)
    print_info "Auto-detecting version bumps needed since $last_tag..."
    
    # Get list of changed files since last release
    local changed_files=$(git diff --name-only "$last_tag"..HEAD 2>/dev/null || git diff --name-only HEAD)
    
    # Flags for which modules need bumping
    local need_sc=false
    local need_codec=false
    local need_vm=false
    local need_sdk=false
    local need_scenario=false
    
    # Track changed files per module
    local sc_files=""
    local codec_files=""
    local vm_files=""
    local sdk_files=""
    local scenario_files=""
    
    # Check each changed file
    while IFS= read -r file; do
        # Skip if file doesn't exist or is in ignored paths
        [[ ! -f "$file" ]] && continue
        [[ "$file" =~ ^target/ ]] && continue
        [[ "$file" =~ ^contracts/examples/.*/interact-rs/ ]] && continue
        
        # Check for klever-sc framework changes
        if [[ "$file" =~ ^framework/(base|derive|meta|scenario|wasm-adapter)/ ]] || \
           [[ "$file" =~ ^contracts/modules/ ]]; then
            need_sc=true
            sc_files="${sc_files}\n    - $file"
        fi
        
        # Check for codec changes
        if [[ "$file" =~ ^data/codec ]]; then
            need_codec=true
            codec_files="${codec_files}\n    - $file"
            # Codec changes require sc bump due to dependency
            need_sc=true
        fi
        
        # Check for VM changes
        if [[ "$file" =~ ^vm/ ]]; then
            need_vm=true
            vm_files="${vm_files}\n    - $file"
            # VM changes might require sc bump
            need_sc=true
        fi
        
        # Check for SDK changes
        if [[ "$file" =~ ^sdk/core/ ]]; then
            need_sdk=true
            sdk_files="${sdk_files}\n    - $file"
        fi
        
        # Check for scenario format changes
        if [[ "$file" =~ ^sdk/scenario-format/ ]]; then
            need_scenario=true
            scenario_files="${scenario_files}\n    - $file"
            # Scenario format changes require sc bump due to dependency
            need_sc=true
        fi
    done <<< "$changed_files"
    
    # Show recommendations
    echo ""
    print_info "=== Version Bump Recommendations ==="
    echo ""
    
    if [[ "$need_sc" == true ]]; then
        print_warning "klever-sc family needs version bump (current: $CURRENT_SC_VERSION)"
        echo "  Affected paths: framework/*, contracts/modules/"
        echo "  Dependencies: Also affected by codec, vm, or scenario-format changes"
        if [[ "$VERBOSE" == true && -n "$sc_files" ]]; then
            echo -e "  Changed files:$sc_files"
        fi
    fi
    
    if [[ "$need_codec" == true ]]; then
        print_warning "klever-sc-codec family needs version bump (current: $CURRENT_CODEC_VERSION)"
        echo "  Affected paths: data/codec*"
        if [[ "$VERBOSE" == true && -n "$codec_files" ]]; then
            echo -e "  Changed files:$codec_files"
        fi
    fi
    
    if [[ "$need_vm" == true ]]; then
        print_warning "klever-chain-vm needs version bump (current: $CURRENT_VM_VERSION)"
        echo "  Affected paths: vm/"
        if [[ "$VERBOSE" == true && -n "$vm_files" ]]; then
            echo -e "  Changed files:$vm_files"
        fi
    fi
    
    if [[ "$need_sdk" == true ]]; then
        print_warning "klever-vm-sdk needs version bump (current: $CURRENT_SDK_VERSION)"
        echo "  Affected paths: sdk/core/"
        if [[ "$VERBOSE" == true && -n "$sdk_files" ]]; then
            echo -e "  Changed files:$sdk_files"
        fi
    fi
    
    if [[ "$need_scenario" == true ]]; then
        print_warning "klever-chain-scenario-format needs version bump (current: $CURRENT_SCENARIO_FORMAT_VERSION)"
        echo "  Affected paths: sdk/scenario-format/"
        if [[ "$VERBOSE" == true && -n "$scenario_files" ]]; then
            echo -e "  Changed files:$scenario_files"
        fi
    fi
    
    if [[ "$need_sc" == false && "$need_codec" == false && "$need_vm" == false && \
          "$need_sdk" == false && "$need_scenario" == false ]]; then
        print_success "No version bumps needed - no changes in versioned modules"
    else
        echo ""
        print_info "Based on dependency rules from publish.sh:"
        print_info "- framework/* crates are always published together"
        print_info "- klever-codec and klever-codec-derive are always published together"
        print_info "- framework/* depends on codec and scenario-format, so their changes require framework bump"
        echo ""
        print_info "Suggested command:"
        echo -n "  $0"
        [[ "$need_sc" == true ]] && echo -n " -s <new-sc-version>"
        [[ "$need_codec" == true ]] && echo -n " -c <new-codec-version>"
        [[ "$need_vm" == true ]] && echo -n " -v <new-vm-version>"
        [[ "$need_sdk" == true ]] && echo -n " --sdk-version <new-sdk-version>"
        [[ "$need_scenario" == true ]] && echo -n " --scenario-version <new-scenario-version>"
        echo ""
    fi
    
    echo ""
    
    # Generate detailed changelog and release recommendation
    if [[ "$need_sc" == true || "$need_codec" == true || "$need_vm" == true || \
          "$need_sdk" == true || "$need_scenario" == true ]]; then
        print_info "=== Changelog Analysis ==="
        echo ""
        
        # Analyze commits
        local features=$(git log --pretty=format:"%h %s" "$last_tag"..HEAD | grep -E "^[a-f0-9]+ feat" || echo "")
        local fixes=$(git log --pretty=format:"%h %s" "$last_tag"..HEAD | grep -E "^[a-f0-9]+ fix|^[a-f0-9]+ Fix" || echo "")
        local breaking=$(git log --pretty=format:"%h %s" "$last_tag"..HEAD | grep -E "BREAKING|breaking" || echo "")
        
        # Count changes
        local feature_count=0
        local fix_count=0
        local breaking_count=0
        
        if [[ -n "$features" ]]; then
            feature_count=$(echo "$features" | wc -l | tr -d ' ')
        fi
        if [[ -n "$fixes" ]]; then
            fix_count=$(echo "$fixes" | wc -l | tr -d ' ')
        fi
        if [[ -n "$breaking" ]]; then
            breaking_count=$(echo "$breaking" | wc -l | tr -d ' ')
        fi
        
        # Show commit categories
        if [[ -n "$features" ]]; then
            print_success "Features ($feature_count):"
            echo "$features" | while IFS= read -r line; do
                echo "  - $line"
            done
            echo ""
        fi
        
        if [[ -n "$fixes" ]]; then
            print_success "Bug Fixes ($fix_count):"
            echo "$fixes" | while IFS= read -r line; do
                echo "  - $line"
            done
            echo ""
        fi
        
        if [[ -n "$breaking" ]]; then
            print_warning "BREAKING CHANGES ($breaking_count):"
            echo "$breaking" | while IFS= read -r line; do
                echo "  - $line"
            done
            echo ""
        fi
        
        # Show detailed changes if verbose
        if [[ "$VERBOSE" == true ]]; then
            print_info "=== Detailed Commit Log ==="
            echo ""
            # Use --no-pager to ensure all commits are shown
            git --no-pager log --pretty=format:"%h %ad %s - %an" --date=short "$last_tag"..HEAD
            echo ""
            echo ""
            
            # Show total commit count
            local total_commits=$(git rev-list --count "$last_tag"..HEAD)
            print_info "Total commits since $last_tag: $total_commits"
            echo ""
        fi
        
        # Show files changed summary
        print_info "=== Files Changed Summary ==="
        echo ""
        
        if [[ "$need_sc" == true ]]; then
            echo "klever-sc framework ($CURRENT_SC_VERSION):"
            local sc_file_count=$(echo -e "$sc_files" | grep -v "^$" | wc -l | tr -d ' ')
            echo "  $sc_file_count files changed"
            if [[ "$VERBOSE" == true ]]; then
                echo -e "$sc_files" | grep -v "^$" | sort | uniq
            fi
            echo ""
        fi
        
        if [[ "$need_codec" == true ]]; then
            echo "klever-sc-codec ($CURRENT_CODEC_VERSION):"
            local codec_file_count=$(echo -e "$codec_files" | grep -v "^$" | wc -l | tr -d ' ')
            echo "  $codec_file_count files changed"
            if [[ "$VERBOSE" == true ]]; then
                echo -e "$codec_files" | grep -v "^$" | sort | uniq
            fi
            echo ""
        fi
        
        if [[ "$need_vm" == true ]]; then
            echo "klever-chain-vm ($CURRENT_VM_VERSION):"
            local vm_file_count=$(echo -e "$vm_files" | grep -v "^$" | wc -l | tr -d ' ')
            echo "  $vm_file_count files changed"
            if [[ "$VERBOSE" == true ]]; then
                echo -e "$vm_files" | grep -v "^$" | sort | uniq
            fi
            echo ""
        fi
        
        if [[ "$need_sdk" == true ]]; then
            echo "klever-vm-sdk ($CURRENT_SDK_VERSION):"
            local sdk_file_count=$(echo -e "$sdk_files" | grep -v "^$" | wc -l | tr -d ' ')
            echo "  $sdk_file_count files changed"
            if [[ "$VERBOSE" == true ]]; then
                echo -e "$sdk_files" | grep -v "^$" | sort | uniq
            fi
            echo ""
        fi
        
        if [[ "$need_scenario" == true ]]; then
            echo "klever-chain-scenario-format ($CURRENT_SCENARIO_FORMAT_VERSION):"
            local scenario_file_count=$(echo -e "$scenario_files" | grep -v "^$" | wc -l | tr -d ' ')
            echo "  $scenario_file_count files changed"
            if [[ "$VERBOSE" == true ]]; then
                echo -e "$scenario_files" | grep -v "^$" | sort | uniq
            fi
            echo ""
        fi
        
        # Release type recommendation
        print_info "=== Release Type Recommendation ==="
        echo ""
        
        local release_type="PATCH"
        local reason=""
        
        if [[ "$breaking_count" -gt 0 ]]; then
            release_type="MAJOR"
            reason="Breaking changes detected"
        elif [[ "$feature_count" -gt 0 ]]; then
            release_type="MINOR"
            reason="New features added (backwards compatible)"
        else
            release_type="PATCH"
            reason="Bug fixes and minor improvements only"
        fi
        
        print_success "Recommended Release Type: $release_type"
        echo "Reason: $reason"
        echo ""
        
        # Version suggestions
        print_info "Suggested versions based on $release_type release:"
        
        if [[ "$need_sc" == true ]]; then
            local new_sc=$(suggest_version "$CURRENT_SC_VERSION" "$release_type")
            echo "  klever-sc: $CURRENT_SC_VERSION → $new_sc"
        fi
        
        if [[ "$need_codec" == true ]]; then
            local new_codec=$(suggest_version "$CURRENT_CODEC_VERSION" "$release_type")
            echo "  klever-sc-codec: $CURRENT_CODEC_VERSION → $new_codec"
        fi
        
        if [[ "$need_vm" == true ]]; then
            local new_vm=$(suggest_version "$CURRENT_VM_VERSION" "$release_type")
            echo "  klever-chain-vm: $CURRENT_VM_VERSION → $new_vm"
        fi
        
        if [[ "$need_sdk" == true ]]; then
            local new_sdk=$(suggest_version "$CURRENT_SDK_VERSION" "$release_type")
            echo "  klever-vm-sdk: $CURRENT_SDK_VERSION → $new_sdk"
        fi
        
        if [[ "$need_scenario" == true ]]; then
            local new_scenario=$(suggest_version "$CURRENT_SCENARIO_FORMAT_VERSION" "$release_type")
            echo "  klever-chain-scenario-format: $CURRENT_SCENARIO_FORMAT_VERSION → $new_scenario"
        fi
        
        echo ""
        print_info "Example command with suggested versions:"
        echo -n "  $0"
        [[ "$need_sc" == true ]] && echo -n " -s $new_sc"
        [[ "$need_codec" == true ]] && echo -n " -c $new_codec"
        [[ "$need_vm" == true ]] && echo -n " -v $new_vm"
        [[ "$need_sdk" == true ]] && echo -n " --sdk-version $new_sdk"
        [[ "$need_scenario" == true ]] && echo -n " --scenario-version $new_scenario"
        echo ""
        
        if [[ "$VERBOSE" == false ]]; then
            echo ""
            print_info "Tip: Use --auto-detect --verbose to see:"
            echo "  - Full commit history with authors and dates"
            echo "  - Complete list of changed files per module"
            echo "  - Detailed analysis of all changes"
        fi
    fi
    
    echo ""
}

# Function to suggest next version based on release type
suggest_version() {
    local current=$1
    local release_type=$2
    
    # Parse current version
    local major=$(echo "$current" | cut -d. -f1)
    local minor=$(echo "$current" | cut -d. -f2)
    local patch=$(echo "$current" | cut -d. -f3)
    
    case "$release_type" in
        MAJOR)
            echo "$((major + 1)).0.0"
            ;;
        MINOR)
            echo "$major.$((minor + 1)).0"
            ;;
        PATCH)
            echo "$major.$minor.$((patch + 1))"
            ;;
    esac
}

# Function to detect current version from Cargo.toml
detect_current_version() {
    local cargo_path=$1
    local version=$(grep "^version = " "$cargo_path" | head -1 | sed 's/version = "\(.*\)"/\1/')
    echo "$version"
}

# Detect current versions
print_info "Detecting current versions..."
CURRENT_SC_VERSION=$(detect_current_version "framework/base/Cargo.toml")
CURRENT_VM_VERSION=$(detect_current_version "vm/Cargo.toml")
CURRENT_CODEC_VERSION=$(detect_current_version "data/codec/Cargo.toml")
CURRENT_SDK_VERSION=$(detect_current_version "sdk/core/Cargo.toml")
CURRENT_SCENARIO_FORMAT_VERSION=$(detect_current_version "sdk/scenario-format/Cargo.toml")

print_info "Current versions:"
print_info "  klever-sc: $CURRENT_SC_VERSION"
print_info "  klever-chain-vm: $CURRENT_VM_VERSION"
print_info "  klever-sc-codec: $CURRENT_CODEC_VERSION"
print_info "  klever-vm-sdk: $CURRENT_SDK_VERSION"
print_info "  klever-chain-scenario-format: $CURRENT_SCENARIO_FORMAT_VERSION"

# If auto-detect mode, show recommendations and exit
if [[ "$AUTO_DETECT" == true ]]; then
    auto_detect_version_bumps
    exit 0
fi

# Check if at least one version was specified
if [[ -z "$SC_VERSION" && -z "$VM_VERSION" && -z "$CODEC_VERSION" && -z "$SDK_VERSION" && -z "$SCENARIO_FORMAT_VERSION" ]]; then
    print_error "At least one version must be specified. Use --auto-detect to see recommendations."
fi

# Validate all provided versions
[[ -n "$SC_VERSION" ]] && validate_version "$SC_VERSION"
[[ -n "$VM_VERSION" ]] && validate_version "$VM_VERSION"
[[ -n "$CODEC_VERSION" ]] && validate_version "$CODEC_VERSION"
[[ -n "$SDK_VERSION" ]] && validate_version "$SDK_VERSION"
[[ -n "$SCENARIO_FORMAT_VERSION" ]] && validate_version "$SCENARIO_FORMAT_VERSION"

# Function to generate changelog entries from git commits
generate_changelog() {
    local from_ref=$1
    local to_ref=${2:-HEAD}
    
    # Get all commits between versions
    local commits=$(git log --pretty=format:"%h %s" "$from_ref".."$to_ref")
    
    # Categorize commits
    local features=""
    local fixes=""
    local breaking=""
    local other=""
    
    while IFS= read -r commit; do
        if [[ "$commit" =~ ^[a-f0-9]+\ feat ]]; then
            features="${features}\n- ${commit#* }"
        elif [[ "$commit" =~ ^[a-f0-9]+\ fix ]]; then
            fixes="${fixes}\n- ${commit#* }"
        elif [[ "$commit" =~ ^[a-f0-9]+\ breaking|BREAKING ]]; then
            breaking="${breaking}\n- ${commit#* }"
        else
            other="${other}\n- ${commit#* }"
        fi
    done <<< "$commits"
    
    # Build changelog entry
    local changelog_entry=""
    
    if [[ -n "$breaking" ]]; then
        changelog_entry="${changelog_entry}\n### Breaking Changes"
        changelog_entry="${changelog_entry}${breaking}\n"
    fi
    
    if [[ -n "$features" ]]; then
        changelog_entry="${changelog_entry}\n### Features"
        changelog_entry="${changelog_entry}${features}\n"
    fi
    
    if [[ -n "$fixes" ]]; then
        changelog_entry="${changelog_entry}\n### Fixes"
        changelog_entry="${changelog_entry}${fixes}\n"
    fi
    
    if [[ -n "$other" ]]; then
        changelog_entry="${changelog_entry}\n### Other Changes"
        changelog_entry="${changelog_entry}${other}\n"
    fi
    
    echo -e "$changelog_entry"
}

# Function to update version in a single Cargo.toml file
update_cargo_version() {
    local file=$1
    local old_version=$2
    local new_version=$3
    
    if [[ "$DRY_RUN" == true ]]; then
        print_info "[DRY RUN] Would update $file: $old_version -> $new_version"
        return 0
    else
        if [[ ! -f "$file" ]]; then
            print_error "File not found: $file"
            return 1
        fi
        
        # Check if the version has already been updated
        if grep -q "^version = \"$new_version\"" "$file"; then
            [[ "$VERBOSE" == true ]] && print_info "$file already at version $new_version"
            return 0
        fi
        
        # Use a more robust sed command
        sed -i.bak "s/^version = \"$old_version\"/version = \"$new_version\"/" "$file"
        local sed_exit_code=$?
        
        # Verify the change was made
        if [[ $sed_exit_code -eq 0 ]] && grep -q "^version = \"$new_version\"" "$file"; then
            rm -f "$file.bak"
            [[ "$VERBOSE" == true ]] && print_info "Updated $file"
            return 0
        else
            print_error "Failed to update $file (sed exit code: $sed_exit_code)"
            # Restore backup if it exists
            [[ -f "$file.bak" ]] && mv "$file.bak" "$file"
            return 1
        fi
    fi
}

# Function to update dependency version in Cargo.toml files
update_dependency_version() {
    local dep_name=$1
    local old_version=$2
    local new_version=$3
    
    # Find all Cargo.toml files
    local cargo_files=$(find . -name "Cargo.toml" -not -path "./target/*" -not -path "./contracts/examples/*/interact-rs/*")
    
    for file in $cargo_files; do
        # Check if file contains the dependency
        if grep -q "$dep_name" "$file"; then
            # Check if it actually has the old version
            local has_old_version=false
            
            # Check TOML section format
            if grep -A5 "^\[.*dependencies\.$dep_name\]" "$file" | grep -q "version.*$old_version"; then
                has_old_version=true
            fi
            
            # Check inline format
            if grep -q "$dep_name.*$old_version" "$file"; then
                has_old_version=true
            fi
            
            if [[ "$has_old_version" == true ]]; then
                if [[ "$DRY_RUN" == true ]]; then
                    print_info "[DRY RUN] Would update $dep_name in $file: $old_version -> $new_version"
                else
                    # Create a temporary file for the updated content
                    local temp_file="${file}.tmp"
                    local in_dep_section=false
                    
                    # Process the file line by line (handle files without trailing newline)
                    while IFS= read -r line || [[ -n "$line" ]]; do
                        # Check if entering dependency section
                        if [[ "$line" =~ ^\[.*dependencies\.$dep_name\] ]]; then
                            in_dep_section=true
                            echo "$line"
                        # Check if leaving any section
                        elif [[ "$in_dep_section" == true ]] && [[ "$line" =~ ^\[ ]]; then
                            in_dep_section=false
                            echo "$line"
                        # Update version line in dependency section
                        elif [[ "$in_dep_section" == true ]] && [[ "$line" =~ ^version ]]; then
                            echo "$line" | sed "s/$old_version/$new_version/g"
                        # Handle inline dependencies
                        elif [[ "$line" =~ $dep_name ]] && [[ "$line" =~ $old_version ]]; then
                            # Update inline format with optional = prefix
                            echo "$line" | sed "s/\(\"=\?\)$old_version\"/\1$new_version\"/g"
                        else
                            echo "$line"
                        fi
                    done < "$file" > "$temp_file"
                    
                    # Replace the original file
                    mv "$temp_file" "$file"
                    [[ "$VERBOSE" == true ]] && print_info "Updated $dep_name in $file"
                fi
            fi
        fi
    done
}

# Function to update version_history.rs
update_version_history() {
    local version=$1
    local version_type=$2
    local file="framework/meta/src/version_history.rs"
    
    if [[ "$version_type" == "sc" ]]; then
        if [[ "$DRY_RUN" == true ]]; then
            print_info "[DRY RUN] Would update LAST_VERSION in $file to $version"
            print_info "[DRY RUN] Would add $version to VERSIONS array"
        else
            # Update LAST_VERSION only (more specific pattern to avoid updating other framework_version! calls)
            sed -i.bak "/^pub const LAST_VERSION: FrameworkVersion = framework_version!/s/framework_version!([0-9.]*)/framework_version!($version)/" "$file"
            
            # Add new version to VERSIONS array (before the closing bracket)
            sed -i.bak "/^pub const VERSIONS: &\[FrameworkVersion\] = framework_versions!\[/,/\];/ {
                /\];/i\\
    $version,
            }" "$file"
            
            rm -f "$file.bak"
            print_info "Updated version_history.rs (LAST_VERSION and VERSIONS array only)"
        fi
    fi
}

# Function to update CHANGELOG.md
update_changelog() {
    local date=$(date +"%Y-%m-%d")
    local changelog_file="CHANGELOG.md"
    local temp_file="CHANGELOG.tmp"
    
    # Build version string for changelog
    local version_parts=()
    [[ -n "$SC_VERSION" ]] && version_parts+=("sc $SC_VERSION")
    [[ -n "$CODEC_VERSION" ]] && version_parts+=("codec $CODEC_VERSION")
    [[ -n "$VM_VERSION" ]] && version_parts+=("vm $VM_VERSION")
    [[ -n "$SCENARIO_FORMAT_VERSION" ]] && version_parts+=("scenario-format $SCENARIO_FORMAT_VERSION")
    [[ -n "$SDK_VERSION" ]] && version_parts+=("sdk $SDK_VERSION")
    
    local version_string=$(IFS=", "; echo "${version_parts[*]}")
    
    # Get last release tag
    local last_tag=$(get_last_release_tag)
    
    # Generate changelog content
    local changelog_content=""
    if [[ "$SKIP_CHANGELOG" == false ]]; then
        # Capture changelog without the INFO line
        changelog_content=$(generate_changelog "$last_tag" 2>/dev/null | grep -v "^\[0;34m\[INFO\]")
    fi
    
    if [[ "$DRY_RUN" == true ]]; then
        print_info "[DRY RUN] Would add to CHANGELOG.md:"
        echo -e "\n## [$version_string] - $date"
        echo -e "$changelog_content"
    else
        # Read the file and find where to insert the new entry
        local in_header=true
        local header_section=""
        local rest_section=""
        local found_insertion_point=false
        
        while IFS= read -r line; do
            if [[ "$line" =~ ^##[[:space:]]\[ ]] && [[ "$in_header" == true ]]; then
                # Found the first version entry, this is our insertion point
                in_header=false
                found_insertion_point=true
            fi
            
            if [[ "$in_header" == true ]]; then
                header_section="${header_section}${line}\n"
            else
                rest_section="${rest_section}${line}\n"
            fi
        done < "$changelog_file"
        
        # Create the new file with proper structure
        {
            # Write the header section (title and versioning info)
            echo -ne "$header_section"
            echo ""
            # Write the new changelog entry
            echo -e "## [$version_string] - $date"
            echo -e "$changelog_content"
            echo ""
            # Write the rest of the file
            echo -ne "$rest_section"
        } > "$temp_file"
        
        mv "$temp_file" "$changelog_file"
        print_info "Updated CHANGELOG.md"
    fi
}

# Main version bump process
print_info "Starting version bump process..."

# SC Version Bump
if [[ -n "$SC_VERSION" ]]; then
    if [[ "$CURRENT_SC_VERSION" == "$SC_VERSION" ]]; then
        print_warning "klever-sc is already at version $SC_VERSION, skipping..."
    else
        print_info "Bumping klever-sc family: $CURRENT_SC_VERSION -> $SC_VERSION"
        
        # Update framework crates
        print_info "Updating framework/base/Cargo.toml..."
        update_cargo_version "framework/base/Cargo.toml" "$CURRENT_SC_VERSION" "$SC_VERSION"
        
        print_info "Updating framework/derive/Cargo.toml..."
        update_cargo_version "framework/derive/Cargo.toml" "$CURRENT_SC_VERSION" "$SC_VERSION"
        
        print_info "Updating framework/meta/Cargo.toml..."
        update_cargo_version "framework/meta/Cargo.toml" "$CURRENT_SC_VERSION" "$SC_VERSION"
        
        print_info "Updating framework/scenario/Cargo.toml..."
        update_cargo_version "framework/scenario/Cargo.toml" "$CURRENT_SC_VERSION" "$SC_VERSION"
        
        print_info "Updating framework/wasm-adapter/Cargo.toml..."
        update_cargo_version "framework/wasm-adapter/Cargo.toml" "$CURRENT_SC_VERSION" "$SC_VERSION"
        
        print_info "Updating contracts/modules/Cargo.toml..."
        update_cargo_version "contracts/modules/Cargo.toml" "$CURRENT_SC_VERSION" "$SC_VERSION"
        
        # Update dependencies
        update_dependency_version "klever-sc" "$CURRENT_SC_VERSION" "$SC_VERSION"
        update_dependency_version "klever-sc-derive" "$CURRENT_SC_VERSION" "$SC_VERSION"
        update_dependency_version "klever-sc-meta" "$CURRENT_SC_VERSION" "$SC_VERSION"
        update_dependency_version "klever-sc-scenario" "$CURRENT_SC_VERSION" "$SC_VERSION"
        update_dependency_version "klever-sc-wasm-adapter" "$CURRENT_SC_VERSION" "$SC_VERSION"
        update_dependency_version "klever-sc-modules" "$CURRENT_SC_VERSION" "$SC_VERSION"
        
        # Update version_history.rs
        update_version_history "$SC_VERSION" "sc"
    fi
fi

# Codec Version Bump
if [[ -n "$CODEC_VERSION" ]]; then
    if [[ "$CURRENT_CODEC_VERSION" == "$CODEC_VERSION" ]]; then
        print_warning "klever-sc-codec is already at version $CODEC_VERSION, skipping..."
    else
        print_info "Bumping klever-sc-codec family: $CURRENT_CODEC_VERSION -> $CODEC_VERSION"
        
        update_cargo_version "data/codec/Cargo.toml" "$CURRENT_CODEC_VERSION" "$CODEC_VERSION"
        update_cargo_version "data/codec-derive/Cargo.toml" "$CURRENT_CODEC_VERSION" "$CODEC_VERSION"
        
        update_dependency_version "klever-sc-codec" "$CURRENT_CODEC_VERSION" "$CODEC_VERSION"
        update_dependency_version "klever-sc-codec-derive" "$CURRENT_CODEC_VERSION" "$CODEC_VERSION"
    fi
fi

# VM Version Bump
if [[ -n "$VM_VERSION" ]]; then
    if [[ "$CURRENT_VM_VERSION" == "$VM_VERSION" ]]; then
        print_warning "klever-chain-vm is already at version $VM_VERSION, skipping..."
    else
        print_info "Bumping klever-chain-vm: $CURRENT_VM_VERSION -> $VM_VERSION"
        
        update_cargo_version "vm/Cargo.toml" "$CURRENT_VM_VERSION" "$VM_VERSION"
        update_dependency_version "klever-chain-vm" "$CURRENT_VM_VERSION" "$VM_VERSION"
    fi
fi

# SDK Version Bump
if [[ -n "$SDK_VERSION" ]]; then
    if [[ "$CURRENT_SDK_VERSION" == "$SDK_VERSION" ]]; then
        print_warning "klever-vm-sdk is already at version $SDK_VERSION, skipping..."
    else
        print_info "Bumping klever-vm-sdk: $CURRENT_SDK_VERSION -> $SDK_VERSION"
        
        update_cargo_version "sdk/core/Cargo.toml" "$CURRENT_SDK_VERSION" "$SDK_VERSION"
        update_dependency_version "klever-vm-sdk" "$CURRENT_SDK_VERSION" "$SDK_VERSION"
    fi
fi

# Scenario Format Version Bump
if [[ -n "$SCENARIO_FORMAT_VERSION" ]]; then
    if [[ "$CURRENT_SCENARIO_FORMAT_VERSION" == "$SCENARIO_FORMAT_VERSION" ]]; then
        print_warning "klever-chain-scenario-format is already at version $SCENARIO_FORMAT_VERSION, skipping..."
    else
        print_info "Bumping klever-chain-scenario-format: $CURRENT_SCENARIO_FORMAT_VERSION -> $SCENARIO_FORMAT_VERSION"
        
        update_cargo_version "sdk/scenario-format/Cargo.toml" "$CURRENT_SCENARIO_FORMAT_VERSION" "$SCENARIO_FORMAT_VERSION"
        update_dependency_version "klever-chain-scenario-format" "$CURRENT_SCENARIO_FORMAT_VERSION" "$SCENARIO_FORMAT_VERSION"
    fi
fi

# Update CHANGELOG.md
update_changelog

# Run meta-update-all to update all Cargo.lock files
if [[ "$SKIP_UPDATE" == false && "$DRY_RUN" == false ]]; then
    print_info "Running meta-update-all to update Cargo.lock files..."
    print_info "This may take a few minutes..."
    
    # First ensure sc-meta is built
    if [[ ! -f "./target/release/sc-meta" ]]; then
        print_info "Building sc-meta first..."
        cargo build --release --bin sc-meta || {
            print_error "Failed to build sc-meta"
            exit 1
        }
    fi
    
    # Run with timeout and show output
    if timeout 300 ./scripts/meta-update-all.sh; then
        print_success "Meta update completed"
    else
        print_warning "Meta update timed out or failed, but continuing..."
    fi
fi

# Build WASM contracts
if [[ "$SKIP_BUILD" == false && "$DRY_RUN" == false ]]; then
    print_info "Building WASM contracts..."
    ./scripts/meta-build-wasm.sh
    print_success "WASM build completed"
fi

# Run tests
if [[ "$SKIP_TESTS" == false && "$DRY_RUN" == false ]]; then
    print_info "Running tests..."
    cargo test
    print_success "All tests passed"
fi

# Create commit message
commit_parts=()
[[ -n "$SC_VERSION" ]] && commit_parts+=("sc $SC_VERSION")
[[ -n "$CODEC_VERSION" ]] && commit_parts+=("codec $CODEC_VERSION")
[[ -n "$VM_VERSION" ]] && commit_parts+=("vm $VM_VERSION")
[[ -n "$SCENARIO_FORMAT_VERSION" ]] && commit_parts+=("scenario-format $SCENARIO_FORMAT_VERSION")
[[ -n "$SDK_VERSION" ]] && commit_parts+=("sdk $SDK_VERSION")
commit_message=$(IFS=", "; echo "${commit_parts[*]}")

if [[ "$DRY_RUN" == true ]]; then
    print_info "[DRY RUN] Complete! No changes were made."
    print_info "Would create commit: '$commit_message'"
    print_info "To apply changes, run without --dry-run"
else
    print_success "Version bump complete!"
    print_info "Next steps:"
    print_info "1. Review the changes: git diff"
    print_info "2. Commit changes: git commit -am '$commit_message'"
    print_info "3. Run publish script: ./scripts/publish.sh"
    print_info "4. Create tag after successful publish: git tag -s -a vX.X.X -m 'Release description'"
fi