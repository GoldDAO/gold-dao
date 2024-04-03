#!/usr/bin/env bash

CANISTER="$1"
VERSION="$2"

CHANGELOG_FILE="backend/canisters/$CANISTER/CHANGELOG.md"

awk -v version="$VERSION" '
  BEGIN {print_version_info = 0}
  /^\#\#\# \[unreleased\]/ {print_version_info = 0}
  /^\#\#\# \[/ {if ($2 == "["version"]") print_version_info = 1; else if (print_version_info) exit}
  print_version_info {print}
' "$CHANGELOG_FILE" > CHANGELOG.md

if [ ! -s CHANGELOG.md ]; then
    echo "Error: No entries found for version $VERSION in the CHANGELOG.md for canister $CANISTER. Make sure to provide the proper changelog for this version." >&2
    rm CHANGELOG.md
    exit 1
fi
