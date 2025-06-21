fn handle(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut index = 0;
    for i in 0..nums.len() {
        if i > 0 && nums[i] != nums[i - 1] {
            res.push(nums[i]);
        } else if i == 0 {
            res.push(nums[i]);
        }
    }
    res
}

pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 1;
    }
    let r = handle(nums);
    if r.len() == 1 {
        return 1;
    }
    println!("{:?}", r);

    let mut res = 1;
    let mut rise = r[1] > r[0];
    for i in 1..r.len() {
        if r[i] > r[i - 1] {
            if rise {
                rise = true;
                if i == 1 {
                    res += 1;
                }
            } else {
                rise = true;
                res += 1;
            }
        } else {
            if rise {
                rise = false;
                res += 1;
            } else {
                rise = false;
                if i == 1 {
                    res += 1;
                }
            }
        }
    }
    println!("{:?}", res);
    0
}

#[test]
fn test() {
    // wiggle_max_length(vec![1,2,2,5]);
    // wiggle_max_length(vec![1,7,4,9,2,5]);
    // wiggle_max_length(vec![1,17,5,10,13,15,10,5,16,8]);
    // wiggle_max_length(vec![1,2,3,4,5,6,7,8,9]);

    wiggle_max_length(vec![0, 0]);
}
