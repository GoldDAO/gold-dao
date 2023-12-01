#!/usr/bin/env bash

SNS_TESTING_INSTANCE=$(
	docker run -p 8000:8000 -p 8080:8080 -v "`pwd`":/dapp -d ghcr.io/dfinity/sns-testing:main dfx start --clean --artificial-delay 100
)
while ! docker logs $SNS_TESTING_INSTANCE 2>&1 | grep -m 1 'Dashboard:'
do
	echo "Awaiting local replica ..."
	sleep 3
done

docker exec -it $SNS_TESTING_INSTANCE bash setup_locally.sh

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

NETWORK="snstesting" scripts/local-deploy.sh

docker exec -it $SNS_TESTING_INSTANCE bash

# At this point, you can work on the hosted replica with both dfx and quill commands,
# or from the canisters frontends served on port 8080.
# Then when finished testing, simply type 'exit' in the terminal.

docker kill $SNS_TESTING_INSTANCE
