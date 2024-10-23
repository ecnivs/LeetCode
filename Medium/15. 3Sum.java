// 15. 3Sum
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Solution {
    public List<List<Integer>> threeSum(int[] nums) {
        List<List<Integer>> res = new ArrayList<>();
        Arrays.sort(nums);

        for (int i = 0; i < nums.length - 2; i++) {
            if (i > 0 && nums[i] == nums[i - 1]) {
                continue;
            }

            if (nums[i] > 0) {
                break;
            }

            int l = i + 1;
            int r = nums.length - 1;

            while (l < r) {
                int total = nums[i] + nums[l] + nums[r];
                if (total < 0) {
                    l++;
                } else if (total > 0) {
                    r--;
                } else {
                    res.add(Arrays.asList(nums[i], nums[l], nums[r]));

                    while (l < r && nums[l] == nums[l + 1]) {
                        l++;
                    }
                    while (l < r && nums[r] == nums[r - 1]) {
                        r--;
                    }
                    l++;
                    r--;
                }
            }
        }
        return res;
    }
}
