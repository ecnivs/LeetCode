// 4. Median of Two Sorted Arrays
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let (m, n) = (nums1.len(), nums2.len());
        let total = m + n;
        let mut left = 0;
        let mut right = m;

        while left <= right {
            let px = (left + right) / 2;
            let py = (total + 1) / 2 - px;

            let left1 = if px == 0 { i32::MIN } else { nums1[px - 1] };
            let right1 = if px == m { i32::MAX } else { nums1[px] };
            let left2 = if py == 0 { i32::MIN } else { nums2[py - 1] };
            let right2 = if py == n { i32::MAX } else { nums2[py] };

            if left1 <= right2 && left2 <= right1 {
                return if total % 2 == 0 {
                    (left1.max(left2) as f64 + right1.min(right2) as f64) / 2.0
                } else {
                    left1.max(left2) as f64
                }
            }
            if left1 > right2 {
                right = px - 1;
            } else {
                left = px + 1;
            }
        }
        0.0
    }
}
