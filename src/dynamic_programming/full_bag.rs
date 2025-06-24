pub fn full_bag(weights: Vec<i32>, vals: Vec<i32>, room: i32) -> usize {
    let len = weights.len();
    let mut dp = vec![vec![0; (room + 1) as usize]; len];
    for i in weights[0]..=room {
        dp[0][i as usize] = dp[0][(i - weights[0]) as usize] + vals[0];
    }

    for i in 1..len {
        for j in 0..=room {
            if j < weights[i] {
                dp[i][j as usize] = dp[i - 1][j as usize];
            } else {
                dp[i][j as usize] =
                    dp[i - 1][j as usize].max(dp[i][(j - weights[i]) as usize] + vals[i]);
            }
        }
    }

    dp[len - 1][room as usize] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", full_bag(vec![1, 3, 4], vec![15, 20, 30], 4));
    }
}
