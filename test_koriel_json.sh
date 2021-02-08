#!/usr/bin/env bash

TESTDIR=/tmp/malgo_test

if [ ! -e $TESTDIR ]; then
  exit 255
fi

set -x # verbose output

for file in `ls $TESTDIR | grep '\.json$'`; do
  diff <(jq -S . $TESTDIR/$file) <(cat $TESTDIR/$file | cargo run | jq -S .)
done
