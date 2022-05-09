// 基数排序
// 用木桶思想 ，把数字的进制位作为索引放入木桶中


pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut max = arr[0];
    while i < arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
        
        i += 1;
    }
    println!("max :{}" ,max);
    // 10个bucket 0-9
    let bucket_count = 10;
    let mut total_bucket : Vec<Vec<i32>> = (0..bucket_count).map(
        |_| Vec::new()
    ).collect();

    // 进制 从个位开始
    let mut system = 0;
    loop {
        if max/10i32.pow(system as u32) == 0 {
            break;
        }
        // 清空
        total_bucket.iter_mut().for_each(|x | { 
            x.clear();
        });
         
        let mut i = 0;
        while i < arr.len() {
            // 当前位
            let mut section = (arr[i]/10i32.pow(system as u32)%10) as usize;
            section = 9-section;
            (&mut total_bucket[section] as &mut Vec<i32>).push(arr[i]);
            i += 1;
        }
        println!("bucket :{:#?}",total_bucket);
        // 把数据装回去
        arr.clear();
        for bucket in total_bucket.iter() {
            for v in bucket.iter() {
                arr.push(*v);
            }
        }
        
        system += 1;
    }
    return arr;
}