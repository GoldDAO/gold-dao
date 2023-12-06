#!/usr/bin/env bash

SNS_TESTING_INSTANCE=$(
	docker run -p 8000:8000 -p 8080:8080 -v "`pwd`":/dapp -d ghcr.io/dfinity/sns-testing:main dfx start --clean --artificial-delay 100
)
while ! docker logs $SNS_TESTING_INSTANCE 2>&1 | grep -m 1 'Dashboard:'
do
	echo "Awaiting local replica ..."
	sleep 3
done

curl --progress-bar -o scripts/nvm_install.sh https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.6/install.sh
docker exec -it $SNS_TESTING_INSTANCE bash /dapp/scripts/nvm_install.sh
rm scripts/nvm_install.sh

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

	cat << EOF
The dockerized replica should now be running with the NNS and SNS canisters deployed,
which should be listed on the dashboard: http://localhost:8000/_/dashboard

You should now finish to setup the node environment and deploy the Gold canisters manually:

nvm install 18
cd /dapp
npm install && npm run build && npm run deploy

Then you should be able to interact with your canisters with dfx from the /dapp folder, and
the NNS and SNS canisters with dfx from the ~/ ($HOME) folder.

Type 'exit' when you are finished.

EOF

docker exec -it $SNS_TESTING_INSTANCE bash

# At this point, you can work on the hosted replica with both dfx and quill commands,
# or from the canisters frontends served on port 8080.
# Then when finished testing, simply type 'exit' in the terminal.

docker kill $SNS_TESTING_INSTANCE
docker rm $SNS_TESTING_INSTANCE
