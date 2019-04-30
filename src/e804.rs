/// unique-morse-code-words
pub struct Solution;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let mores_table = vec![".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
        let mut output = std::collections::HashSet::new();
        let char_a = 97;
        for word in words {
            let mut mores_word = String::new();
            for ch in word.as_bytes() {
                mores_word.push_str(mores_table[(*ch - char_a) as usize]);
            }
            output.insert(mores_word);
        }
        output.len() as i32
    }
}