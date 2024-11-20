// 4. Median of Two Sorted Arrays
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged = nums1;
        merged.extend(nums2);
        merged.sort();

        let n = merged.len();
        if n % 2 == 0 {
            (merged[n / 2 - 1] as f64 + merged[n / 2] as f64) / 2.0
        } else {
            merged[n / 2] as f64
        }
    }
}
