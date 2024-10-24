# 20. Valid Parentheses
class Solution:
    def isValid(self, s: str) -> bool:
        matching = {')': '(', '}': '{', ']': '['}
        stack = []

        for char in s:
            if char in matching.values():
                stack.append(char)
            elif char in matching.keys():
                if stack == [] or stack[-1] != matching[char]:
                    return False
                stack.pop()
        return stack == []
