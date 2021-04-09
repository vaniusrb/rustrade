#!/bin/bash
cargo +nightly run --release -- -y BTCUSDT -m 15 -s "2020-12-21 00:00:00" -e "2020-12-23 23:45:00" script-back-test --file examples/macd.rhai
