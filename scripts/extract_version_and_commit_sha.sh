#!/usr/bin/env bash

CANISTER_NAME=$1
NETWORK=$2

if [[ -n $CI_COMMIT_TAG && $NETWORK == "ic" ]]; then

	if [[ ! $CI_COMMIT_TAG =~ ^([a-zA-Z0-9_]+-v[0-9]+\.[0-9]+\.[0-9]+)$ ]]; then
			echo "Error: commit tag $CI_COMMIT_TAG doesn't match expected format of xxx-v1.2.3" >&2
			exit 1
	fi

	VERSION="${CI_COMMIT_TAG#*-v}" # converts test-v1.2.3 into 1.2.3
	NAME="${CI_COMMIT_TAG%-v*}" # converts test-v1.2.3 into test

	# Check if NAME matches CANISTER_NAME
	# The script requires CANISTER_NAME to be defined for staging deployment and
	# because CI_COMMIT_TAG are only set on master, we add this safety check that they match
	if [[ "$NAME" != "$CANISTER_NAME" ]]; then
			echo "Error: NAME extracted from CI_COMMIT_TAG ('$NAME') does not match CANISTER_NAME ('$CANISTER_NAME')." >&2
			exit 1
	fi

else
	VERSION="_STAGINGTEST_"
fi

if [[ -n $CI_COMMIT_SHORT_SHA ]]; then
	COMMIT_SHA=$CI_COMMIT_SHORT_SHA
else
	COMMIT_SHA="$(git rev-parse --short HEAD)_local"
fi

echo "CANISTER: $CANISTER_NAME, VERSION: $VERSION, COMMIT_SHA: $COMMIT_SHA"

export VERSION
export COMMIT_SHA
