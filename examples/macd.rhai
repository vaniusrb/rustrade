fn run() {
    set_change_trend_buy(
        rsi(15, 14) < 30 && macd(15, 34, 72, 17) > macd_signal(15, 34, 72, 17)
    );

    set_change_trend_sell(
        rsi(15, 14) > 70 && macd(15, 34, 72, 17) <= macd_signal(15, 34, 72, 17)
    );

    let g = gain_perc();
    if g > 2.0 {
        log("stop gain: " + g);
        show_min_man();
        sell(balance_asset());
    }

    if g < -3.0 {
        log("stop loss: " + g);
        show_min_man();
        sell(balance_asset());
    }

    if change_trend_buy() && is_sold() {
        let fiat = balance_fiat();
        let asset = fiat_to_asset(fiat);
        //log("change_trend_buy");
        buy(asset);
    }

    if change_trend_sell() && is_bought() {
        //log("change_trend_sell");
        sell(balance_asset());
    }
}

fn show_min_man() {
    log("min " + min(15, 4) + " max " + max(15, 4));
}