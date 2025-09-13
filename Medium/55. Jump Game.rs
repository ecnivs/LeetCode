impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut visited = vec![false; n];
        let mut stack = vec![0];

        while let Some(i) = stack.pop() {
            if i >= n - 1 {
                return true;
            }
            if visited[i] {
                continue;
            }
            visited[i] = true;

            let max_jump = nums[i] as usize;
            for step in 1..=max_jump {
                let next = i + step;
                if next < n {
                    stack.push(next);
                } else {
                    return true;
                }
            }
        }

        false
    }
}
