// 20. Valid Parenthesis
class Solution {
    public boolean isValid(String s) {
        int n = s.length();
        char[] stack = new char[n];
        int top = -1;

        for (char c : s.toCharArray()) {
            if (c == '(' || c == '{' || c == '[') {
                stack[++top] = c;
            } else {
                if (top == -1)
                    return false;
                char topChar = stack[top--];
                if ((c == ')' && topChar != '(') || (c == '}' && topChar != '{') || (c == ']' && topChar != '[')) {
                    return false;
                }
            }
        }
        return top == -1;
    }
}
