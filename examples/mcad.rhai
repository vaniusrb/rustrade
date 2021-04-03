fn buy() {
    if macd(15, 34, 72, 17) > macd_signal(15, 34, 72, 17) {
        // If I should buy
        if is_sold() {
            let fiat = balance_fiat();
            fiat_to_asset(fiat)
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
