# 7. Reverse Integer
class Solution:
    def reverse(self, x: int) -> int:
        reversed_x = 0
        sign = (x > 0) - (x < 0)
        x = abs(x)

        while x:
            reversed_x = reversed_x * 10 + x % 10
            x //= 10
        reversed_x *= sign
        return reversed_x if -2**31 <= reversed_x <= 2**31 - 1 else 0
