// dp[i][j] = dp[i - 1][j] + dp[i][j - coins[i]]
// 转化为一维：dp[j] += dp[j - coins[i]]
pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![0; amount + 1];
    dp[0] = 1;
    for coin in coins {
        // 这里并不需要考虑遍历顺序
        for j in coin as usize..=amount {
            dp[j] += dp[j - coin as usize];
        }
    }
    dp[amount]
}
