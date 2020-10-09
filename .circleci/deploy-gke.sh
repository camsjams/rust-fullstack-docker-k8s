#! /bin/bash
PACKAGE_VERSION=$(sed -nE 's/^\s*version = "(.*?)"$/\1/p' Cargo.toml)
NAME_TAG=$CIRCLE_PROJECT_REPONAME:$PACKAGE_VERSION-$CIRCLE_BUILD_NUM
HOST=$GCLOUD_CR_REGION/$GCLOUD_PROJECT

echo "applying infrastructure for ${PWD}/infrastructure/${K8S_PATH}/kubernetes.yaml"
sudo /opt/google-cloud-sdk/bin/kubectl apply -f ${PWD}/infrastructure/${K8S_PATH}/kubernetes.yaml

echo "updating deployment image to ${HOST}/${NAME_TAG}"
sudo /opt/google-cloud-sdk/bin/kubectl set image deployment/rust-server rust-server=$HOST/$NAME_TAG
