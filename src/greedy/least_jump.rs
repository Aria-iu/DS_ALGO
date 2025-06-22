pub fn least_jump(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }
    let mut cur_distance = 0;
    let mut ans = 0;
    let mut next_distance = 0;
    for (i, &n) in nums.iter().enumerate().take(nums.len() - 1) {
        next_distance = (n as usize + i).max(next_distance);
        if i == cur_distance {
            if cur_distance < nums.len() - 1 {
                ans += 1;
                cur_distance = next_distance;
                if next_distance >= nums.len() - 1 {
                    break;
                };
            } else {
                break;
            }
        }
    }
    ans
}

#[test]
fn test() {
    println!("{:?}", least_jump(vec![2, 3, 1, 1, 4]));
}
