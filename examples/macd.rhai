fn buy() {
    if macd(/*min*/ 15, 34, 72, 17) > macd_signal(/*min*/ 15, 34, 72, 17) {
        // If I should buy
        if is_sold() {
            fiat_to_asset(balance_fiat())
        } else {
            0.0
        }
    } else {
        // If I should sell
        if is_bought() {
            balance_asset() * -1
        } else {
            0.0
        }
    }
}
