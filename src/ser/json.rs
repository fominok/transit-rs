use super::*;
use serde_json::Value as JsVal;

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
        let mut vec = Vec::with_capacity(2);
        vec.push(JsVal::String(self.tag));
        vec.push(val);
        JsVal::Array(vec)
    }
}

impl SerializeTagMap for JsonTagMapSerializer {
    type Output = JsVal;

    fn serialize_pair<K: TransitSerialize, V: TransitSerialize>(&mut self, k: K, v: V) {
        self.map_serializer.serialize_pair(k, v);
    }

    fn end(self) -> Self::Output {
        let val = self.map_serializer.end();
        let mut vec = Vec::with_capacity(2);
        vec.push(JsVal::String(self.tag));
        vec.push(val);
        JsVal::Array(vec)
    }
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
            let mut vec = Vec::with_capacity(2);
            vec.push(JsVal::String("~#cmap".to_owned()));
            vec.push(JsVal::Array(val));
            JsVal::Array(vec)
        } else {
            let mut vec = Vec::with_capacity(self.buf_keys.len() + 1);
            vec.push(JsVal::String("^".to_owned()));
            for (k, v) in self.buf_str_keys.into_iter().zip(self.buf_vals.into_iter()) {
                vec.push(k.expect("wut"));
                vec.push(v);
            }
            JsVal::Array(vec)
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
            let mut vec = Vec::with_capacity(2);
            vec.push(JsVal::String("~#".to_owned()));
            vec.push(v);
            JsVal::Array(vec)
        } else {
            v
        }
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
        assert_eq!(json!(["^", "~i-6", "swag", "~i4", "yolo"]), tr);
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
            json!(["~#cmap", [["^", "~?f", "tset", "~?t", "test"], 1337],]),
            tr
        );
    }

    #[test]
    fn quoting() {
        let tr = to_transit_json(5i32);
        assert_eq!(json!(["~#", 5]), tr);
    }

    #[test]
    fn tagged_set() {
        let mut hs = BTreeSet::new();
        hs.insert(0);
        hs.insert(2);
        hs.insert(4);

        let tr = to_transit_json(hs);
        assert_eq!(json!(["~#set", [0, 2, 4]]), tr);
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
                ["^", "hih", true, "test", true],
                ["^", "not ok", false, "ok", true]
            ]),
            tr
        );
    }

    #[test]
    fn null() {
        let kek = Some("swag");
        let lol = None;
        let v = vec![kek, lol];
        let tr = to_transit_json(v);

        assert_eq!(json!(["swag", null]), tr);
    }

    #[test]
    fn null_key() {
        let mut hm = BTreeMap::new();
        let lol: Option<i32> = None;
        hm.insert(lol, 1337);
        let tr = to_transit_json(hm);

        assert_eq!(json!(["^", "~_", 1337]), tr);
    }
}
