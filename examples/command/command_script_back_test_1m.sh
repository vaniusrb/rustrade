#!/bin/bash
cargo run --release -- -y BTCUSDT -m 1 -s "2020-12-01 00:00:00" -e "2020-12-31 23:45:00" script-back-test --file examples/macd.rhai
