#!/usr/bin/env bash

echo "Using host identiy $(dfx identity whoami) to replicate id on dockerized environement."

NETNAME="snstesting"

SNS_TESTING_INSTANCE=$(
	docker run -p 8000:8000 -p 8080:8080 -v "`pwd`":/dapp -d ghcr.io/dfinity/sns-testing:main dfx start --clean --artificial-delay 100
)
while ! docker logs $SNS_TESTING_INSTANCE 2>&1 | grep -m 1 'Dashboard:'
do
	echo "Awaiting local replica ..."
	sleep 3
done

WALLETID=$(dfx identity get-wallet --network ${NETNAME})
#dfx identity deploy-wallet --network $NETNAME $WALLETID
dfx identity export $(dfx identity whoami) > tmp_id.pem

docker exec -it $SNS_TESTING_INSTANCE dfx identity import -v --storage-mode plaintext --force default /dapp/tmp_id.pem
rm tmp_id.pem
#docker exec -it $SNS_TESTING_INSTANCE dfx identity set-wallet -v ${WALLETID}

docker exec -it $SNS_TESTING_INSTANCE bash setup_locally.sh

curl --progress-bar -o scripts/nvm_install.sh https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.6/install.sh
docker exec -it $SNS_TESTING_INSTANCE bash /dapp/scripts/nvm_install.sh
rm scripts/nvm_install.sh
docker exec -it $SNS_TESTING_INSTANCE bash "nvm install 18"

NETWORK="snstesting" scripts/local-deploy.sh

case "$(uname -s)" in
	Linux*)
		xdg-open http://localhost:8000/_/dashboard
		xdg-open http://qsgjb-riaaa-aaaaa-aaaga-cai.localhost:8080/;;
	Darwin*)
		open http://localhost:8000/_/dashboard
		open http://qsgjb-riaaa-aaaaa-aaaga-cai.localhost:8080/;;
	*)
		echo "Unrecognized system. Visit the local NNS frontend in your browser: http://qsgjb-riaaa-aaaaa-aaaga-cai.localhost:8080/"
esac

docker exec -it $SNS_TESTING_INSTANCE bash

# At this point, you can work on the hosted replica with both dfx and quill commands,
# or from the canisters frontends served on port 8080.
# Then when finished testing, simply type 'exit' in the terminal.

docker kill $SNS_TESTING_INSTANCE
docker rm $SNS_TESTING_INSTANCE
