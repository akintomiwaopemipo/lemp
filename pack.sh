set -e

APP_NAME=lemp
APP_VERSION=`"target/release/$APP_NAME" --version | cut -d " " -f2`
zip $HOME/Documents/$APP_NAME-v$APP_VERSION-ubuntu-24.04-amd64.zip target/release/$APP_NAME