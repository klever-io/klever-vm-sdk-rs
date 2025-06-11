#!/bin/bash

# Script to check all Cargo.toml files for outdated dependencies
# Usage: ./scripts/check-dependencies.sh [OPTIONS] [PATH]
#
# Examples:
#   ./scripts/check-dependencies.sh                    # Check all files
#   ./scripts/check-dependencies.sh --fix              # Fix all outdated dependencies
#   ./scripts/check-dependencies.sh -v                 # Verbose output
#   ./scripts/check-dependencies.sh contracts/examples # Check specific directory
#   ./scripts/check-dependencies.sh --show-all -v      # Show all files with details

set -e

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

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Function to get version from Cargo.toml
get_version() {
    local file=$1
    grep "^version" "$file" | sed -E -n 's/.*"=?([0-9.]+)".*/\1/p' | head -1
}

# Detect current versions from source files
KLEVER_SC_VERSION=$(get_version "framework/base/Cargo.toml")
KLEVER_VM_VERSION=$(get_version "vm/Cargo.toml")
KLEVER_SC_CODEC_VERSION=$(get_version "data/codec/Cargo.toml")
KLEVER_VM_SDK_VERSION=$(get_version "sdk/core/Cargo.toml")
KLEVER_SCENARIO_FORMAT_VERSION=$(get_version "sdk/scenario-format/Cargo.toml")

# Verify versions were detected
if [[ -z "$KLEVER_SC_VERSION" ]] || [[ -z "$KLEVER_VM_VERSION" ]] || [[ -z "$KLEVER_SC_CODEC_VERSION" ]] || [[ -z "$KLEVER_VM_SDK_VERSION" ]] || [[ -z "$KLEVER_SCENARIO_FORMAT_VERSION" ]]; then
    print_error "Failed to detect current versions from source files"
    echo ""
    [[ -z "$KLEVER_SC_VERSION" ]] && print_error "  klever-sc version not found in framework/base/Cargo.toml"
    [[ -z "$KLEVER_VM_VERSION" ]] && print_error "  klever-chain-vm version not found in vm/Cargo.toml"
    [[ -z "$KLEVER_SC_CODEC_VERSION" ]] && print_error "  klever-sc-codec version not found in data/codec/Cargo.toml"
    [[ -z "$KLEVER_VM_SDK_VERSION" ]] && print_error "  klever-vm-sdk version not found in sdk/core/Cargo.toml"
    [[ -z "$KLEVER_SCENARIO_FORMAT_VERSION" ]] && print_error "  klever-chain-scenario-format version not found in sdk/scenario-format/Cargo.toml"
    echo ""
    echo "Please ensure you're running this script from the repository root."
    exit 1
fi

# Parse command line arguments
FIX_MODE=false
VERBOSE=false
SHOW_CORRECT=false
SEARCH_PATH="."

while [[ $# -gt 0 ]]; do
    case $1 in
        --fix)
            FIX_MODE=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        --show-all)
            SHOW_CORRECT=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS] [PATH]"
            echo ""
            echo "Options:"
            echo "  --fix          Automatically fix outdated dependencies"
            echo "  -v, --verbose  Show detailed output"
            echo "  --show-all     Show files with correct versions too"
            echo "  -h, --help     Show this help message"
            echo ""
            echo "Arguments:"
            echo "  PATH           Directory to search (default: current directory)"
            echo ""
            echo "Examples:"
            echo "  $0                         # Check all files"
            echo "  $0 --fix                   # Fix all outdated dependencies"
            echo "  $0 -v contracts/examples   # Verbose check of examples"
            echo "  $0 --show-all -v           # Show all dependencies"
            exit 0
            ;;
        -*)
            echo "Unknown option: $1"
            echo "Use -h or --help for usage information"
            exit 1
            ;;
        *)
            # Assume it's a path
            SEARCH_PATH="$1"
            shift
            ;;
    esac
done

# Function to check dependency version in a file
check_dependency() {
    local file=$1
    local dep_name=$2
    local expected_version=$3
    local found_issue=false
    
    # Check if dependency exists in file
    if grep -q "$dep_name" "$file"; then
        # Check TOML section format [dependencies.dep_name] or [dev-dependencies.dep_name]
        if grep -A5 "^\[.*dependencies\.$dep_name\]" "$file" > /dev/null 2>&1; then
            local version_line=$(grep -A5 "^\[.*dependencies\.$dep_name\]" "$file" | grep "^version" | head -1)
            if [[ -n "$version_line" ]]; then
                local current_version=$(echo "$version_line" | sed -E -n 's/.*version[[:space:]]*=[[:space:]]*"=?([0-9.]+)".*/\1/p')
                if [[ "$current_version" != "$expected_version" && "$current_version" != "." ]]; then
                    echo "  - $dep_name: $current_version -> $expected_version (TOML section)"
                    found_issue=true
                fi
            fi
        fi
        
        # Check inline format: dep_name = "version" or dep_name = { version = "version", ... }
        local inline_matches=$(grep "$dep_name.*=" "$file" | grep -v "^\[")
        if [[ -n "$inline_matches" ]]; then
            while IFS= read -r line; do
                # Skip empty lines
                [[ -z "$line" ]] && continue
                
                # Extract version from inline format
                local current_version=""
                if echo "$line" | grep -q "version.*="; then
                    # Format: dep_name = { version = "0.x.x", ... }
                    current_version=$(echo "$line" | sed -E -n 's/.*version[[:space:]]*=[[:space:]]*"=?([0-9.]+)".*/\1/p')
                else
                    # Format: dep_name = "0.x.x"
                    current_version=$(echo "$line" | sed -E -n 's/.*=[[:space:]]*"=?([0-9.]+)".*/\1/p')
                fi
                
                if [[ -n "$current_version" && "$current_version" != "$expected_version" && "$current_version" != "." ]]; then
                    echo "  - $dep_name: $current_version -> $expected_version (inline)"
                    found_issue=true
                fi
            done <<< "$inline_matches"
        fi
    fi
    
    if [[ "$found_issue" == true ]]; then
        echo "true"
    else
        echo "false"
    fi
}

