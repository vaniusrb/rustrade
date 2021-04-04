#!/bin/bash
docker run -p 5050:80 \
    --name pgadmin4 \
    -e 'PGADMIN_DEFAULT_EMAIL=vanius@gmail.com' \
    -e 'PGADMIN_DEFAULT_PASSWORD=password' \
    -d dpage/pgadmin4
    --network host \
