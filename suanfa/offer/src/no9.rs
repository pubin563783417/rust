/*
给定一个正整数数组 nums和整数 k ，请找出该数组内乘积小于 k 的连续的子数组的个数。
1.找到最大的组合 2.最大组合的元素个数就是新增的无重复子数组个数
*/


pub fn run() {
    let res = js(19,vec![10,9,10,4,3,8,3,3,6,2,10,10,9,3]);
    assert_eq!(res , 18);
}

fn js(target: i64,  nums: Vec<i64>) -> i64 {
    
    let mut res = 0;
    let mut sums = Vec::<i64>::new();
    for (i , num) in nums.iter().enumerate() {
        if i == 0 {
            sums.push(*num);
        }else {
            sums.push(sums[i-1].saturating_mul(*num));
        }
    }
    println!("sums :{:?}" , sums);

    for i in 0..sums.len() {
        let mut j = sums.len()-1;
        while j >= i && j != usize::MAX{
            let sum = if i == 0 {
                sums[j]
            }else{sums[j]/sums[i-1]};
            if sum >= target {
                j = j.checked_sub(1).unwrap_or(usize::MAX);
                
            }else {
                res += j - i + 1;
                break;
            }
        }
    }
    
    return res as i64;
}