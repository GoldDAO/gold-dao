#!/usr/bin/env bash


show_help() {
  cat << EOF
Script to prase proposal template and generate the proposal text.
Needs to receive
* <CANISTER-NAME> (e.g. icp_neuron or gld_dashboard)
* <CANISTER-TYPE> (backend or frontend)

Usage:
  prepare_proposal_summary.sh [options] <CANISTER-NAME> <CANISTER-TYPE>

Options:
  -h, --help        Show this message and exit
EOF
}

if [[ $# -gt 3 ]]; then
  while [[ "$1" =~ ^- && ! "$1" == "--" ]]; do
    case $1 in
      -h | --help )
        show_help
        exit
        ;;
    esac;
    shift;
  done
  if [[ "$1" == '--' ]]; then shift; fi
else
  echo "Error: missing <CANISTER-NAME>, <CANISTER-TYPE> and/or <VERSION> argument"
  exit 1
fi

PROPOSAL_SUMMARY_FILE=proposal.md

CANISTER_NAME="$1"
VERSION=$2
CANISTER_TYPE=$3

if [[ -z $CANISTER_NAME ]]; then
    echo "Error: CANISTER_NAME is not defined." >&2
    exit 1
fi

# if [[ -n $CI_COMMIT_TAG ]]; then
# 	VERSION="${CI_COMMIT_TAG#*-v}"
# 	NAME="${CI_COMMIT_TAG%-v*}"

# 	# Check if NAME matches CANISTER_NAME
# 	# The script requires CANISTER_NAME to be defined for staging deployment and
# 	# because CI_COMMIT_TAG are only set on master, we add this safety check that they match
# 	if [[ "$NAME" != "$CANISTER_NAME" ]]; then
# 			echo "Error: NAME extracted from CI_COMMIT_TAG ('$NAME') does not match CANISTER_NAME ('$CANISTER_NAME')." >&2
# 			exit 1
# 	fi
if [[ $VERSION =~ '/^\d+\.\d+\.\d+$/' ]]; then
	./scripts/parse_changelog.sh $CANISTER_NAME $VERSION
	exit_status=$? # Capture the exit status of the last command

	if [[ $exit_status -eq 1 ]]; then
			echo "Error: parse_changelog.sh exited with status 1." >&2
			exit 1
	fi
else
	echo "No changelog for staging deployment" > CHANGELOG.md
fi

if [[ -n $CI_COMMIT_SHORT_SHA ]]; then
	COMMITSHA=$CI_COMMIT_SHORT_SHA
else
	COMMITSHA="00000000"
fi

echo $COMMIT_SHA
echo $VERSION
echo $CANISTER_NAME
echo $BATCH_ID
echo $EVIDENCE

export DETAILS_URL="https://github.com/GoldDAO/gldt-swap/commit/${COMMITSHA}"
sed "s/<<VERSIONTAG>>/${VERSION}/g" proposal_${CANISTER_TYPE}_template.md > $PROPOSAL_SUMMARY_FILE && /
sed -i '' "s/<<COMMITHASH>>/${COMMITSHA}/g" $PROPOSAL_SUMMARY_FILE && /
sed -i '' "s/<<CANISTER>>/${CANISTER_NAME}/g" $PROPOSAL_SUMMARY_FILE

if [[ $CANISTER_TYPE == "frontend" ]]; then
	sed -i '' "s/<<BATCH_ID>>/${BATCH_ID}/g" $PROPOSAL_SUMMARY_FILE
	sed -i '' "s/<<EVIDENCE>>/${EVIDENCE}/g" $PROPOSAL_SUMMARY_FILE
fi

cat CHANGELOG.md >> $PROPOSAL_SUMMARY_FILE

cat $PROPOSAL_SUMMARY_FILE

return
