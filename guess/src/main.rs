
use rand::Rng;
use std::io;

fn main() {
    let rng = rand::thread_rng().gen_range(1..101);
    println!("猜字游戏！");

    println!("点击 回车 开始");

    let mut start = String::new();
    io::stdin()
    .read_line(&mut start).expect("开始错误");


    println!("开始了!");

    loop {
        println!("输入你的数字 ：");
        let mut num = String::new();

        let input = io::stdin()
        .read_line(&mut num);
        if input.is_ok() {
            let num = num.trim().parse::<i32>();
            if num.is_ok() {
                let num = num.unwrap();
                if num > rng {
                    println!("你的数字太大了 : {}" , &num);
                }else if num < rng {
                    println!("你的数字太小了 : {}" , &num);
                }else {
                    break;
                }
                continue;
            }
        }
        println!("你的输入有误!");
    }
    
    println!("恭喜成功猜对 ：谜底 {}" , rng);
}
