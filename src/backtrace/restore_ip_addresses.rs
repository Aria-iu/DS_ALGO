fn is_valid(s: &[char], start: usize, end: usize) -> bool {
    if start > end {
        return false;
    }
    if s[start] == '0' && start != end {
        return false;
    }
    let mut num = 0;
    for &c in s.iter().take(end + 1).skip(start) {
        if !('0'..='9').contains(&c) {
            return false;
        }
        if let Some(digit) = c.to_digit(10) {
            num = num * 10 + digit;
        }
        if num > 255 {
            return false;
        }
    }
    true
}

fn backtracking(
    result: &mut Vec<String>,
    s: &mut Vec<char>,
    start_index: usize,
    mut point_num: usize,
) {
    let len = s.len();
    if point_num == 3 {
        if is_valid(s, start_index, len - 1) {
            result.push(s.iter().collect::<String>());
        }
        return;
    }
    for i in start_index..len {
        if is_valid(s, start_index, i) {
            point_num += 1;
            s.insert(i + 1, '.');
            backtracking(result, s, i + 2, point_num);
            point_num -= 1;
            s.remove(i + 1);
        } else {
            break;
        }
    }
}

pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let len = s.len();
    if len < 4 || len > 12 {
        return result;
    }
    let mut s = s.chars().collect::<Vec<char>>();
    backtracking(&mut result, &mut s, 0, 0);
    result
}
