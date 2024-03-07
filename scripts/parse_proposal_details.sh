#!/usr/bin/env bash

if [[ -v $CI_COMMIT_TAG ]]; then
	VERSION=$CI_COMMIT_TAG
else
	VERSION="!STAGINGTEST!"
fi

if [[ -v $CI_COMMIT_SHORT_SHA ]]; then
	COMMITSHA=$CI_COMMIT_SHORT_SHA
else
	COMMITSHA="00000000"
fi

DETAILSURLFIELD=$(cat proposal_template.md | grep "details_url")
export DETAILS_URL=${DETAILSURLFIELD:13}
sed -i.bak "s/<<VERSIONTAG>>/${VERSION}/g" proposal_template.md && \
sed -i "s/<<COMMITHASH>>/${COMMITSHA}/g" proposal_template.md && \
cp -f proposal_template.md proposal.md
mv -f proposal_template.md.bak proposal_template.md
return
