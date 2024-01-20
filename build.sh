#!/bin/sh

# Colours
RED='\e[1;31m'
NC='\e[0m'

echo -n "This is meant to be run in the project root. Continue? (Y/n) "
read CONTINUE

case $CONTINUE in
  n)
    exit 0
  ;;

  N)
    exit 0
  ;;
esac

MODE="debug"
if [ ! -z "$1" ]; then
  if [ "$1" != "release" ]; then
    echo -e "${RED}ERROR:${NC} Unknown argument: ${1}, currently known arguments: release"
    exit 1
  fi

  MODE="release"
fi

clean_old() {
  rm -rf ./target/$MODE/*
}

rmd() {
  rm -rfv ./target/$MODE/$1
}

clean_new() {
  cp -r ./assets ./target/$MODE/
  cp ./examples/* ./target/$MODE/examples

  rmd build
  rmd deps
  rmd incremental
  rmd lymap.d
}

build() {
  clean_old

  if [ "$MODE" == "release" ]; then
    cargo build --release
  else
    cargo build
  fi

  clean_new
}

build
