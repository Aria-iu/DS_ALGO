pub fn partition_labels(s: String) -> Vec<i32> {
    let mut hash = vec![0; 26];
    for (i, &c) in s.as_bytes().iter().enumerate() {
        hash[(c - b'a') as usize] = i;
    }
    let mut res = vec![];
    let (mut left, mut right) = (0, 0);
    for (i, &c) in s.as_bytes().iter().enumerate() {
        right = right.max(hash[(c - b'a') as usize]);
        if i == right {
            res.push((right - left + 1) as i32);
            left = i + 1;
        }
    }
    res
}

#[test]
fn test() {
    println!(
        "{:?}",
        partition_labels("ababcbacadefegdehijhklij".to_string())
    );
}
