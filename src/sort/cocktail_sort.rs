use core::num;

pub fn cocktail_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let mut bubble = true;
    let len = nums.len();
    // 双端排序，每次不用再排最前面和最后面的位置
    for i in 0..(len >> 1) {
        if bubble {
            bubble = false;

            for j in i..(len - i - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    bubble = true;
                }
            }

            for j in (i + 1..=(len - i - 1)).rev() {
                if nums[j] < nums[j - 1] {
                    nums.swap(j, j - 1);
                    bubble = true;
                }
            }
        } else {
            break;
        }
    }
}
