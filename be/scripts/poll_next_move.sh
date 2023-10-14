#!/bin/bash

if [ $# -ne 1 ]
then
  echo "Usage: $0 <port>"
  exit 1
fi

port=$1
max_wait=10
start_time=$(date +%s)

while [ $(($(date +%s) - $start_time)) -lt $max_wait ]
do
  response=$(curl -s -o /dev/null -w "%{http_code}" "localhost:$port/api/bot/next?level=Normal" \
    -H "content-type: application/json" \
    -d '{"cells":[0, 0, 0, 0, 0, 0, 0, 0, 0],"p1_mark":1,"bot_mark":-1,"empty_mark":0}')

  if [ $response -eq 200 ]
  then
    exit 0
  fi

  sleep 1
done

echo "Timed out waiting for answer status code 200"
exit 1
