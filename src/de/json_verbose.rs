use super::*;
use serde_json::{
    map::{IntoIter as JsMapIntoIter},
    Value as JsVal,
};

pub fn from_transit_json<T: TransitDeserialize>(v: JsVal) -> TResult<T> {
    TransitDeserialize::transit_deserialize(JsonDeserializer, v)
}

struct JsonObjectIntoIter {
    js_iter: JsMapIntoIter,
}

impl Iterator for JsonObjectIntoIter {
    type Item = (JsVal, JsVal);

    fn next(&mut self) -> Option<Self::Item> {
        self.js_iter.next().map(|(k, v)| (JsVal::String(k), v))
    }
}

#[derive(Clone, Debug)]
struct JsonDeserializer;

impl TransitDeserializer for JsonDeserializer {
    type Input = JsVal;
    type DeserializeArray = std::vec::IntoIter<JsVal>;
    type DeserializeMap = JsonObjectIntoIter;

    fn deserialize_string(self, v: Self::Input) -> TResult<String> {
        v.as_str()
            .map(|x| x.to_owned())
            .ok_or(Error::DoNotMatch(format!("{} is not string", v)))
    }

    fn deserialize_bool(self, v: Self::Input) -> TResult<bool> {
        v.as_bool()
            .ok_or(Error::DoNotMatch(format!("{} is not bool", v)))
    }

    fn deserialize_int(self, v: Self::Input) -> TResult<i64> {
        v.as_i64()
            .ok_or(Error::DoNotMatch(format!("{} is not int", v)))
    }

    fn deserialize_float(self, v: Self::Input) -> TResult<f64> {
        v.as_f64()
            .ok_or(Error::DoNotMatch(format!("{} is not float", v)))
    }

    fn deserialize_array(self, v: Self::Input) -> TResult<(Self::DeserializeArray, Option<usize>)> {
        if let JsVal::Array(vec) = v {
            let l = vec.len();
            Ok((vec.into_iter(), Some(l)))
        } else {
            Err(Error::DoNotMatch(format!("{} is not an array", v)))
        }
    }

    fn deserialize_map(self, v: Self::Input) -> TResult<(Self::DeserializeMap, Option<usize>)> {
        if let JsVal::Object(m) = v {
            let l = m.len();
            Ok((
                JsonObjectIntoIter {
                    js_iter: m.into_iter(),
                },
                Some(l),
            ))
        } else {
            Err(Error::DoNotMatch(format!("{} is not a map", v)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn dumb_array() {
        let v = vec![1, 2, 3];
        let tr: Vec<i32> = from_transit_json(json!([1, 2, 3])).unwrap();
        assert_eq!(v, tr);
    }

    // TODO: Quoting
    // Check that something like 5 cannot be parsed on top level

    #[test]
    fn scalar_map_btree() {
        let mut m = BTreeMap::new();
        m.insert(4, "yolo".to_owned());
        m.insert(-6, "swag".to_owned());

        let tr = from_transit_json(json!({
            "~i4": "yolo",
            "~i-6": "swag"
        }))
        .unwrap();
        assert_eq!(m, tr);
    }

    #[test]
    fn map_composite_keys() {
        let mut key1: BTreeMap<bool, String> = BTreeMap::new();
        key1.insert(true, "test".to_owned());
        key1.insert(false, "tset".to_owned());

        let mut m = BTreeMap::new();
        m.insert(key1, 1337);

        let tr: BTreeMap<BTreeMap<bool, String>, i32> = from_transit_json(json!({
            "~#cmap": [
                {
                    "~?t": "test",
                    "~?f": "tset",
                },
                1337
            ],
        }))
        .unwrap();
        assert_eq!(tr, m);
    }

    #[test]
    fn map_composite_keys_hashmap() {
        let mut key1: BTreeMap<bool, String> = BTreeMap::new();
        key1.insert(true, "test".to_owned());
        key1.insert(false, "tset".to_owned());

        let mut m = HashMap::new();
        m.insert(key1, 1337);

        let tr: HashMap<BTreeMap<bool, String>, i32> = from_transit_json(json!({
            "~#cmap": [
                {
                    "~?t": "test",
                    "~?f": "tset",
                },
                1337
            ],
        }))
        .unwrap();
        assert_eq!(tr, m);
    }
}
