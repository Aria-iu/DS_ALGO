pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
    nums.sort_by_key(|b| std::cmp::Reverse(b.abs()));
    for v in nums.iter_mut() {
        if *v < 0 && k > 0 {
            *v *= -1;
            k -= 1;
        }
    }
    if k % 2 == 1 {
        *nums.last_mut().unwrap() *= -1;
    }
    nums.iter().sum()
}

#[test]
fn test() {
    println!(
        "{:?}",
        largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2)
    );
}
