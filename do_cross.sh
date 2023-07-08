#!/usr/bin/env bash
set -e

if [ -z "$1" ]
then
    arg="build"
else
    arg=$@
fi
cross ${arg} --target x86_64-pc-windows-gnu
