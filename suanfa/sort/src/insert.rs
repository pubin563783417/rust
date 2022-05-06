/// 插入排序 针对有序序列很快
pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    while i < arr.len() {
        
        if i != 0 {
            let v = arr[i];
            let mut j = i-1;
            let mut index = j;
            loop {
                if arr[j] < v {
                    arr[j+1] = arr[j];
                    index = j;
                }else {
                    break;
                }
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            arr[index] = v;
        }
        
        i += 1;
    }
    return arr;

}