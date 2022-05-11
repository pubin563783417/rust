/*
给定一个已按照 升序排列  的整数数组 numbers ，请你从数组中找出两个数满足相加之和等于目标数 target 。

函数应该以长度为 2 的整数数组的形式返回这两个数的下标值。numbers 的下标 从 0 开始计数 ，所以答案数组应当满足 0 <= answer[0] < answer[1] < numbers.length 。

假设数组中存在且只存在一对符合条件的数字，同时一个数字不能使用两次。

初始时两个指针分别指向第一个元素位置和最后一个元素的位置。每次计算两个指针指向的两个元素之和，并和目标值比较。如果两个元素之和等于目标值，则发现了唯一解。
如果两个元素之和小于目标值，则将左侧指针右移一位。如果两个元素之和大于目标值，则将右侧指针左移一位。移动指针之后，重复上述操作，直到找到答案。

*/
pub fn run() {
    let indexs = js(vec![1 , 2 ,5 , 7 , 10 , 18] , 12);
    assert_eq!(indexs , vec![1 , 4]);
}

fn js(numbers : Vec<i32> , target : i32 ) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len()-1;
    loop {
        if numbers[i] + numbers[j] > target {
            j -= 1;
        }else if numbers[i] + numbers[j] < target{
            i += 1;
        }else {
            break;
        }
    }
    return vec![i as i32,j as i32];
}

