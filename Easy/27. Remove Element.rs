// 27. Remove Element
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut j = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[j] = nums[i];
                j += 1;
            }
        }

        j as i32
    }
}

