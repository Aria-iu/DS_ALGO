pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut path: Vec<i32> = Vec::new();
    let mut used: Vec<bool> = vec![false; candidates.len()];
    let mut candidates = candidates;
    // 排序
    candidates.sort();
    backtracking(&mut result, &mut path, &candidates, target, 0, 0, &mut used);
    result
}

fn backtracking(
    result: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
    candidates: &Vec<i32>,
    target: i32,
    mut sum: i32,
    start_index: usize,
    used: &mut Vec<bool>,
) {
    if sum == target {
        result.push(path.to_vec());
        return;
    }
    for i in start_index..candidates.len() {
        if sum + candidates[i] <= target {
            // 如果 当前选择的数和上一次一样，而且遍历到当前时，上一个还没有使用过，也就是说上一个已经处理过了。
            // 那么，就直接跳过当前的。
            // 例如：[2,2,3]，处理完第一个2之后used[0] = false，到第二个2时，情况和第一个一样，不用再处理了。
            if i > 0 && candidates[i] == candidates[i - 1] && used[i - 1] == false {
                continue;
            }
            sum += candidates[i];
            path.push(candidates[i]);
            // 记录已经用过的
            used[i] = true;
            backtracking(result, path, candidates, target, sum, i + 1, used);
            used[i] = false;
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
                    assert_eq!(combination_sum2(k,v), expected);
                }
            )*
        }
    }
    combination_tests! {
        test_7: (vec![10,1,2,7,6,1,5],8,vec![
                vec![1, 1, 6],
                vec![1, 2, 5],
                vec![1, 7],
                vec![2, 6],
            ]),
    }
}
