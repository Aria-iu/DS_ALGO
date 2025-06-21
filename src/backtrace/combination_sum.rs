pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut path: Vec<i32> = Vec::new();
    backtracking(&mut result, &mut path, &candidates, target, 0, 0);
    result
}

fn backtracking(
    result: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
    candidates: &Vec<i32>,
    target: i32,
    mut sum: i32,
    start_index: usize,
) {
    if sum == target {
        result.push(path.to_vec());
        return;
    }

    for i in start_index..candidates.len() {
        if sum + candidates[i] <= target {
            path.push(candidates[i]);
            sum += candidates[i];
            backtracking(result, path, candidates, target, sum, i);
            sum -= candidates[i];
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! combination_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (k,v,expected) = $test_case;
                    assert_eq!(combination_sum(k,v), expected);
                }
            )*
        }
    }
    combination_tests! {
        test_7: (vec![2,3,6,7],7,vec![
                vec![2,2,3],
                vec![7],
            ]),
    }
}
