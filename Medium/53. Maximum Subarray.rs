impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_current = nums[0];
        let mut max_global = nums[0];

        for &num in nums.iter().skip(1) {
            max_current = std::cmp::max(num, max_current + num);
            max_global = std::cmp::max(max_global, max_current)
        }

        max_global
    }
}
