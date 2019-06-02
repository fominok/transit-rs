use super::*;
use serde_json::{map::Map as JsMap, Value as JsVal};
use transit_derive::TransitSerialize;

pub fn to_transit_json<T: TransitSerialize>(v: T) -> JsVal {
    v.transit_serialize(JsonSerializer::top())
}

pub struct JsonArraySerializer {
    buf: Vec<JsVal>,
}

pub struct JsonMapSerializer {
    buf_str_keys: Vec<Option<JsVal>>,
    buf_keys: Vec<JsVal>,
    buf_vals: Vec<JsVal>,
    cmap: bool,
}

impl SerializeMap for JsonMapSerializer {
    type Output = JsVal;

    fn serialize_pair<K: TransitSerialize, V: TransitSerialize>(&mut self, k: K, v: V) {
        self.cmap = self.cmap || (K::TF_TYPE == TransitType::Composite);
        self.buf_keys
            .push(k.transit_serialize(JsonSerializer::default()));
        self.buf_vals
            .push(v.transit_serialize(JsonSerializer::default()));
        // FIXME: compute cmap in the beginning and do not compute this vector if not needed
        self.buf_str_keys
            .push(k.transit_serialize_key(JsonSerializer::default()));
    }

    fn end(self) -> Self::Output {
        if self.cmap {
            let mut val: Vec<JsVal> = Vec::with_capacity(2 * self.buf_keys.len());
            for (k, v) in self.buf_keys.into_iter().zip(self.buf_vals.into_iter()) {
                val.push(k);
                val.push(v);
            }
            let mut map = JsMap::with_capacity(1);
            map.insert("~#cmap".to_owned(), JsVal::Array(val));
            JsVal::Object(map)
        } else {
            let mut map = JsMap::with_capacity(self.buf_keys.len());
            for (k, v) in self.buf_str_keys.into_iter().zip(self.buf_vals.into_iter()) {
                if let JsVal::String(key) = k.expect("Dubg shit") {
                    map.insert(key, v);
                } else {
                    unimplemented!();
                }
            }
            JsVal::Object(map)
        }
    }
}

impl SerializeArray for JsonArraySerializer {
    type Output = JsVal;
    fn serialize_item<T: TransitSerialize>(&mut self, v: T) {
        self.buf
            .push(v.transit_serialize(JsonSerializer::default()));
    }

    fn end(self) -> Self::Output {
        JsVal::Array(self.buf)
    }
}

#[derive(Clone)]
struct JsonSerializer {
    top_level: bool,
}

impl Default for JsonSerializer {
    fn default() -> Self {
        JsonSerializer { top_level: false }
    }
}

impl JsonSerializer {
    fn top() -> Self {
        JsonSerializer { top_level: true }
    }

    fn quote_check(self, v: JsVal) -> JsVal {
        if self.top_level {
            let mut m = JsMap::with_capacity(1);
            m.insert("~#".to_owned(), v);
            JsVal::Object(m)
        } else {
            v
        }
    }
}

struct JsonTagArraySerializer {
    tag: String,
    array_serializer: JsonArraySerializer,
}

struct JsonTagMapSerializer {
    tag: String,
    map_serializer: JsonMapSerializer,
}

impl SerializeTagArray for JsonTagArraySerializer {
    type Output = JsVal;

    fn serialize_item<T: TransitSerialize>(&mut self, v: T) {
        self.array_serializer.serialize_item(v);
    }

    fn end(self) -> Self::Output {
        let val = self.array_serializer.end();
        let mut m = JsMap::with_capacity(1);
        m.insert(self.tag, val);
        JsVal::Object(m)
    }
}

impl SerializeTagMap for JsonTagMapSerializer {
    type Output = JsVal;

    fn serialize_pair<K: TransitSerialize, V: TransitSerialize>(&mut self, k: K, v: V) {
        self.map_serializer.serialize_pair(k, v);
    }

    fn end(self) -> Self::Output {
        let val = self.map_serializer.end();
        let mut m = JsMap::with_capacity(1);
        m.insert(self.tag, val);
        JsVal::Object(m)
    }
}

impl TransitSerializer for JsonSerializer {
    type Output = JsVal;
    type SerializeArray = JsonArraySerializer;
    type SerializeMap = JsonMapSerializer;
    type SerializeTagArray = JsonTagArraySerializer;
    type SerializeTagMap = JsonTagMapSerializer;

    fn serialize_null(self) -> Self::Output {
        self.quote_check(JsVal::Null)
    }

    fn serialize_string(self, v: &str) -> Self::Output {
        self.quote_check(v.into())
    }

    fn serialize_bool(self, v: bool) -> Self::Output {
        self.quote_check(v.into())
    }

    fn serialize_int(self, v: i64) -> Self::Output {
        self.quote_check(v.into())
    }

    fn serialize_float(self, v: f64) -> Self::Output {
        self.quote_check(v.into())
    }

