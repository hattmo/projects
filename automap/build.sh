#/bin/bash

cargo build --release
if [[ $1 -eq 0 ]]; then
    docker build . -t hattmo/automap
fi
