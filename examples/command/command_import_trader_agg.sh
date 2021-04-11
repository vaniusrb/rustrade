#!/bin/bash
cargo +nightly run --release -- -y BTCUSDT -m 15 -s "2021-04-09 00:00:00" trade import
cargo +nightly run --release -- -y BTCUSDT -m 15 -s "2021-04-09 00:00:00" trade check
