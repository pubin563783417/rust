
pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    while i < arr.len() {
        
        let mut j = i;
        let mut max_index = j;
        while j < arr.len() {
            if arr[j] > arr[max_index] {
                max_index = j;
            }
            j+=1;
        }
        let temp = arr[i];
        arr[i] = arr[max_index];
        arr[max_index] = temp;
        i+=1;
    }
    return arr;
}