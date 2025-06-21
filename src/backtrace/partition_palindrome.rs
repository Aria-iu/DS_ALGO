pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut ret = vec![];
    let mut path = vec![];
    let sub_str: Vec<char> = s.chars().collect();

    backtracing(&sub_str, 0, &mut ret, &mut path);

    ret
}

fn backtracing(
    sub_str: &Vec<char>,
    start: usize,
    ret: &mut Vec<Vec<String>>,
    path: &mut Vec<String>,
) {
    //如果起始位置大于s的大小，说明找到了一组分割方案
    if start >= sub_str.len() {
        ret.push(path.clone());
        return;
    }

    for i in start..sub_str.len() {
        if !is_palindrome(sub_str, start, i) {
            continue;
        }
        //如果是回文子串，则记录
        let s: String = sub_str[start..i + 1].into_iter().collect();
        path.push(s);

        //起始位置后移，保证不重复
        backtracing(sub_str, i + 1, ret, path);
        path.pop();
    }
}

fn is_palindrome(s: &Vec<char>, start: usize, end: usize) -> bool {
    let (mut start, mut end) = (start, end);

    while start < end {
        if s[start] != s[end] {
            return false;
        }

        start += 1;
        end -= 1;
    }

    true
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
                    assert_eq!(partition(k), expected);
                }
            )*
        }
    }
    combination_tests! {
        test_generate_4_2: ("aab".to_string(), vec![
                vec!["a","a","b"],
                vec!["aa","b"],
            ]),
    }
}
