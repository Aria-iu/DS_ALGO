fn backtracking(
    result: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
    nums: &Vec<i32>,
    used: &mut Vec<bool>,
) {
    let len = nums.len();
    if path.len() == len {
        result.push(path.clone());
        return;
    }
    for i in 0..len {
        if used[i] == true {
            continue;
        }
        used[i] = true;
        path.push(nums[i]);
        backtracking(result, path, nums, used);
        path.pop();
        used[i] = false;
    }
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut path: Vec<i32> = Vec::new();
    let mut used = vec![false; nums.len()];
    backtracking(&mut result, &mut path, &nums, &mut used);
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
                    assert_eq!(permute(k), expected);
                }
            )*
        }
    }
    combination_tests! {
        test_generate_4_2: (vec![1,2,3], vec![
                vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]
            ]),
    }
}
