pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut path = vec![];
    backtrace(&mut res, &mut path, n, k, 0, 1);
    res
}

fn backtrace(
    res: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
    target_num: i32,
    k: i32,
    sum: i32,
    start_index: i32,
) {
    if sum > target_num {
        return;
    }
    let len = path.len() as i32;
    if len == k {
        if sum == target_num {
            res.push(path.to_vec());
        }
        return;
    }
    for i in start_index..=9 - (k - len) + 1 {
        path.push(i);
        backtrace(res, path, target_num, k, sum + i, i + 1);
        path.pop();
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
                    let (k, n, expected) = $test_case;
                    assert_eq!(combination_sum3(k, n), expected);
                }
            )*
        }
    }
    combination_tests! {
        test_generate_4_2: (2, 4, vec![
                vec![1,3],
            ]),
    }
}
