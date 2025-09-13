use std::collections::HashMap;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach = 0;
        let dest = nums.len() - 1;

        for (i, item) in nums.into_iter().enumerate() {
            if i > max_reach {
                return false;
            }

            max_reach = max_reach.max(i + item as usize);

            if max_reach >= dest {
                return true;
            }
        }

        return false;
    }
}
