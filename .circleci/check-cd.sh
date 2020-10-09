#! /bin/bash
apk --update add curl

case $CIRCLE_BRANCH in
	develop|master) \
		echo "starting deploy for CD branch ${CIRCLE_BRANCH}"
		curl --user ${CIRCLE_API_TOKEN}: \
			--data build_parameters[CIRCLE_JOB]=deploy_${CIRCLE_BRANCH} \
			--data revision=$CIRCLE_SHA1 \
			https://circleci.com/api/v1.1/project/github/$CIRCLE_PROJECT_USERNAME/$CIRCLE_PROJECT_REPONAME/tree/$CIRCLE_BRANCH
		;;
	# git push origin HEAD:deployToDev -f
	deployToDev) \
		echo "starting deploy for CD branch ${CIRCLE_BRANCH}"
		curl --user ${CIRCLE_API_TOKEN}: \
			--data build_parameters[CIRCLE_JOB]=deploy_develop \
			--data revision=$CIRCLE_SHA1 \
			https://circleci.com/api/v1.1/project/github/$CIRCLE_PROJECT_USERNAME/$CIRCLE_PROJECT_REPONAME/tree/$CIRCLE_BRANCH
		;;
	# git push origin HEAD:deployToStage -f
	deployToStage|release/v*|hotfix/v*) \
		echo "starting deploy to staging for CD branch ${CIRCLE_BRANCH}"
		curl --user ${CIRCLE_API_TOKEN}: \
			--data build_parameters[CIRCLE_JOB]=deploy_staging \
			--data revision=$CIRCLE_SHA1 \
			https://circleci.com/api/v1.1/project/github/$CIRCLE_PROJECT_USERNAME/$CIRCLE_PROJECT_REPONAME/tree/$CIRCLE_BRANCH
		;;
	*) echo "${CIRCLE_BRANCH} is not a CD branch"
		;;
esac
