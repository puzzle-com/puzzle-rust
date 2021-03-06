#!/bin/bash

echo 'waiting on healthcheck' >&2

for _ in {1..100}; do
    if [[ "$(curl -s -o /dev/null -w '%{http_code}' http://localhost:3030/status)" == "200" ]]; then
        exit 0
    fi
    sleep 5
done

echo 'ERROR: waiting timeout' >&2
exit 1
