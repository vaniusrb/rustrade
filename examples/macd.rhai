fn buy() {
    let result = 0.0;
    if rsi(15, 14) < 30 && macd(/*min*/ 15, 34, 72, 17) > macd_signal(/*min*/ 15, 34, 72, 17) {
        // If I should buy
        if is_sold() {
            let fiat = balance_fiat();
            result = fiat_to_asset(fiat);
        }
    };
    if rsi(15, 14) > 70 && macd(/*min*/ 15, 34, 72, 17) <= macd_signal(/*min*/ 15, 34, 72, 17) {
        // If I should sell
        if is_bought() {
            result = balance_asset() * -1;
        }
    };
    result
}
