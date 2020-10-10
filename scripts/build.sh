#! /bin/bash
set -e
PACKAGE_NAME=$(sed -nE 's/^\s*name = "(.*?)"$/\1/p' Cargo.toml)
PACKAGE_VERSION=$(sed -nE 's/^\s*version = "(.*?)"$/\1/p' Cargo.toml)
GIT_HASH=$(git rev-parse --short HEAD)
NAME_TAG=$PACKAGE_NAME:$PACKAGE_VERSION
BUILD_VERSION=$PACKAGE_VERSION-$GIT_HASH
HOST=us.gcr.io/rust-fullstack-demo

docker build -t $HOST/$NAME_TAG .

echo "pushing image ${HOST}/${NAME_TAG}"
# sudo /opt/google-cloud-sdk/bin/gcloud docker -- push $HOST/$NAME_TAG

echo "cleaning image ${HOST}/${NAME_TAG}"
# docker rmi -f ${HOST}/${NAME_TAG}
