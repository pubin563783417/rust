use crate::insert;
/// 希尔  对插入排序的优化，降低了平均时间复杂度
/// sort
pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut d = arr.len();// 步长
    loop {
        // 每次步长/2
        d /= 2;
        if d == 1 {
            // 使用插入排序
            arr = insert::sort(arr);
            break;
        }else {
            let mut i = 0;
            while i + d < arr.len() {
                if arr[i] < arr[i+d] {
                    let temp = arr[i];
                    arr[i] = arr[i+d];
                    arr[i+d] = temp;
                }
                i += 1;
            }
        }
    }
    return arr;
}