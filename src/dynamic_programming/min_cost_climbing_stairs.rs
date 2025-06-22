// dp[i] = min(dp[i-1]+cost[i-1],dp[i-2]+cost[i-2])
// dp[0] = 0; dp[1] = 0;
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut dp = vec![0; cost.len() + 1];
    for i in 2..=cost.len() {
        dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
    }
    dp[cost.len()]
}
