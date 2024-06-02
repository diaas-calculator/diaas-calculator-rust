#!/usr/bin/ksh

env=$1
NETWORK=

if [ "$env" == "" ]; then
    echo "missing env"
    exit 1
fi

if [ "$env" != localhost -a "$env" != "cc" ]; then
    echo "wrong env: $env"
    exit 1
fi

NETWORK="--network diaas-db"

docker run -v $(pwd):/tmp $NETWORK --rm --name pgloader dimitri/pgloader:latest pgloader /tmp/csv.load.$env
docker run -v $(pwd):/tmp $NETWORK --rm --name pgloader dimitri/pgloader:latest pgloader /tmp/csv.load.i18n.$env
