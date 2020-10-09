#! /bin/bash

echo "Authenticating with Google Cloud"
case $CIRCLE_BRANCH in
	develop|deployToDev) \
		echo "authenticating with DEV for CD branch ${CIRCLE_BRANCH}"
		echo $GCLOUD_DEV_SERVICE_KEY | base64 --decode --ignore-garbage > ${HOME}/gcloud-service-key.json
		;;
	deployToQa/*) \
		echo "authenticating with QA for CD branch ${CIRCLE_BRANCH}"
		echo $GCLOUD_QA_SERVICE_KEY | base64 --decode --ignore-garbage > ${HOME}/gcloud-service-key.json
		;;
	deployToStage|release/v*|hotfix/v*) \
		echo "authenticating with STAGING for CD branch ${CIRCLE_BRANCH}"
		echo $GCLOUD_STAGING_SERVICE_KEY | base64 --decode --ignore-garbage > ${HOME}/gcloud-service-key.json
		;;
	master) \
		echo "authenticating with PRODUCTION for CD branch ${CIRCLE_BRANCH}"
		echo $GCLOUD_PRODUCTION_SERVICE_KEY | base64 --decode --ignore-garbage > ${HOME}/gcloud-service-key.json
		;;
	*) echo "${CIRCLE_BRANCH} is not supported yet"
		;;
esac

echo "Updating Google Cloud"
sudo /opt/google-cloud-sdk/bin/gcloud --quiet components update
sudo /opt/google-cloud-sdk/bin/gcloud --quiet components update kubectl
sudo /opt/google-cloud-sdk/bin/gcloud auth activate-service-account --key-file ${HOME}/gcloud-service-key.json

echo "Configuring Google Cloud"
sudo /opt/google-cloud-sdk/bin/gcloud config set project $GCLOUD_PROJECT
sudo /opt/google-cloud-sdk/bin/gcloud --quiet config set container/cluster $GCLOUD_CLUSTER_NAME
sudo /opt/google-cloud-sdk/bin/gcloud config set compute/zone ${GCLOUD_COMPUTE_ZONE}
sudo /opt/google-cloud-sdk/bin/gcloud --quiet container clusters get-credentials $GCLOUD_CLUSTER_NAME

echo "Printing cluster information"
sudo /opt/google-cloud-sdk/bin/kubectl cluster-info

echo "change owner"
sudo chown -R circleci:circleci /home/circleci/.kube
