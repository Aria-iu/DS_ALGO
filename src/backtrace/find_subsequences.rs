use std::collections::HashSet;

fn backtracking(
    result: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
    nums: &Vec<i32>,
    start_index: usize,
) {
    if path.len() > 1 {
        result.push(path.clone());
    }
    let len = nums.len();
    let mut uset: HashSet<i32> = HashSet::new();
    for i in start_index..len {
        if (!path.is_empty() && nums[i] < *path.last().unwrap()) || uset.contains(&nums[i]) {
            continue;
        }
        uset.insert(nums[i]);
        path.push(nums[i]);
        backtracking(result, path, nums, i + 1);
        path.pop();
    }
}

pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut path: Vec<i32> = Vec::new();
    backtracking(&mut result, &mut path, &nums, 0);
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
                    assert_eq!(find_subsequences(k), expected);
                }
            )*
        }
    }
    combination_tests! {
        test_generate_4_2: (vec![4, 7, 6, 7], vec![
                vec![4, 7], vec![4, 7, 7], vec![4, 6], vec![4, 6, 7], vec![7, 7], vec![6, 7]
            ]),
    }
}
