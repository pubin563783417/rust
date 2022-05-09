
// 计数排序 ，首先创建一个和排序最大数大小相等的数组 ，然后依次将数据放进去


pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut max = arr[0];
    while i < arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
        i += 1;
    }
    let mut list : Vec<i32> = (0..max).map(
        |_| -1
    ).collect();
    let mut i = 0;
    while i < arr.len() {
        list[(arr[i]-1) as usize] = arr[i];
        i += 1;
    }
    let mut new : Vec<i32> = Vec::new();
    for v in list.into_iter().rev() {
        if v > -1 {
            new.push(v);
        }
    }
    return new;
}