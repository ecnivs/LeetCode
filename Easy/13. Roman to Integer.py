# 13. Roman to Integer
class Solution(object):
     def romanToInt(self, s):
        sum = 0
        prevValue = 0
        values = [0] * 256
        values[ord('I')] = 1; values[ord('V')] = 5; values[ord('X')] = 10; values[ord('L')] = 50
        values[ord('C')] = 100; values[ord('D')] = 500; values[ord('M')] = 1000
        
        for c in s:
            currentValue = values[ord(c)]
            sum += (currentValue - 2 * prevValue) if (currentValue > prevValue) else currentValue
            prevValue = currentValue
        return sum

