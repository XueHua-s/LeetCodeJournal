use std::collections::HashMap;


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map = HashMap::new();
    for i in 0..nums.len()  {
        let complement = target - nums[i];
        if let Some(&index) = hash_map.get(&complement) {
            return vec![i as i32, index as i32];
        }
        hash_map.insert(nums[i], i);
    }
    Vec::new()
}
fn main() {
    println!("Hello, world!");
}
