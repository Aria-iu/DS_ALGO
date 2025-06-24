pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for s in strs {
        let (mut one_num, mut zero_num) = (0, 0);
        for c in s.chars() {
            match c {
                '0' => zero_num += 1,
                '1' => one_num += 1,
                _ => (),
            }
        }
        for i in (zero_num..=m).rev() {
            for j in (one_num..=n).rev() {
                dp[i][j] = dp[i][j].max(dp[i - zero_num][j - one_num] + 1);
            }
        }
    }
    dp[m][n]
}
