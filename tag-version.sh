set -e

APP_NAME=lemp

APP_VERSION=`$APP_NAME --version | cut -d " " -f2`

git tag -a v$APP_VERSION -m "Version $APP_VERSION"

git push origin --tags
