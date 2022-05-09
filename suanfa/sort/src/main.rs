mod insert;
mod rand_num_array;
mod xier;
mod pick;
mod maopao;
mod merge;
mod fast;
mod heap;
mod counting;
mod bucket;
mod radix;

use std::time::SystemTime;

fn main() {
    let array = rand_num_array::gen_rand_array(20, 500);
    // array = vec![119, 84, 247, 441, 340, 273, 320, 401, 100];
    println!("ori array : \n{:?}", array);
    let sys_time = SystemTime::now();
    let array = radix::sort(array);
    println!("\n\n");
    println!("sorted array : \n{:?}", array);
    
    let new_sys_time = SystemTime::now();
    let difference = new_sys_time.duration_since(sys_time)
    .expect("Clock may have gone backwards");
    println!("\n\nsorted {}! 耗时间：{:?}" , if is_sorted(array) { "success"} else {"faiture"} , difference);
}

fn is_sorted(arr: Vec<i32>) -> bool{
    let mut i = 0;
    let mut last = *arr.first().unwrap();
    while i < arr.len() {
        if arr[i] > last {
            return false;
        }
        last = arr[i];
        i += 1;
    }
    return true;
}
