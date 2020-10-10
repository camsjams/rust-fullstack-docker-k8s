#! /bin/bash
set -e
PACKAGE_VERSION=$(sed -nE 's/^\s*version = "(.*?)"$/\1/p' Cargo.toml)
NAME_TAG=$CIRCLE_PROJECT_REPONAME:$PACKAGE_VERSION-$CIRCLE_BUILD_NUM
BUILD_VERSION=$PACKAGE_VERSION-$CIRCLE_BUILD_NUM-$CIRCLE_SHA1
HOST=$GCLOUD_CR_REGION/$GCLOUD_PROJECT

docker build -t $HOST/$NAME_TAG \
	--build-arg BUILD_VERSION=${BUILD_VERSION} \
	.

echo "pushing image ${HOST}/${NAME_TAG}"
sudo /opt/google-cloud-sdk/bin/gcloud docker -- push $HOST/$NAME_TAG
