#!/usr/bin/env bash

SNS_TESTING_INSTANCE=$(
	docker run -p 8000:8000 -p 8080:8080 -v "`pwd`":/dapp -d ghcr.io/dfinity/sns-testing:main dfx start --clean
)
while ! docker logs $SNS_TESTING_INSTANCE 2>&1 | grep -m 1 'Dashboard:'
do
	echo "Awaiting local replica ..."
	sleep 3
done

docker exec -it $SNS_TESTING_INSTANCE bash setup_locally.sh

case "$(uname -s)" in
	Linux*)
		xdg-open http://qsgjb-riaaa-aaaaa-aaaga-cai.localhost:8080/;;
	Darwin*)
		open http://qsgjb-riaaa-aaaaa-aaaga-cai.localhost:8080/;;
	*)
		echo "Unrecognized system. Visit the local NNS frontend in your browser: http://qsgjb-riaaa-aaaaa-aaaga-cai.localhost:8080/"
esac


docker exec -it $SNS_TESTING_INSTANCE bash "cd /dapp && NETWORK='local' scripts/local-deploy.sh"

docker exec -it $SNS_TESTING_INSTANCE bash

docker kill $SNS_TESTING_INSTANCE
