// 13. Roman to Integer
import java.util.HashMap;

class Solution {
    public int romanToInt(String s) {
        HashMap<Character, Integer> translations = new HashMap<>();
        translations.put('I', 1);
        translations.put('V', 5);
        translations.put('X', 10);
        translations.put('L', 50);
        translations.put('C', 100);
        translations.put('D', 500);
        translations.put('M', 1000);
        
        int number = 0;
        s = s.replace("IV", "IIII").replace("IX", "VIIII");
        s = s.replace("XL", "XXXX").replace("XC", "LXXXX");
        s = s.replace("CD", "CCCC").replace("CM", "DCCCC");
        
        for (char c : s.toCharArray()) {
            number += translations.get(c);
        }
        
        return number;
    }
}
