
# download sns client
# GIT_HASH="f72ad66cc01b5441da26a596abaa2117c023844d" # latest version
GIT_HASH="25d05cb2d7f3ec39541c825659199610e3fcea48" # working version but not the latest one
PLATFORM=$(uname | tr '[[:upper:]]' '[[:lower:]]')
DOWNLOAD_NAME="sns"
DEST=./sns-exec
curl "https://download.dfinity.systems/ic/${GIT_HASH}/binaries/x86_64-${PLATFORM}/${DOWNLOAD_NAME}.gz" | zcat >"$DEST" && chmod +x "$DEST"

# delete all previous wasms
dfx canister uninstall-code --network staging sns_governance &&
dfx canister uninstall-code --network staging sns_index &&
dfx canister uninstall-code --network staging sns_root &&
dfx canister uninstall-code --network staging sns_ledger &&
dfx canister uninstall-code --network staging sns_swap

# execute sns
./sns-exec deploy-testflight --verbose --network staging --init-config-file=$(pwd)/sns/config/sns_init_staging.yaml
