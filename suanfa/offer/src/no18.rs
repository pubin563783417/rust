/*
给定一个字符串 s ，验证 s 是否是 回文串 ，只考虑字母和数字字符，可以忽略字母的大小写。

本题中，将空字符串定义为有效的 回文串 。


*/
pub fn run() {
    assert!(js(" ".to_string()));
}

fn js(s: String) -> bool {
    
    let chars = s.to_lowercase().chars().fold(vec![], 
        |mut x , y| {
            if (y <= 'z' && y >= 'a') || (y <= '9' && y >= '0') {
                x.push(y);
            }
            return x;
        }
    );
    if chars.len() == 0 {
        return true;
    }
    println!("chars : {:?}" , chars);
    let mut left = 0;
    let mut right = chars.len()-1;
    while left <= right {
        if chars[left] != chars[right] {
            return false;
        }
        if right == 0 {
            break;
        }
        left += 1;
        right -= 1;
    }
    return true;
}