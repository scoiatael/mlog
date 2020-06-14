#!/usr/bin/env bash

SELF="$(dirname $0)"

$SELF/../../target/release/mlog $*
