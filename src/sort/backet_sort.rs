use std::fmt::Debug;

// 桶定义
// hasher 是一个函数，计算时传入
// values 是数据容器，保存数据
struct Bucket<H, T> {
    hasher: H,
    values: Vec<T>,
}

impl<H, T> Bucket<H, T> {
    fn new(hasher: H, value: T) -> Bucket<H, T> {
        Bucket {
            hasher,
            values: vec![value],
        }
    }
}

// 桶排序，Debug 特性是为了打印 T
fn bucket_sort<H, T, F>(nums: &mut [T], hasher: F)
where
    H: Ord,
    T: Ord + Clone + Debug,
    F: Fn(&T) -> H,
{
    let mut buckets: Vec<Bucket<H, T>> = Vec::new();

    for value in nums.iter() {
        let hasher = hasher(&value);

        // 对桶中数据二分搜索并排序
        match buckets.binary_search_by(|bucket| bucket.hasher.cmp(&hasher)) {
            Ok(index) => buckets[index].values.push(value.clone()),
            Err(index) => buckets.insert(index, Bucket::new(hasher, value.clone())),
        }
    }

    // 拆桶，将所有排序数据融合到一个 Vec
    let ret = buckets
        .into_iter()
        .flat_map(|mut bucket| {
            bucket.values.sort();
            bucket.values
        })
        .collect::<Vec<T>>();

    nums.clone_from_slice(&ret);

    // println!("sorted nums: {:?}", ret);
}
