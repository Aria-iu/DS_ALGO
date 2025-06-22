pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if intervals.is_empty() {
        return res;
    }
    intervals.sort_by_key(|a| a[0]);
    res.push(intervals[0].clone());
    for interval in intervals.into_iter().skip(1) {
        let res_last_ele = res.last_mut().unwrap();
        if res_last_ele[1] >= interval[0] {
            res_last_ele[1] = interval[1].max(res_last_ele[1]);
        } else {
            res.push(interval);
        }
    }
    res
}

#[test]
fn test() {
    println!(
        "{:?}",
        merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
    );
}
