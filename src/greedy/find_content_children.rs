pub fn find_content_children(mut children: Vec<i32>, mut cookies: Vec<i32>) -> i32 {
    children.sort();
    cookies.sort();

    let (mut child, mut cookie) = (0, 0);
    while child < children.len() && cookie < cookies.len() {
        // 优先选择最小饼干喂饱孩子
        if children[child] <= cookies[cookie] {
            child += 1;
        }
        cookie += 1;
    }
    child as i32
}
