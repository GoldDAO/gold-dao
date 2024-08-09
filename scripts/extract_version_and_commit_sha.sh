#!/usr/bin/env bash

CANISTER_NAME=$1

if [[ -n $CI_COMMIT_TAG ]]; then
	VERSION="${CI_COMMIT_TAG#*-v}"
	NAME="${CI_COMMIT_TAG%-v*}"

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


export VERSION
export COMMIT_SHA

return
