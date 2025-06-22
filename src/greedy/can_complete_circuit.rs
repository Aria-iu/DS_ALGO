pub fn can_complete_circuit1(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut cur_sum = 0;
    let mut min = i32::MAX;
    for i in 0..gas.len() {
        let rest = gas[i] - cost[i];
        cur_sum += rest;
        if cur_sum < min {
            min = cur_sum;
        }
    }
    if cur_sum < 0 {
        return -1;
    }
    if min > 0 {
        return 0;
    }
    for i in (0..gas.len()).rev() {
        let rest = gas[i] - cost[i];
        min += rest;
        if min >= 0 {
            return i as i32;
        }
    }
    -1
}

pub fn can_complete_circuit2(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut cur_sum = 0;
    let mut total_sum = 0;
    let mut start = 0;
    for i in 0..gas.len() {
        cur_sum += gas[i] - cost[i];
        total_sum += gas[i] - cost[i];
        if cur_sum < 0 {
            start = i + 1;
            cur_sum = 0;
        }
    }
    if total_sum < 0 {
        return -1;
    }
    start as i32
}

#[test]
fn test() {
    println!(
        "{:?}",
        can_complete_circuit2(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
    );
}
