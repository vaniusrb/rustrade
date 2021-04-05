#!/bin/bash
# docker run --network="host" --name postgres-server -e POSTGRES_PASSWORD=password -d postgres

docker stop postgres-server
docker rm postgres-server

docker run -p 5432:5432 \
--name postgres-server \
-e POSTGRES_PASSWORD=password \
-d postgres
# --network host \

# login pgadmin:
#   vanius@gmail.com
#   password

# login bd:
#   localhost
#   postgres
#   password
