pub fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 2] + dp[i - 1];
    }
    dp[n]
}
