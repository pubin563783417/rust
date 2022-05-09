use  crate::insert;
// 木桶排序
//  最优O(n)  ,对计数排序的优化  ，当每一个值对应一个bucket 达到最优
// 每个木桶代表一个区间 ，把值放进不同的桶里  ，然后每个木桶排序 ，合并
pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut max = arr[0];
    let mut min = arr[0];
    while i < arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
        if arr[i] < min {
            min = arr[i];
        }
        i += 1;
    }
    // 设计每个木桶放10个数
    // 木桶数量 = count/10
    let bucket_count = arr.len()/10;
    let mut total_bucket : Vec<Vec<i32>> = (0..bucket_count).map(
        |_| Vec::new()
    ).collect();

    let diff = (max-min)/(bucket_count as i32);
    
    let mut i = 0;
    while i < arr.len() {
        let v = arr[i];
        let v = v - min;
        let mut section  = (v/diff) as usize;
        let section_max = bucket_count - 1;
        if section > section_max  {
            section = section_max;
        }
        section = section_max-section;
        (&mut total_bucket[section] as &mut Vec<i32>).push(v);
        i += 1;
    }
    let mut arr = Vec::<i32>::new();
    let mut i = 0;
    while i < total_bucket.len() {
        let list = &total_bucket[i];
        let mut list = insert::sort(list.to_vec());
        println!("section : {:?}" , list);
        arr.append(&mut list);

        i += 1;
    }

    
    return arr;
}

// impl Copy for Vec<i32> {
    
// }