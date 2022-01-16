#!/bin/bash
ROOT=$PWD
for d in */ ; do
    if [ "$d" != "pvrustlib/" ] && [ "$d" != "target/" ]; then
        cd "$ROOT/$d"
        ./build.sh
        mv *.tar "$ROOT/"
    fi
done
