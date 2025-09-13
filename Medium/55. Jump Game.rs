impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut i = 0;

        while i < n {
            if i + nums[i] as usize >= n - 1 {
                return true;
            }
            if nums[i] == 0 {
                return false;
            }
            i += nums[i] as usize;
        }

        false
    }
}

