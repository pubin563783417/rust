/*
给定一个含有 n 个正整数的数组和一个正整数 target 。

找出该数组中满足其和 ≥ target 的长度最小的 连续子数组 [numsl, numsl+1, ..., numsr-1, numsr] ，并返回其长度。如果不存在符合条件的子数组，返回 0 。

使用双指针法
*/

use std::cmp::Ordering;

pub fn run() {
    let res = js(7,vec![2,3,1,2,4,3]);
    assert_eq!(res , 2);
}

fn js(target: i32, mut nums: Vec<i32>) -> i32 {
    
    let mut res = i32::MAX;
    let mut i = 0;
    let mut j = 0;
    while j <= nums.len()-1 {
        let arr = &nums[i..=j];
        let count : i32 = arr.iter().sum();
        if count >= target {
            let _res = (j-i + 1) as i32;
            if _res < res {
                res = _res ;
            }
            i += 1;
        }else {
            j += 1;
        }
    }
    return if res == i32::MAX {0}else{res};
}