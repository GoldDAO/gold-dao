#!/usr/bin/env bash

CANISTER_NAME="$1"

if [[ -z $CANISTER_NAME ]]; then
    echo "Error: CANISTER_NAME is not defined." >&2
    exit 1
fi

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

	./scripts/parse_changelog.sh $CANISTER_NAME $VERSION
else
	VERSION="!STAGINGTEST!"
	echo "No changelog for staging deployment" > CHANGELOG.md
fi

if [[ -n $CI_COMMIT_SHORT_SHA ]]; then
	COMMITSHA=$CI_COMMIT_SHORT_SHA
else
	COMMITSHA="00000000"
fi

DETAILSURLFIELD=$(cat proposal_template.md | grep "details_url")
export DETAILS_URL=${DETAILSURLFIELD:13}
sed "s/<<VERSIONTAG>>/${VERSION}/g" proposal_template.md > proposal.md && \
sed -i '' "s/<<COMMITHASH>>/${COMMITSHA}/g" proposal.md && \
sed -i '' "s/<<CANISTER>>/${CANISTER_NAME}/g" proposal.md && \
cat CHANGELOG.md >> proposal.md
return
