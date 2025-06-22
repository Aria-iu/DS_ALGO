// dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - weight[i]] + value[i]);
pub fn zero_one_bag(weights: Vec<i32>, vals: Vec<i32>, room: i32) -> () {
    let len = weights.len();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; (room + 1) as usize]; len];
    for j in 0..=room {
        if j < weights[0] {
            dp[0][j as usize] = 0;
        } else {
            dp[0][j as usize] = vals[0];
        }
    }

    for i in 1..len {
        for j in 0..=room {
            if j - weights[i] >= 0 {
                dp[i][j as usize] =
                    dp[i - 1][j as usize].max(dp[i - 1][(j - weights[i]) as usize] + vals[i]);
            } else {
                dp[i][j as usize] = dp[i - 1][j as usize];
            }
        }
    }
    println!("{:?}", dp);
}

// 可以将dp写为一维数组，滚动更新
// dp[j] = max(dp[j],dp[j-weigjt[i]]+vals[i])
// 二维dp，dp[i][j]都是通过上一层即dp[i - 1][j - x]计算而来，本层的dp[i][j]并不会被覆盖
// 一维dp，dp[i][j]依赖于本层的dp[i][j - x]，所以要从右到左更新
pub fn zero_one_bag_oned(weights: Vec<i32>, vals: Vec<i32>, room: i32) -> () {
    let len = weights.len();
    let mut dp: Vec<i32> = vec![0; (room + 1) as usize];
    for i in 0..weights.len() {
        for j in (weights[i]..=room).rev() {
            let j = j as usize;
            dp[j] = dp[j].max(dp[j - weights[i] as usize] + vals[i]);
        }
    }
    println!("{:?}", dp);
}

#[test]
fn test() {
    zero_one_bag_oned(vec![1, 3, 4], vec![15, 20, 30], 4);
}
