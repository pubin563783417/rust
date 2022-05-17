/*
给定一个非空字符串 s，请判断如果 最多 从字符串中删除一个字符能否得到一个回文字符串。 ”avcvac“

1.看看左指针的下一个元素是否和右指针元素相等，相等的话左指针可以直接跳过当前索引，相当于删除一个字符
2.如果第一个方法失败，则尝试右指针的下一个元素是否和左指针当前元素相等，如果成功直接返回true。
3.两种方法要分开尝试
 */

 pub fn run() {
    assert!(js("cuppucu".to_string()));
 }

 fn js(s: String) -> bool {
     let chars = s.chars().fold(vec![], 
        |mut x , y| {
            x.push(y);
            return x;
        }
    );
    if chars.len() <= 0 {
        return true;
    }
    println!("chars : {:?} , count : {}" , chars ,chars.len());
    let mut reset_flag = 0;
    let mut left = 0;
    let mut right = chars.len()-1;

    let mut store_reset_flag = 0;
    let mut store_left = 0;
    let mut store_right = 0;

    let mut step = 1;
    while left <= right {
        if chars[left] != chars[right] {
            if reset_flag == 1 {
                println!("left :{} {}  , right :{} {}" , left ,chars[left], right , chars[right]);

                if step == 2 {
                    println!("重试一次 {} {} {}" , store_reset_flag , store_left,store_right);
                    reset_flag = store_reset_flag;
                    left = store_left;
                    right = store_right;
                    
                    continue;
                }
                return false;
            }
            let way1 = chars[left + 1] == chars[right];
            let way2 = chars[left] == chars[right-1];
            println!("way1 :{} , way2 :{} ,step : {}" , way1 , way2 ,step);
            if way1 && way2 {
                store_reset_flag = reset_flag;
                store_left = left;
                store_right = right;

                reset_flag += 1;
                // 两种方式都可以 ， 先尝试第一种  ,如果失败再尝试第二种
                if step == 1 {
                    println!("+ left :{} , right :{}" , left , right);
                    left += 1;
                    step = 2;

                    
                }else if step == 2 {
                    println!("- left :{} , right :{}" , left , right);
                    right -= 1;
                    step = 3;
                    
                }
            
            }else if way1 {
                left += 1;
                reset_flag += 1;
                    
            }else if way2 {
                right -= 1;
                reset_flag += 1;
                    
            }else {
                return false;
            }
            
        }
        left += 1;
        right -= 1;
    }
    return true;
 }

