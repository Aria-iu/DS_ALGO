pub fn comb_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }
    let mut i;
    let mut gap: usize = nums.len();

    // 先做一个大致的排序
    while gap > 0 {
        gap = (gap as f32 * 0.8) as usize;
        i = gap;
        while i < nums.len() {
            if nums[i - gap] > nums[i] {
                nums.swap(i - gap, i);
            }
            i += 1;
        }
    }

    let mut exchange = true;
    while exchange {
        exchange = false;
        i = 0;
        while i < nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                exchange = true;
            }
            i += 1;
        }
    }
}
