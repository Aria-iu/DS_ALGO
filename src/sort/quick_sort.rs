pub fn quick_sort1(nums: &mut [i32], low: usize, high: usize) {
    if low < high {
        let split = partition(nums, low, high);
        if split > 1 {
            quick_sort1(nums, low, split - 1);
        }
        quick_sort1(nums, split + 1, high);
    }
}

fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let mut lm = low;
    let mut rm = high;
    loop {
        while lm <= rm && nums[lm] <= nums[low] {
            lm += 1;
        }

        while lm <= rm && nums[rm] >= nums[low] {
            rm -= 1;
        }
        if lm > rm {
            break;
        } else {
            nums.swap(lm, rm);
        }
    }
    nums.swap(low, rm);

    rm
}

pub fn quick_sort2(nums: &mut [i32], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let mut lm = low;
    let mut rm = high;
    while lm < rm {
        while lm < rm && nums[low] <= nums[rm] {
            rm -= 1;
        }
        while lm < rm && nums[lm] <= nums[low] {
            lm += 1;
        }
        nums.swap(lm, rm);
    }
    nums.swap(low, lm);

    if lm > 1 {
        quick_sort2(nums, low, lm - 1);
    }
    quick_sort2(nums, rm + 1, high);
}
