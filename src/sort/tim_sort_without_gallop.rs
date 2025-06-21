// 蒂姆排序 ： 未完成。
const MIN_MERGE: usize = 64;

struct SortStat<'a> {
    list: &'a mut [i32],
    runs: Vec<Run>,
    pos: usize,
}

#[derive(Debug, Copy, Clone)]
struct Run {
    pos: usize,
    len: usize,
}

struct MergeLo<'a> {
    list_len: usize,     // 待 排 序 集 合 长 度
    first_pos: usize,    // run1 的 起 始 位 置
    first_len: usize,    // run1 的长度
    second_pos: usize,   // run2 的 起 始 位 置
    dest_pos: usize,     // 排 序 结 果 的 下 标 位 置
    list: &'a mut [i32], // 待 排 序 集 合 的 部 分 区 间
    temp: Vec<i32>,      // 长 度 设 置 为 run1、run2 中 较 短 值
}

struct MergeHi<'a> {
    first_pos: isize,
    second_pos: isize,
    dest_pos: isize,
    list: &'a mut [i32],
    temp: Vec<i32>, // 临 时 存 储 ， 放 后 面 便 于 内 存 对 齐 优 化
}

// 计算 minrun， 实 际 范 围 为 [32, 64]
fn calc_minrun(len: usize) -> usize {
    let mut r = 0;
    let mut new_len = len;
    while new_len >= MIN_MERGE {
        r |= new_len & 1;
        new_len >>= 1;
    }
    new_len + r
}

// 计算 run 的 起 始 下 标 ， 并 将 逆 序 run 转 成 正 序
// fn count_run(list: &mut [i32]) -> usize {
//     let (ord, pos) = find_run(list);
//     if ord {
//         // 逆 序 转 正 序
//         list.split_at_mut(pos).0.reverse();
//     }
//     pos
// }
