// 9. Palindrome Number
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0){
            return false;
        }
        let mut reversed = 0;
        let mut temp  = x;

        while (temp != 0){
            let digit = (temp as i32) % 10;
            reversed = (reversed * 10) + digit;
            temp /= 10;
        }

        return (reversed == x);
    }
}