    fn serialize_array(self, len: Option<usize>) -> Self::SerializeArray {
        if let Some(len) = len {
            JsonArraySerializer {
                buf: Vec::with_capacity(len),
            }
        } else {
            JsonArraySerializer { buf: Vec::new() }
        }
    }

    fn serialize_map(self, len: Option<usize>) -> Self::SerializeMap {
        if let Some(len) = len {
            JsonMapSerializer {
                buf_str_keys: Vec::with_capacity(len),
                buf_keys: Vec::with_capacity(len),
                buf_vals: Vec::with_capacity(len),
                cmap: false,
            }
        } else {
            JsonMapSerializer {
                buf_str_keys: Vec::new(),
                buf_keys: Vec::new(),
                buf_vals: Vec::new(),
                cmap: false,
            }
        }
    }

    fn serialize_tagged_array(self, tag: &str, len: Option<usize>) -> Self::SerializeTagArray {
        JsonTagArraySerializer {
            tag: tag.to_owned(),
            array_serializer: self.serialize_array(len),
        }
    }

    fn serialize_tagged_map(self, tag: &str, len: Option<usize>) -> Self::SerializeTagMap {
        JsonTagMapSerializer {
            tag: tag.to_owned(),
            map_serializer: self.serialize_map(len),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn scalar_map_btree() {
        let mut m = BTreeMap::new();
        m.insert(4, "yolo");
        m.insert(-6, "swag");

        let tr = to_transit_json(m);
        assert_eq!(
            json!({
                "~i4": "yolo",
                "~i-6": "swag"
            }),
            tr
        );
    }

    #[test]
    fn map_composite_keys() {
        let mut key1: BTreeMap<bool, &str> = BTreeMap::new();
        key1.insert(true, "test");
        key1.insert(false, "tset");

        let mut m = BTreeMap::new();
        m.insert(key1, 1337);

        let tr = to_transit_json(m);
        assert_eq!(
            json!({
                "~#cmap": [
                    {
                        "~?t": "test",
                        "~?f": "tset",
                    },
                    1337
                ],
            }),
            tr
        );
    }

    #[test]
    fn quoting() {
        let tr = to_transit_json(5i32);
        assert_eq!(
            json!({
                "~#": 5
            }),
            tr
        );
    }

    #[test]
    fn tagged_set() {
        let mut hs = BTreeSet::new();
        hs.insert(0);
        hs.insert(2);
        hs.insert(4);

        let tr = to_transit_json(hs);
        assert_eq!(json!({"~#set": [0, 2, 4]}), tr);
    }

    #[test]
    fn custom_derive_stuct_map() {
        use chrono::{DateTime, TimeZone, Utc};
        #[derive(Clone, TransitSerialize)]
        struct User {
            name: String,
            related: BTreeSet<String>,
            registered: DateTime<Utc>,
            skills_by_rates: BTreeMap<i32, BTreeSet<String>>,
        }

        let mut rel = BTreeSet::new();
        rel.insert("Billy".to_owned());
        rel.insert("Mark".to_owned());
        rel.insert("Steve".to_owned());

        let mut skills = BTreeMap::new();
        let mut hs1 = BTreeSet::new();
        hs1.insert("Linux".to_owned());
        hs1.insert("Git".to_owned());
        skills.insert(3, hs1);
        let mut hs2 = BTreeSet::new();
        hs2.insert("Performance artist".to_owned());
        skills.insert(2, hs2);
        let mut hs3 = BTreeSet::new();
        hs3.insert("Rust".to_owned());
        skills.insert(1, hs3);

        let u = User {
            name: "Van".to_owned(),
            related: rel,
            registered: Utc.ymd(1995, 10, 11).and_hms(0, 0, 0),
            skills_by_rates: skills,
        };
        let tr = to_transit_json(u);
        assert_eq!(
            json!(
                {
                    "~#user": {
                        "name": "Van",
                        "related": {"~#set": ["Billy", "Mark", "Steve"]},
                        "registered": "~t1995-10-11T00:00:00Z",
                        "skills_by_rates": {
                            "~i3": {"~#set": ["Git", "Linux"]},
                            "~i2": {"~#set": ["Performance artist"]},
                            "~i1": {"~#set": ["Rust"]},
                        }
                    }
                }
            ),
            tr
        );
    }

    #[test]
    fn custom_derive_stuct_tuple() {
        #[derive(Clone, TransitSerialize)]
        struct Point(i32, i32);
        let p = Point(13, 37);

        let tr = to_transit_json(p);
        assert_eq!(json!({"~#point": [13, 37]}), tr);
    }

    #[test]
    fn array() {
        let mut m1 = BTreeMap::new();
        let mut m2 = BTreeMap::new();
        m1.insert("test", true);
        m1.insert("hih", true);
        m2.insert("ok", true);
        m2.insert("not ok", false);
        let v = vec![Box::new(m1), Box::new(m2)];

        let tr = to_transit_json(v);
        assert_eq!(
            json!([
                 {
                     "test": true,
                     "hih": true
                 },
                 {
                     "ok": true,
                     "not ok": false
                 }
            ]),
            tr
        );
    }
}
