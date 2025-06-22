// 对一个数组分割出一个子集，子集和等于总和的一半
// 等价于将数组中的货物装满容量为总和一半的背包
pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum = nums.iter().sum::<i32>() as usize;
    if sum % 2 == 1 {
        return false;
    }
    let target = sum / 2;
    let mut dp = vec![0; target + 1];
    for n in nums {
        for j in (n as usize..=target).rev() {
            dp[j] = dp[j].max(dp[j - n as usize] + n)
        }
    }
    if dp[target] == target as i32 {
        return true;
    }
    false
}
