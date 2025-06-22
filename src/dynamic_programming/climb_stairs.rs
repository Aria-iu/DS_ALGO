// 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
// dp[i] = dp[i-1] + dp[i-2]
pub fn climb_stairs(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let (mut a, mut b, mut f) = (1, 1, 0);
    for _ in 2..=n {
        f = a + b;
        a = b;
        b = f;
    }
    f
}
