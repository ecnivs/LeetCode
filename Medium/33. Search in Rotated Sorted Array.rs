impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];

            if mid_val == target {
                return mid as i32;
            }

            if nums[left as usize] <= mid_val {
                if nums[left as usize] <= target && target < mid_val {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if mid_val < target && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        -1
    }
}
