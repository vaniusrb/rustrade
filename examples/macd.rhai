fn trend() {
    if rsi(15, 14) < 30 // && macd(15, 34, 72, 17) > macd_signal(15, 34, 72, 17)
    {
        return 1;
    }
    if rsi(15, 14) > 70 // && macd(15, 34, 72, 17) <= macd_signal(15, 34, 72, 17)
    {
        return -1;
    }
    0
}

fn run() {
    let g = gain_perc();
    if g > 2.0 {
        log("stop gain: " + g);
        return balance_asset();
    }
    if g < -3.0 {
        log("stop loss: " + g);
        return balance_asset();
    }
    0.0
}

fn change_trend(t) {
    if t > 0 && is_sold() {
        let fiat = balance_fiat();
        let asset = fiat_to_asset(fiat);
        return asset;
    }
    if t < 0 && is_bought() {
        return balance_asset() * -1;
    }
    0.0
}

