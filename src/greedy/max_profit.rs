pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 1..prices.len() {
        result += (prices[i] - prices[i - 1]).max(0);
    }
    result
}
