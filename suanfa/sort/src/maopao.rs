// 冒泡
pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    while i < arr.len() {
        let mut j = 0;
        while j < arr.len() - i - 1 {
            let _1 = arr[j];
            let _2 = arr[j + 1];
            if _1 < _2 {
                arr[j] = _2;
                arr[j+1] = _1;
            }
            j += 1;
        }
        i += 1;
    }
    return arr;
}
