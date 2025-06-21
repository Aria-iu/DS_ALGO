pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() <=1 {
        return true;
    }
    let mut i = 0;
    let mut bound = 0;
    while i<= bound{
        bound = (i + nums[i] as usize).max(bound);
        if bound >= nums.len() -1{
            return true;
        }
        i+=1;
    }
    false
}
