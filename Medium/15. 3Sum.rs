// 15. 3Sum
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();
        let mut res = Vec::new();
        let length = nums.len();

        for i in 0..length - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if nums[i] > 0 {
                break;
            }

            let (mut l, mut r) = (i + 1, length - 1);

            while l < r {
                let total = nums[i] + nums[l] + nums[r];
                
                if total < 0 {
                    l += 1;
                } else if total > 0 {
                    r -= 1;
                } else {
                    res.push(vec![nums[i], nums[l], nums[r]]);

                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }

                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                }
            }
        }
        res
    }
}
