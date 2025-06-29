// dp[i]：分拆数字i，可以得到的最大乘积为dp[i]。
// dp[i] = max(dp[i], max((i - j) * j, dp[i - j] * j));
pub fn integer_break(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[2] = 1;
    for i in 3..=n {
        for j in 1..i - 1 {
            dp[i] = dp[i].max((i - j) * j).max(dp[i - j] * j);
        }
    }
    dp[n] as i32
}
