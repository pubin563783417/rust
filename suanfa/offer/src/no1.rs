// 给定两个整数 a 和 b ，求它们的除法的商 a/b ，
// 要求不得使用乘号 '*'、除号 '/' 以及求余符号 '%' 

pub fn run() {
    
    assert_eq!(js(10 , 2) , 5);
    assert_eq!(js(-9 , 100) , 0);
    assert_eq!(js(-9 , 100) , 0);
    assert_eq!(js(0 , -100) , 0);
    assert_eq!(js(-9 , 1) , -9);
    assert_eq!(js(-9 , -3) , 3);
    assert_eq!(js(i32::MAX , -1) , i32::MIN+1);
    assert_eq!(js(i32::MIN , -1) , i32::MAX);
}

fn js(a : i32 , b : i32) -> i32{
    if b == 0 {
        return 0;
    }
    if a == i32::MIN && b == -1 {
        return i32::MAX;
    }
    let sign = if ((a > 0) && (b > 0)) || ((a < 0) && (b < 0)) {1}else{-1};
    // 转化成统一符号 ，方便后面做减法操作，转成负数，因为如果是负数转正式，可能会越界，如-i32::MIN -> 2^31 > i32:MAX
    let mut a = if a > 0 {-a}else {a};
    let b = if b > 0 {-b}else {b};
    let mut res = 0;
    while b >= a {
        a -= b;
        res += 1;
    }
    return if sign > 0 {res}else{-res};
}