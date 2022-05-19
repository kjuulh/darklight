#!/bin/bash

docker_compose=docker-compose.dev.yml

docker compose -f $docker_compose up --no-start

docker compose -f $docker_compose start crdb-cert

sleep 5

docker compose  -f $docker_compose start roach-0

sleep 5

docker compose  -f $docker_compose start roach-1
docker compose  -f $docker_compose start roach-2
docker compose  -f $docker_compose start crdb-lb

sleep 5

docker compose  -f $docker_compose start crdb-init
