// 58. Length of Last Word
impl Solution {
    pub fn length_of_last_word(mut s: String) -> i32 {
        s = s.replace("  ", " ");
        let words: Vec<&str> = s.trim().split(" ").collect();
        let last_word: &str = words[words.len() - 1];

        last_word.len() as i32
    }
}
