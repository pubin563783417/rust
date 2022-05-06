// 归并排序
// 分而治之
// 稳定排序算法，一般用于对总体无序，但是各子项相对有序的数列。
pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() == 0 || arr.len() == 1 {
        return arr;
    }
    // 1. 分割
    let a = arr.len()/2;
    let b = arr.len()-a;
    
    let b =  arr[a..arr.len()].to_vec();
    let a =  arr[0..a].to_vec();
    // 递归 返回有序子数组
    let a = sort(a);
    let b = sort(b);
    // 2.归并
    let mut vec : Vec<i32> = Vec::with_capacity(a.len()+b.len());

    if a.len() == 0 {
        return b;
    }else if b.len() == 0 {
        return a;
    }

    let mut i = 0;
    let mut j = 0;
    loop {
        if i == a.len() && j == b.len() {
            break;
        }else if i == a.len() {
            // a已经用完了
            vec.push(b[j]);
            j += 1;
        }else if j == b.len() {
            // b已经用完了 ,去加入a的元素
            vec.push(a[i]);
            i += 1;
        }else {
            if a[i] > b[j] {
                vec.push(a[i]);
                i += 1;
            }else {
                vec.push(b[j]);
                j += 1;
            }
        } 
    }
    return vec;
}