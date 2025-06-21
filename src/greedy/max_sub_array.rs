pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = i32::MIN;
    let mut curr = 0;
    for n in nums.iter() {
        curr += n;
        max_sum = max_sum.max(curr);
        curr = curr.max(0);
    }
    max_sum
}

#[test]
fn test() {
    println!("{:?}", max_sub_array(vec![0, 0]));
}
