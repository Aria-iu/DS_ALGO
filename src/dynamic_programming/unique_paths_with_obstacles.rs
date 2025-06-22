pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m: usize = obstacle_grid.len();
    let n: usize = obstacle_grid[0].len();
    // 起始或终止是障碍物
    if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
        return 0;
    }
    let mut dp = vec![vec![0; n]; m];
    // 第一列有障碍物，第一列障碍物下面不可达
    for i in 0..m {
        if obstacle_grid[i][0] == 1 {
            break;
        } else {
            dp[i][0] = 1;
        }
    }
    // 第一行有障碍物，第一行右边不可达
    for j in 0..n {
        if obstacle_grid[0][j] == 1 {
            break;
        } else {
            dp[0][j] = 1;
        }
    }
    // 正常处理
    for i in 1..m {
        for j in 1..n {
            if obstacle_grid[i][j] == 1 {
                // 如果当前是障碍物，不用管，依旧为0
                continue;
            }
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }
    dp[m - 1][n - 1]
}
