fn buy() {
    if macd(15, 34, 72, 17) > macd_signal(15, 34, 72, 17) {
        true
    } else {
        false
    }
}
