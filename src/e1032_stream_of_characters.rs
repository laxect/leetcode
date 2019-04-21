use std::collections::VecDeque;

pub struct StreamChecker {
    words: Vec<String>,
    find: VecDeque<(usize, usize)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {

    pub fn new(words: Vec<String>) -> Self {
        Self {
            words,
            find: VecDeque::new(),
        }
    }

    pub fn query(&mut self, letter: char) -> bool {
        let mut res = false;
        for _ in 0..self.find.len() {
            if let Some((i, mut ind)) = self.find.pop_front() {
                ind += 1;
                let l = self.words[i].len();
                if l > ind && self.words[i].as_bytes()[ind] as char == letter {
                    if ind + 1 == l {
                        res = true;
                    } else {
                        self.find.push_back((i, ind));
                    }
                }
            }
        }
        // add new find
        for i in 0..self.words.len() {
            if letter == self.words[i].as_bytes()[0] as char {
                if self.words[i].len() > 1 {
                    self.find.push_back((i, 0));
                } else {
                    res = true;
                }
            }
        }
        res
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
