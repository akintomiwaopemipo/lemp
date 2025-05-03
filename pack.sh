set -e

APP_NAME=lemp
PWD=`pwd`
APP_VERSION=`"./target/release/$APP_NAME" --version | cut -d " " -f2`
ARCHIVE_PATH=$PWD/releases/$APP_NAME-v$APP_VERSION-ubuntu-24.04-amd64.zip

mkdir -p $PWD/releases

pushd "target/release" > /dev/null
    zip -q "$ARCHIVE_PATH" $APP_NAME
popd > /dev/null

if [ -f "$ARCHIVE_PATH" ]; then
    echo "Binary packed to $ARCHIVE_PATH"
else
    echo "Could not pack binary"
    exit 1
fi