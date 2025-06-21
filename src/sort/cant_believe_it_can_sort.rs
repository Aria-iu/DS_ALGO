pub fn cbic_sort1(nums: &mut [i32]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}
