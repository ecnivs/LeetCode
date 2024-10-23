# 3. Longest Substring Without Repeating Characters
class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        index = [-1] * 128
        max_length = 0
        left = 0

        for right in range(len(s)):
            char = s[right]
            left = max(index[ord(char)] + 1, left)
            index[ord(char)] = right
            max_length = max(max_length, right - left + 1)
        return max_length
