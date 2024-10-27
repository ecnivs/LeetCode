# 5. Longest Palindromic Substring
class Solution:
    def longestPalindrome(self, s: str) -> str:
        if not s:
            return ""

        transformed = self.transform(s)
        n = len(transformed)
        p = [0] * n
        center = right = 0

        for i in range(1, n - 1):
            mirror = 2 * center - i
            if right > i:
                p[i] = min(right - i, p[mirror])

            a, b = i + (1 + p[i]), i - (1 + p[i])
            while a < n - 1 and b > 0 and transformed[a] == transformed[b]:
                p[i] += 1
                a += 1
                b -= 1

            if i + p[i] > right:
                center = i
                right = i + p[i]

        max_len, center_index = max((n, i) for i, n in enumerate(p))
        start = (center_index - max_len) // 2
        return s[start:start + max_len]

    def transform(self, s: str) -> str:
        return '^#' + '#'.join(s) + '#$'
