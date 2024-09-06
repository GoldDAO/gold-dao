#!/usr/bin/env bash


PROPOSAL_SUMMARY_FILE=proposal.md
CHANGELOG_FILE=CHANGELOG.md

CANISTER_NAME="$1"
VERSION=$2 # needs to be in format 1.2.3
CANISTER_TYPE=$3 # needs to be "backend" or "frontend"

if [[ -z $CANISTER_NAME ]]; then
    echo "Error: CANISTER_NAME is not defined." >&2
    exit 1
fi

if [[ $VERSION =~ ^([0-9]+\.[0-9]+\.[0-9]+)$ ]]; then
	./scripts/parse_changelog.sh $CANISTER_NAME $VERSION $CANISTER_TYPE
	exit_status=$? # Capture the exit status of the last command

	if [[ $exit_status -eq 1 ]]; then
			echo "Error: parse_changelog.sh exited with status 1." >&2
			exit 1
	fi
elif [[ $VERSION == "_STAGINGTEST_" ]]; then
	echo "No changelog for staging deployment" > $CHANGELOG_FILE
else
  echo "Invalid version $VERSION."
  exit 1
fi

echo "
  Creating proposal summary with

  ** CANISTER_NAME: $CANISTER_NAME
  ** VERSION: $VERSION
  ** COMMIT_SHA: $COMMIT_SHA
  ** CANISTER_TYPE: $CANISTER_TYPE

  In case of frontend proposal:
  ** BATCH_ID: $BATCH_ID
  ** EVIDENCE: $EVIDENCE
"

export DETAILS_URL="https://github.com/GoldDAO/gldt-swap/commit/${COMMIT_SHA}"
sed "s/<<VERSIONTAG>>/${VERSION}/g" proposal_${CANISTER_TYPE}_template.md > $PROPOSAL_SUMMARY_FILE

if [[ "$(uname -s)" == "Darwin" ]]; then
# mac requires to run set with '' -> sed -i '' ... whereas pipeline doesn't
  sed -i '' "s/<<COMMITHASH>>/${COMMIT_SHA}/g" $PROPOSAL_SUMMARY_FILE
  sed -i '' "s/<<CANISTER>>/${CANISTER_NAME}/g" $PROPOSAL_SUMMARY_FILE

  if [[ $CANISTER_TYPE == "frontend" ]]; then
    sed -i '' "s/<<BATCH_ID>>/${BATCH_ID}/g" $PROPOSAL_SUMMARY_FILE
    sed -i '' "s/<<EVIDENCE>>/${EVIDENCE}/g" $PROPOSAL_SUMMARY_FILE
  fi
else
  sed -i "s/<<COMMITHASH>>/${COMMIT_SHA}/g" $PROPOSAL_SUMMARY_FILE
  sed -i "s/<<CANISTER>>/${CANISTER_NAME}/g" $PROPOSAL_SUMMARY_FILE

  if [[ $CANISTER_TYPE == "frontend" ]]; then
    sed -i "s/<<BATCH_ID>>/${BATCH_ID}/g" $PROPOSAL_SUMMARY_FILE
    sed -i "s/<<EVIDENCE>>/${EVIDENCE}/g" $PROPOSAL_SUMMARY_FILE
  fi
fi

cat $CHANGELOG_FILE >> $PROPOSAL_SUMMARY_FILE

echo "
******************************************
Proposal summary:
"

cat $PROPOSAL_SUMMARY_FILE

echo "
******************************************
"

return
