#!/bin/bash

docker rm -f automap
docker run -d --name automap -e TARGET="172.30.0.0/16 172.31.0.0/16" -p 3000:3000 automap