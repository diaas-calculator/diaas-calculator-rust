#!/usr/bin/ksh
NETWORK="--network diaas-db"

usage(){
    echo "$0 [-e <env>] [-f <file>]"
    echo "-e : load all files for the given environment (cc or localhost)"
    echo "-f : load a single file (provide full relative filename)"
    echo "Example 1: $0 -e localhost"
    echo "Example 2: $0 -f csv_load_food.localhost"
    exit 1
}

while getopts e:f: opt ; do
  case $opt in
    e) env="$OPTARG";;
    f) file="$OPTARG";;
    ?) echo "unknown option: $opt"; usage; exit 1;;
  esac
done
shift $(($OPTIND - 1))


if [ "$env" == "" -a "$file" == "" ]; then
    echo "missing env or file"
    usage
fi

if [ "$env" != "" -a "$file" != "" ]; then
    echo "provide only one of env (-e) or file (-f)"
    usage
fi

if [ "$file" != "" ]; then
    if [ ! -f $(pwd)/$file ]; then
        echo "cannot find file $(pwd)/$file"
        exit 1
    fi
    docker run -v $(pwd):/tmp $NETWORK --rm --name pgloader dimitri/pgloader:latest pgloader /tmp/$file
else
    if [ "$env" != localhost -a "$env" != "cc" ]; then
        echo "wrong env: $env"
        exit 1
    fi
    docker run -v $(pwd):/tmp $NETWORK --rm --name pgloader dimitri/pgloader:latest pgloader /tmp/csv_load_food.$env
    docker run -v $(pwd):/tmp $NETWORK --rm --name pgloader dimitri/pgloader:latest pgloader /tmp/csv_load_food_i18n.$env
    docker run -v $(pwd):/tmp $NETWORK --rm --name pgloader dimitri/pgloader:latest pgloader /tmp/csv_load_mix.$env
    docker run -v $(pwd):/tmp $NETWORK --rm --name pgloader dimitri/pgloader:latest pgloader /tmp/csv_load_mix_food.$env
fi





