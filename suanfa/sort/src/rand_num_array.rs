use rand::Rng;

pub fn gen_rand_array(len: i32, max: i32) -> Vec<i32> {
    let array: Vec<i32> = (0..len).collect();

    let array: Vec<i32> = array
        .iter()
        .map(|_| rand::thread_rng().gen_range(1..max))
        .collect();

    return array;
}
