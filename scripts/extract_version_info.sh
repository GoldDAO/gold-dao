#!/usr/bin/env bash


if [[ $# -ne 1 ]]; then
  echo "Error: missing <CANISTER-NAME> argument."
  exit 1
fi

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

	# ./scripts/parse_changelog.sh $CANISTER_NAME $VERSION
	# exit_status=$? # Capture the exit status of the last command

	# if [[ $exit_status -eq 1 ]]; then
	# 		echo "Error: parse_changelog.sh exited with status 1." >&2
	# 		exit 1
	# fi
else
	VERSION="_STAGINGTEST_"
	# echo "No changelog for staging deployment" > CHANGELOG.md
fi

export VERSION

# return
