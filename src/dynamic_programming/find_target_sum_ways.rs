// 给定一个非负整数数组，a1, a2, ..., an, 和一个目标数，S。现在你有两个符号 + 和 -。
// 对于数组中的任意一个整数，你都可以从 + 或 -中选择一个符号添加在前面。
// 返回可以使最终数组和为目标数 S 的所有添加符号的方法数。
// 有 left组合 - right组合 = target。
// left + right = sum，而sum是固定的。right = sum - left
// left - (sum - left) = target 推导出 left = (target + sum)/2 。
// target是固定的，sum是固定的，left就可以求出来。
// 此时问题就是在集合nums中找出和为left的组合。
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let sum = nums.iter().sum::<i32>();
    if target.abs() > sum {
        return 0;
    }
    if (target + sum) % 2 == 1 {
        return 0;
    }
    let size = (sum + target) as usize / 2;
    let mut dp = vec![0; size + 1];
    dp[0] = 1;
    for n in nums {
        for s in (n as usize..=size).rev() {
            dp[s] += dp[s - n as usize];
        }
    }
    dp[size]
}
