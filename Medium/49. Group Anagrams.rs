use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        for word in strs {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort_unstable();
            let key: String = chars.into_iter().collect();

            groups.entry(key).or_default().push(word);
        }

        groups.into_values().collect()
    }
}
