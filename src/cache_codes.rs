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

pub(crate) struct KeyCacher<'a> {
    numbers_iter: NumbersIter,
    map: HashMap<&'a str, String>,
}

impl<'a> KeyCacher<'a> {
    pub(crate) fn cache<'b>(&'b mut self, s: &'a str) -> &'b str {
        match self.map.entry(s) {
            hash_map::Entry::Occupied(e) => e.into_mut(),
            hash_map::Entry::Vacant(e) => {
                let code = self.numbers_iter.next().expect("Too many keys to cache");
                e.insert(code);
                s
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
        // Here is how it will be used:
        // There is data that will outlive cacher;
        // Returned references from cacher will be dropped before next call;
        let data = vec![
            "test1".to_owned(),
            "test2".to_owned(),
            "test3".to_owned(),
            "test4".to_owned(),
            "test5".to_owned(),
            "test6".to_owned(),
            "test7".to_owned(),
            "test8".to_owned(),
            "test9".to_owned(),
            "test10".to_owned(),
            "test11".to_owned(),
            "test12".to_owned(),
        ];

        assert_eq!(cacher.cache(&data[0]), "test1");
        assert_eq!(cacher.cache(&data[1]), "test2");
        assert_eq!(cacher.cache(&data[2]), "test3");
        assert_eq!(cacher.cache(&data[3]), "test4");
        assert_eq!(cacher.cache(&data[4]), "test5");
        assert_eq!(cacher.cache(&data[5]), "test6");
        assert_eq!(cacher.cache(&data[6]), "test7");
        assert_eq!(cacher.cache(&data[7]), "test8");
        assert_eq!(cacher.cache(&data[8]), "test9");
        assert_eq!(cacher.cache(&data[9]), "test10");
        assert_eq!(cacher.cache(&data[10]), "test11");

        assert_eq!(cacher.cache(&data[4]), "^4");
        assert_eq!(cacher.cache(&data[5]), "^5");

        assert_eq!(cacher.cache(&data[11]), "test12");
    }

    #[test]
    fn two_digits() {
        let mut cacher = KeyCacher::new();
        let mut data = Vec::with_capacity(CACHE_CODE_DIGITS as usize);
        for i in 0..CACHE_CODE_DIGITS {
            data.push(format!("test{}", i));
        }
        // mutable borrows for `data` end here
        for i in 0..CACHE_CODE_DIGITS {
            cacher.cache(&data[i as usize]);
        }

        // 'static slices will outlive anyways
        assert_eq!(cacher.cache("another one"), "another one");
        assert_eq!(cacher.cache("another one"), "^10");
        assert_eq!(cacher.cache("another two"), "another two");

        assert_eq!(cacher.cache("another one"), "^10");
        assert_eq!(cacher.cache("another one"), "^10");
        assert_eq!(cacher.cache("another two"), "^11");
    }
}
