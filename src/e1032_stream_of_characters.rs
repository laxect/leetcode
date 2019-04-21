pub struct StreamChecker {
    words: Vec<String>,
    find: String,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {

    pub fn new(words: Vec<String>) -> Self {
        Self {
            words,
            find: String::new(),
        }
    }

    pub fn query(&mut self, letter: char) -> bool {
        self.find.push(letter);
        self.words.iter().any(|s| self.find.ends_with(s.as_str()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_0() {
        let mut streamChecker = StreamChecker::new(vec!["cd".into(), "f".into(), "kl".into()]); // init the dictionary.
        streamChecker.query('a'); // return false
        streamChecker.query('b'); // return false
        streamChecker.query('c'); // return false
        streamChecker.query('d'); // return true, because 'cd' is in the wordlist
        streamChecker.query('e'); // return false
        streamChecker.query('f'); // return true, because 'f' is in the wordlist
        streamChecker.query('g'); // return false
        streamChecker.query('h'); // return false
        streamChecker.query('i'); // return false
        streamChecker.query('j'); // return false
        streamChecker.query('k'); // return false
        streamChecker.query('l'); // return true, because 'kl' is in the wordlist
    }
}
