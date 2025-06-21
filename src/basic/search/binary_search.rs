use core::num;

pub fn binary_search1(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    while low <= high && !found {
        // let mid = (low+high)>>1;
        let mid = low + ((high - low) >> 1);
        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    found
}

pub fn binary_search2(nums: &[i32], num: i32) -> bool {
    if nums.len() == 0 {
        return false;
    }
    let mid = nums.len() >> 1;
    if nums[mid] == num {
        return true;
    } else if num < nums[mid] {
        return binary_search2(&nums[..mid], num);
    } else {
        return binary_search2(&nums[mid + 1..], num);
    }
}

// O(loglog(n))
pub fn interpolation_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() {
        return false;
    }
    let mut low = 0usize;
    let mut high = nums.len() - 1;

    loop {
        let low_val = nums[low];
        let high_val = nums[high];

        if high <= low || target < low_val || target > high_val {
            break;
        }

        let offset = (target - low_val) * (high - low) as i32 / (high_val - low_val);
        let interpolant = low + offset as usize;

        if nums[interpolant] > target {
            high = interpolant - 1;
        } else if nums[interpolant] < target {
            low = interpolant + 1;
        } else {
            break;
        }
    }

    target == nums[high]
}

pub fn exponential_search(nums: &[i32], target: i32) -> bool {
    let size = nums.len();
    if size == 0 {
        return false;
    }
    let mut high = 1usize;
    while high < size && nums[high] < target {
        high <<= 1;
    }
    // 上 界 的 一 半 一 定 可 以 作 为 下 界
    let low = high >> 1;
    // 使 用 前 面 实 现 的 二 分 查 找
    binary_search2(&nums[low..size.min(high + 1)], target)
}
