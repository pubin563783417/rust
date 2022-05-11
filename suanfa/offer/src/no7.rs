/*
给定一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a ，b ，c ，使得 a + b + c = 0 ？请找出所有和为 0 且 不重复 的三元组。

先排序 ，然后使用首尾双指针法
*/


use std::{cmp::Ordering, collections::HashSet};

pub fn run() {
    let nums = vec![-1,0,1,2,-1,-4];
    let res = js(nums);
    assert_eq!(res , vec![[-1,-1,2],[-1,0,1]]);
}

fn js(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    // 升序
    nums.sort_by(|x , y| {
        if x > y {
            Ordering::Greater
        }else if x < y {
            Ordering::Less
        }else {
            Ordering::Equal
        }
    });
    println!("nums : {:?}" ,&nums);
    let mut set : HashSet<Vec<i32>> = HashSet::new();
    let mut res : Vec<Vec<i32>> = Vec::new();
    for (n , v)in nums.iter().enumerate() {
        let target = 0 - v;
        let mut i = 0;
        let mut j = nums.len()-1;
        loop {
            if i >= j {
                break;
            }
            if n == i{
                i += 1;
                continue;
            }
            if n == j{
                j -= 1;
                continue;
            }
            if nums[i] + nums[j] > target {
                j -= 1;
            }else if nums[i] + nums[j] < target{
                i += 1;
            }else {
                let mut indexs = vec![*v , nums[i] , nums[j]];
                let inputs = indexs.clone();
                indexs.sort_by(|x , y| {
                    if x > y {
                        Ordering::Greater
                    }else if x < y {
                        Ordering::Less
                    }else {
                        Ordering::Equal
                    }
                });
                i += 1;
                if !set.contains(&indexs) {
                    res.push(inputs);
                    set.insert(indexs);
                    break;
                }
                
            }
        }
        println!("res : {:?}" , &res);
    }
    
    return res;
}