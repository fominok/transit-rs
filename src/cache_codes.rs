use std::collections::HashMap;
use std::convert::TryFrom;

const CACHE_CODE_DIGITS: u8 = 44;
const BASE_CHAR_INDEX: u8 = 48;
const DEFAULT_CAPACITY: usize = 50;

pub(crate) struct NumbersIter {
    index: u16,
}

impl Iterator for NumbersIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let hi: u8 = u8::try_from(self.index / (CACHE_CODE_DIGITS as u16)).ok()?;
        let lo: u8 = u8::try_from(self.index % (CACHE_CODE_DIGITS as u16)).ok()?;

        self.index += 1;

        Some(if hi == 0 {
            format!("^{}", (lo + BASE_CHAR_INDEX) as char)
        } else {
            format!(
                "^{}{}",
                (hi + BASE_CHAR_INDEX) as char,
                (lo + BASE_CHAR_INDEX) as char
            )
        })
    }
}

pub(crate) struct KeyCacher {
    numbers_iter: NumbersIter,
    map: HashMap<String, String>,
}

// FIXME: reduce copying
impl KeyCacher {
    pub(crate) fn cache(&mut self, s: &str) -> String {
        if let Some(code) = self.map.get(s) {
            code.to_owned()
        } else {
            let code = self.numbers_iter.next().expect("Too many keys to cache");
            self.map.insert(s.to_owned(), code.clone());
            s.to_owned() // too bad
        }
    }

    pub(crate) fn new() -> Self {
        KeyCacher {
            numbers_iter: NumbersIter { index: 0 },
            map: HashMap::with_capacity(DEFAULT_CAPACITY),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_digit() {
        let mut cacher = KeyCacher::new();
        assert_eq!(cacher.cache("test1"), "test1");
        assert_eq!(cacher.cache("test2"), "test2");
        assert_eq!(cacher.cache("test3"), "test3");
        assert_eq!(cacher.cache("test4"), "test4");
        assert_eq!(cacher.cache("test5"), "test5");
        assert_eq!(cacher.cache("test6"), "test6");
        assert_eq!(cacher.cache("test7"), "test7");
        assert_eq!(cacher.cache("test8"), "test8");
        assert_eq!(cacher.cache("test9"), "test9");
        assert_eq!(cacher.cache("test10"), "test10");
        assert_eq!(cacher.cache("test11"), "test11");

        assert_eq!(cacher.cache("test5"), "^4");
        assert_eq!(cacher.cache("test6"), "^5");

        assert_eq!(cacher.cache("test12"), "test12");
    }

    #[test]
    fn two_digits() {
        let mut cacher = KeyCacher::new();
        for i in 0..CACHE_CODE_DIGITS {
            cacher.cache(&format!("test{}", i));
        }

        assert_eq!(cacher.cache("another one"), "another one");
        assert_eq!(cacher.cache("another one"), "^10");
        assert_eq!(cacher.cache("another two"), "another two");

        assert_eq!(cacher.cache("another one"), "^10");
        assert_eq!(cacher.cache("another one"), "^10");
        assert_eq!(cacher.cache("another two"), "^11");
    }
}
