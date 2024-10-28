# 22. Generate Parenthesis
class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        result = []
        item = ""
        self.dsf(item, result, n, 0, 0)
        return result

    def dsf(self, item, result, n, left, right):
        if left == right and left == n:
            result.append(item)
            return

        if left < n:
            self.dsf(item+"(", result, n, left + 1, right)

        if right < left:
            self.dsf(item+")", result, n, left, right+1)
