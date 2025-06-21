fn backtracking(
    result: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
    nums: &Vec<i32>,
    start_index: usize,
    used: &mut Vec<bool>,
) {
    result.push(path.clone());
    let len = nums.len();
    // if start_index >= len { return; }
    for i in start_index..len {
        // 判断上一个是不是和当前要选择的元素相同且已经处理过了，而不是在上一层处理的。
        if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
            continue;
        }
        path.push(nums[i]);
        used[i] = true;
        // i+1 向后选择
        backtracking(result, path, nums, i + 1, used);
        used[i] = false;
        path.pop();
    }
}

pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut path: Vec<i32> = Vec::new();
    let mut used = vec![false; nums.len()];
    // 排序
    nums.sort();
    backtracking(&mut result, &mut path, &nums, 0, &mut used);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! combination_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (k, expected) = $test_case;
                    assert_eq!(subsets_with_dup(k), expected);
                }
            )*
        }
    }
    combination_tests! {
        test_generate_4_2: (vec![1,2,2], vec![
                vec![], vec![1], vec![1, 2], vec![1, 2, 2], vec![2], vec![2, 2]
            ]),
    }
}
