use std::collections::{hash_map, HashMap};
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

impl KeyCacher {
    pub(crate) fn cache(&mut self, s: String) -> Option<&str> {
        match self.map.entry(s) {
            hash_map::Entry::Occupied(e) => Some(e.into_mut()),
            hash_map::Entry::Vacant(e) => {
                let code = self.numbers_iter.next().expect("Too many keys to cache");
                e.insert(code);
                None
            }
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

        assert_eq!(cacher.cache("test1".to_owned()), None);
        assert_eq!(cacher.cache("test2".to_owned()), None);
        assert_eq!(cacher.cache("test3".to_owned()), None);
        assert_eq!(cacher.cache("test4".to_owned()), None);
        assert_eq!(cacher.cache("test5".to_owned()), None);
        assert_eq!(cacher.cache("test6".to_owned()), None);
        assert_eq!(cacher.cache("test7".to_owned()), None);
        assert_eq!(cacher.cache("test8".to_owned()), None);
        assert_eq!(cacher.cache("test9".to_owned()), None);
        assert_eq!(cacher.cache("test10".to_owned()), None);
        assert_eq!(cacher.cache("test11".to_owned()), None);

        assert_eq!(cacher.cache("test5".to_owned()), Some("^4"));
        assert_eq!(cacher.cache("test6".to_owned()), Some("^5"));

        assert_eq!(cacher.cache("test12".to_owned()), None);
    }

    #[test]
    fn two_digits() {
        let mut cacher = KeyCacher::new();

        for i in 0..CACHE_CODE_DIGITS {
            cacher.cache(format!("test{}", i));
        }

        assert_eq!(cacher.cache("another one".to_owned()), None);
        assert_eq!(cacher.cache("another one".to_owned()), Some("^10"));
        assert_eq!(cacher.cache("another two".to_owned()), None);

        assert_eq!(cacher.cache("another one".to_owned()), Some("^10"));
        assert_eq!(cacher.cache("another one".to_owned()), Some("^10"));
        assert_eq!(cacher.cache("another two".to_owned()), Some("^11"));
    }
}
