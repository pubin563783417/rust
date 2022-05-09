// 堆排序 时间复杂度都是O(nlogn)，空间复杂度O(1)。
// 大顶堆 ， 跟节点为最大值  ，完全二差数  ， 父节点下标为i ，那么会有左子节点i*2+1
// 1.将数组构造成大顶堆 2.切换下标0节点和最后一个节点的值  ，重复1.2
// 1: 找到非叶子节点，从最后一个开始一次遍历，调整他们的位置
// 第一个叶子节点索引 ： count/2,最后一个非叶子节点：count/2-1,
pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() == 1 || arr.len() == 0{
        return arr;
    }
    let mut sorted_count = 0;
    let l = arr.len();
    loop {
        // 1.构造
        let list = &mut arr[..(l-sorted_count)];
        if list.len() == 1 {
            
        }else {
            // 找到最后一个非叶子节点 调整他们的子节点位置
            let mut i = list.len()/2-1;
            loop {
                if list[i] > list[i*2+1]  {
                    let temp = list[i];
                    list[i] = list[i*2+1];
                    list[i*2+1] = temp;
                }
                if i*2+2 < list.len() && list[i] > list[i*2+2] {
                    let temp = list[i];
                    list[i] = list[i*2+2];
                    list[i*2+2] = temp;
                }

                if i == 0 {
                    break;
                }
                i -= 1;
            } 
        }
        // 2.获取大顶值 0和最后一个互换
        let temp = list[0];
        list[0] = list[list.len()-1];
        list[list.len()-1] = temp;

        sorted_count += 1;
        if sorted_count == arr.len() {
            break;
        }
    }
    
    return arr;
}
