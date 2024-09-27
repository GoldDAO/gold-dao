#!/usr/bin/env bash

CANISTER_NAME=$1
NETWORK=$2

if [[ -n $CI_COMMIT_TAG && $NETWORK == "ic" ]]; then

	if [[ ! $CI_COMMIT_TAG =~ ^([a-zA-Z0-9_]+-v[0-9]+\.[0-9]+\.[0-9]+(-reinstall)?)$ ]]; then
			echo "Error: commit tag $CI_COMMIT_TAG doesn't match expected format of $CANISTER_NAME-v1.2.3(-reinstall)" >&2
			exit 1
	fi

	NAME=$(echo "$CI_COMMIT_TAG" | awk -F'-' '{print $1}')
	VERSION_STRING=$(echo "$CI_COMMIT_TAG" | awk -F'-' '{print $2}')
	REINSTALL=$(echo "$CI_COMMIT_TAG" | awk -F'-' '{print $3}')

	VERSION="${VERSION_STRING#*v}" # converts v1.2.3 into 1.2.3

	VERSION_MAJOR=$(echo "$VERSION" | awk -F'.' '{print $1}')
	VERSION_MINOR=$(echo "$VERSION" | awk -F'.' '{print $2}')
	VERSION_PATCH=$(echo "$VERSION" | awk -F'.' '{print $3}')

	BUILD_VERSION="record { major = $VERSION_MAJOR:nat32; minor = $VERSION_MINOR:nat32; patch = $VERSION_PATCH:nat32 }"

	# Check if NAME matches CANISTER_NAME
	# The script requires CANISTER_NAME to be defined for staging deployment and
	# because CI_COMMIT_TAG are only set on master, we add this safety check that they match
	if [[ "$NAME" != "$CANISTER_NAME" ]]; then
			echo "Error: NAME extracted from CI_COMMIT_TAG ('$NAME') does not match CANISTER_NAME ('$CANISTER_NAME')." >&2
			exit 1
	fi
else
	VERSION="_STAGINGTEST_"
	BUILD_VERSION="record { major = 0:nat32; minor = 0:nat32; patch = 0:nat32 }"
fi

if [[ -n $CI_COMMIT_SHORT_SHA ]]; then
	COMMIT_SHA=$CI_COMMIT_SHORT_SHA
else
	COMMIT_SHA="$(git rev-parse --short HEAD)_local"
fi

echo "CANISTER: $CANISTER_NAME, VERSION: $VERSION, COMMIT_SHA: $COMMIT_SHA, BUILD_VERSION: $BUILD_VERSION, $REINSTALL"

export VERSION
export COMMIT_SHA
export BUILD_VERSION
export REINSTALL
