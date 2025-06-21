fn backtracking(
    result: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
    nums: &Vec<i32>,
    start_index: usize,
) {
    result.push(path.clone());
    let len = nums.len();
    // if start_index >= len { return; }
    for i in start_index..len {
        path.push(nums[i]);
        backtracking(result, path, nums, i + 1);
        path.pop();
    }
}

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
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
                    assert_eq!(subsets(k), expected);
                }
            )*
        }
    }
    combination_tests! {
        test_generate_4_2: (vec![1,2,3], vec![
                vec![], vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 3], vec![2], vec![2, 3], vec![3]
            ]),
    }
}
