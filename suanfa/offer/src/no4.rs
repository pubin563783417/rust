/* 
给你一个整数数组 nums ，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次 。请你找出并返回那个只出现了一次的元素。
关键：虑答案的第 ii 个二进制位（ii 从 00 开始编号），它可能为 00 或 11。对于数组中非答案的元素，每一个元素都出现了 33 次，对应着第 ii 个二进制位的 33 个 00 或 33 个 11，无论是哪一种情况，它们的和都是 33 的倍数（即和为 00 或 33）。因此：

答案的第 ii 个二进制位就是数组中所有元素的第 ii 个二进制位之和除以 33 的余数。
*/

pub fn run() {
    let target = js(vec![0b1011 ,0b0111,0b0111,0b0111,0b1110,0b1110,0b1110]);
    assert_eq!(target , 0b1011);
}
fn js(nums : Vec<i32>) -> i32{
    let mut target :i32 = 0b00000000;
    for i in 0..32 {
        let mut bit_total = 0;
        for num in nums.iter() {
            bit_total += (num >> i) & 1;
        }
        target |= ((bit_total%3) as i32) << i;
    }
    return target;
}