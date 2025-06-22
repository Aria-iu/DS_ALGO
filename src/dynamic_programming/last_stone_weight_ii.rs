pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let sum = stones.iter().sum::<i32>();
    let target = sum as usize / 2;
    let mut dp = vec![0; target + 1];
    for s in stones {
        for j in (s as usize..=target).rev() {
            dp[j] = dp[j].max(dp[j - s as usize] + s);
        }
    }
    sum - dp[target] * 2
}
