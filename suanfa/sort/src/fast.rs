use rand::Rng;

// 快速排序 ， 平均时间 O(n log n)  ，空间  O(1)。
// 分治 然后 选取基准 ，小的放右边，大的放左边
pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    _sort(&mut arr[..]);
    return arr;
}

fn _sort(arr : &mut [i32]) {
    if arr.len() == 0 || arr.len() == 1 {
        return;
    }
    // 基准排序
    let base_value = arr[0];
    let mut mark = 0;
    let mut i = 0;
    while i < arr.len() {
        if arr[i] > base_value {
            let temp = arr[i];
            arr[i] = arr[mark];
            arr[mark] = temp;
            mark += 1;
        }
        i += 1;
    }
    //  基准值是一个最大或者最小
    if mark == 0 || mark == arr.len() {
        mark = 1;
    }
    
    // 递归
    _sort(&mut arr[0..mark]);
    _sort(&mut arr[mark..]);
}