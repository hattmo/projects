#!/bin/bash

base=$1
start=$2
end=$3

until [ $start -gt $end ]
do
if [ "$(ping -q -n -c 1 -w 1 $base.$start | cut -d ',' -f 2 -s | cut -d " " -f 2)" = "1" ]
then
echo $base.$start
fi
start=$((start+1))
done
