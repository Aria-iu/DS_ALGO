pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut queue = vec![];
    people.sort_by(|a, b| {
        if a[0] == b[0] {
            return a[1].cmp(&b[1]);
        }
        b[0].cmp(&a[0])
    });
    queue.push(people[0].clone());
    for v in people.iter().skip(1) {
        queue.insert(v[1] as usize, v.clone());
    }
    queue
}

#[test]
fn test() {
    println!(
        "{:?}",
        reconstruct_queue(vec![
            vec![6, 0],
            vec![5, 0],
            vec![4, 0],
            vec![3, 2],
            vec![2, 2],
            vec![1, 4]
        ])
    );
}
