use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut count = HashMap::new();
        for w in words {
            *count.entry(w.clone()).or_insert(0) += 1;
        }

        let mut unique_words: Vec<String> = count.keys().cloned().collect();

        unique_words.sort_by(|a, b| {
            let freq_cmp = count[b].cmp(&count[a]);

            if freq_cmp == std::cmp::Ordering::Equal {
                a.cmp(b)
            } else {
                freq_cmp
            }
        });

        unique_words.into_iter().take(k as usize).collect()
    }
}
