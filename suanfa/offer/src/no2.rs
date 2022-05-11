

//给定两个 01 字符串 a 和 b ，请计算它们的和，并以二进制字符串的形式输出。
// 输入为 非空 字符串且只包含数字 1 和 0。
//
pub fn run() {
    let res = js("1".to_string(),"1101".to_string());
    assert_eq!(res , "1110");
}

fn js(a : String , b : String) -> String{
    const RADIX: u32 = 10;
    let mut a_chars = a.chars().rev();
    let mut b_chars = b.chars().rev();
    let max_bits = if a.len() > b.len() {a.len()}else{b.len()};
    let mut res = String::new();
    let mut i = 0;
    let mut carry_bit = 0;
    while i < 64 && i < max_bits {
        
        let i_a = a_chars.next();
        let i_a = if i_a.is_some() {
            i_a.unwrap().to_digit(RADIX).unwrap()
        }else{0};
        let i_b = b_chars.next();
        let i_b = if i_b.is_some() {
            i_b.unwrap().to_digit(RADIX).unwrap()
        }else{0};
        let mut bit = i_b + i_a + carry_bit;
        carry_bit = 0;

        if bit > 1 {
            carry_bit = 1;
            bit = bit - 2;
        }
        res.insert_str(0, bit.to_string().as_str());
        println!("i_a :{} , i_b :{} ,bit : {}, res :{}" ,i_a ,i_b ,bit ,res);
        i += 1;
    }
    return res;
}