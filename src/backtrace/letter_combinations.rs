const MAPS: [&str; 10] = [
    "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
];

fn back_trace(result: &mut Vec<String>, s: &mut String, digits: &String, index: usize) {
    let len = digits.len();
    if len == index {
        result.push(s.to_string());
        return;
    }
    let digit = (digits.as_bytes()[index] - b'0') as usize;
    for i in MAPS[digit].chars() {
        s.push(i);
        back_trace(result, s, digits, index + 1);
        s.pop();
    }
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let mut res = vec![];
    let mut s = String::new();
    back_trace(&mut res, &mut s, &digits, 0);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! combination_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (k,expected) = $test_case;
                    assert_eq!(letter_combinations(k), expected);
                }
            )*
        }
    }
    combination_tests! {
        test_23: ("23".to_string(), vec![
            "ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"
        ]),
    }
}