# Find all Cargo.toml files
print_info "Scanning for Cargo.toml files in $SEARCH_PATH..."
cargo_files=$(find "$SEARCH_PATH" -name "Cargo.toml" -not -path "*/target/*" -not -path "*/interact-rs/*" | sort)

# Statistics
total_files=0
files_with_issues=0
total_issues=0

# Header
echo ""
print_info "Expected dependency versions (detected from source):"
echo "  klever-sc: $KLEVER_SC_VERSION (from framework/base/Cargo.toml)"
echo "  klever-chain-vm: $KLEVER_VM_VERSION (from vm/Cargo.toml)"
echo "  klever-sc-codec: $KLEVER_SC_CODEC_VERSION (from data/codec/Cargo.toml)"
echo "  klever-vm-sdk: $KLEVER_VM_SDK_VERSION (from sdk/core/Cargo.toml)"
echo "  klever-chain-scenario-format: $KLEVER_SCENARIO_FORMAT_VERSION (from sdk/scenario-format/Cargo.toml)"
echo ""

# Check each file
print_info "Checking dependencies..."
echo ""

for file in $cargo_files; do
    total_files=$((total_files + 1))
    file_has_issues=false
    file_dependencies=""
    
    # Check each dependency
    for dep in "klever-sc:$KLEVER_SC_VERSION" \
               "klever-sc-derive:$KLEVER_SC_VERSION" \
               "klever-sc-meta:$KLEVER_SC_VERSION" \
               "klever-sc-scenario:$KLEVER_SC_VERSION" \
               "klever-sc-wasm-adapter:$KLEVER_SC_VERSION" \
               "klever-chain-vm:$KLEVER_VM_VERSION" \
               "klever-sc-codec:$KLEVER_SC_CODEC_VERSION" \
               "klever-sc-codec-derive:$KLEVER_SC_CODEC_VERSION" \
               "klever-vm-sdk:$KLEVER_VM_SDK_VERSION" \
               "klever-chain-scenario-format:$KLEVER_SCENARIO_FORMAT_VERSION"; do
        
        IFS=':' read -r dep_name expected_version <<< "$dep"
        
        # Check if dependency exists
        if grep -q "$dep_name" "$file"; then
            # Capture both the result and any output
            output=$(check_dependency "$file" "$dep_name" "$expected_version" 2>&1)
            result=$(echo "$output" | tail -1)
            # Get all lines except the last one
            dependency_info=$(echo "$output" | sed '$d')
            
            if [[ "$result" == "true" ]]; then
                file_has_issues=true
                if [[ -n "$dependency_info" ]]; then
                    file_dependencies+="$dependency_info\n"
                fi
            elif [[ "$SHOW_CORRECT" == true ]]; then
                file_dependencies+="  ✓ $dep_name: $expected_version\n"
            fi
        fi
    done
    
    # Print results based on flags
    if [[ "$file_has_issues" == true ]]; then
        files_with_issues=$((files_with_issues + 1))
        print_warning "$file"
        if [[ -n "$file_dependencies" ]]; then
            echo -e "$file_dependencies"
        fi
    elif [[ "$SHOW_CORRECT" == true && -n "$file_dependencies" ]]; then
        print_success "$file"
        if [[ "$VERBOSE" == true ]]; then
            echo -e "$file_dependencies"
        fi
    elif [[ "$VERBOSE" == true ]]; then
        echo "  $file - OK"
    fi
done

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
print_info "Summary:"
echo "  Total files scanned: $total_files"
if [[ $files_with_issues -eq 0 ]]; then
    print_success "All dependencies are up to date! ✓"
else
    print_warning "Files with outdated dependencies: $files_with_issues"
    
    if [[ "$FIX_MODE" == true ]]; then
        echo ""
        print_info "Running version bump script to fix outdated dependencies..."
        
        # Run the version bump script
        if [[ -f "./scripts/version-bump.sh" ]]; then
            bash ./scripts/version-bump.sh \
                -s "$KLEVER_SC_VERSION" \
                -c "$KLEVER_SC_CODEC_VERSION" \
                -v "$KLEVER_VM_VERSION" \
                --sdk-version "$KLEVER_VM_SDK_VERSION" \
                --scenario-version "$KLEVER_SCENARIO_FORMAT_VERSION" \
                --skip-changelog \
                --skip-tests \
                --skip-update \
                --skip-build
            
            print_success "Dependencies updated!"
        else
            print_error "version-bump.sh not found!"
        fi
    else
        echo ""
        echo "Run with --fix flag to automatically update outdated dependencies:"
        echo "  ./scripts/check-dependencies.sh --fix"
    fi
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"