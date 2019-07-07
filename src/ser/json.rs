use super::*;
use itertools::Itertools;
use serde_json::{map::Map as JsMap, Value as JsVal};

pub fn to_transit_json<T: TransitSerialize>(v: T) -> JsVal {
    v.transit_serialize(&JsonSerializer::top())
}

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
    
    fn quote_check(&self, v: JsVal) -> JsVal {
        if self.top_level {
            let mut m = JsMap::with_capacity(1);
            m.insert("~#".to_owned(), v);
            JsVal::Object(m)
        } else {
            v
        }
    }
}

impl TransitSerializer for JsonSerializer {
    type Output = JsVal;
    type ArraySerializer = JsonArraySerializer;
    type MapSerializer = JsonMapSerializer;
    type TaggedArraySerializer = JsonTaggedArraySerializer;
    type TaggedMapSerializer = JsonTaggedMapSerializer;

    fn serialize_null(&self) -> Self::Output {
        self.quote_check(JsVal::Null)
    }

    fn serialize_string(&self, v: &str) -> Self::Output {
        self.quote_check(v.into())
    }

    fn serialize_bool(&self, v: bool) -> Self::Output {
        self.quote_check(v.into())
    }

    fn serialize_int(&self, v: i64) -> Self::Output {
        self.quote_check(v.into())
    }

    fn serialize_float(&self, v: f64) -> Self::Output {
        self.quote_check(v.into())
    }

    fn serialize_array(&self, len: Option<usize>) -> Self::ArraySerializer {
        let inner = Self::default();
        if let Some(len) = len {
            JsonArraySerializer {
                buf: Vec::with_capacity(len),
                inner_serializer: inner,
            }
        } else {
            JsonArraySerializer {
                buf: Vec::new(),

                inner_serializer: inner,
            }
        }
    }

    fn serialize_map(&self, len: Option<usize>) -> Self::MapSerializer {
        let inner = Self::default();
        if let Some(len) = len {
            JsonMapSerializer {
                buf_keys: Vec::with_capacity(len),
                buf_vals: Vec::with_capacity(len),
                cmap: false,
                inner_serializer: inner,
            }
        } else {
            JsonMapSerializer {
                buf_keys: Vec::new(),
                buf_vals: Vec::new(),
                cmap: false,
                inner_serializer: inner,
            }
        }
    }

    fn serialize_tagged_array(&self, tag: &str, len: Option<usize>) -> Self::TaggedArraySerializer {
        JsonTaggedArraySerializer {
            tag: tag.to_owned(),
            array_serializer: self.serialize_array(len),
        }
    }

    fn serialize_tagged_map(&self, tag: &str, len: Option<usize>) -> Self::TaggedMapSerializer {
        JsonTaggedMapSerializer {
            tag: tag.to_owned(),
            map_serializer: self.serialize_map(len),
        }
    }

    fn serialize_array_iter<'t, T, I>(&self, v: I) -> Self::Output
    where
        T: TransitSerialize + 't,
        I: Iterator<Item = &'t T>,
    {
        let serializer = Self::default();
        let v_ser = v.map(|x| x.transit_serialize(&serializer)).collect();
        JsVal::Array(v_ser)
    }

    fn serialize_map_iter<'t, K, V, I>(&self, v: I) -> Self::Output
    where
        K: TransitSerialize + 't,
        V: TransitSerialize + 't,
        I: Iterator<Item = (&'t K, &'t V)>,
    {
        let serializer = Self::default();
        let mut has_comp_key = false;
        let (ser_k, ser_v): (Vec<JsVal>, Vec<JsVal>) = v
            .map(|(key, value)| {
                (
                    {
                        let k = key.transit_serialize_key(&serializer);
                        match k {
                            Some(x) => JsVal::String(x),
                            None => {
                                has_comp_key = true;
                                key.transit_serialize(&serializer)
                            }
                        }
                    },
                    value.transit_serialize(&serializer),
                )
            })
            .unzip();

        if has_comp_key {
            let interleaved: Vec<JsVal> = ser_k.into_iter().interleave(ser_v).collect();
            let mut m = JsMap::with_capacity(1);
            m.insert("~#cmap".to_owned(), JsVal::Array(interleaved));
            JsVal::Object(m)
        } else {
            let mut m = JsMap::with_capacity(ser_k.len());
            for (key, value) in ser_k.into_iter().zip(ser_v) {
                m.insert(
                    key.as_str()
                        .expect("Scalar keys are always strings")
                        .to_owned(),
                    value,
                );
            }
            JsVal::Object(m)
        }
    }

    fn serialize_tagged_array_iter<'t, T, I>(&self, tag: &str, v: I) -> Self::Output
    where
        T: TransitSerialize + 't,
        I: Iterator<Item = &'t T>,
    {
        let v_ser = self.serialize_array_iter(v);
        let mut m = JsMap::with_capacity(1);
        m.insert(tag.to_owned(), v_ser);
        JsVal::Object(m)
    }

    fn serialize_tagged_map_iter<'t, K, V, I>(&self, tag: &str, v: I) -> Self::Output
    where
        K: TransitSerialize + 't,
        V: TransitSerialize + 't,
        I: Iterator<Item = (&'t K, &'t V)>,
    {
        let m_ser = self.serialize_map_iter(v);
        let mut m = JsMap::with_capacity(1);
        m.insert(tag.to_owned(), m_ser);
        JsVal::Object(m)
    }
}

impl TransitKeySerializer for JsonSerializer {
    type Output = String;

    fn serialize_key(&self, v: &str) -> Self::Output {
        v.to_owned()
    }
}

pub struct JsonArraySerializer {
    buf: Vec<JsVal>,
    inner_serializer: JsonSerializer,
}

pub struct JsonMapSerializer {
    buf_keys: Vec<JsVal>,
    buf_vals: Vec<JsVal>,
    cmap: bool,
    inner_serializer: JsonSerializer,
}

impl TransitMapSerializer for JsonMapSerializer {
    type Output = JsVal;

    fn serialize_pair<K: TransitSerialize, V: TransitSerialize>(&mut self, k: &K, v: &V) {
        if let Some(x) = k.transit_serialize_key(&self.inner_serializer) {
            self.buf_keys.push(JsVal::String(x));
        } else {
            self.cmap = true;
            self.buf_keys
                .push(k.transit_serialize(&self.inner_serializer));
        }
        self.buf_vals
            .push(v.transit_serialize(&self.inner_serializer));
    }

    fn end(self) -> Self::Output {
        if self.cmap {
            let interleaved: Vec<JsVal> = self
                .buf_keys
                .into_iter()
                .interleave(self.buf_vals)
                .collect();
            let mut m = JsMap::with_capacity(1);
            m.insert("~#cmap".to_owned(), JsVal::Array(interleaved));
            JsVal::Object(m)
        } else {
            let mut m = JsMap::with_capacity(self.buf_keys.len());
            for (key, value) in self.buf_keys.into_iter().zip(self.buf_vals) {
                m.insert(
                    key.as_str()
                        .expect("Scalar keys are always strings")
                        .to_owned(),
                    value,
                );
            }
            JsVal::Object(m)
        }
    }
}

impl TransitArraySerializer for JsonArraySerializer {
    type Output = JsVal;
    fn serialize_item<T: TransitSerialize>(&mut self, v: &T) {
        self.buf.push(v.transit_serialize(&self.inner_serializer));
    }

    fn end(self) -> Self::Output {
        JsVal::Array(self.buf)
    }
}

struct JsonTaggedArraySerializer {
    tag: String,
    array_serializer: JsonArraySerializer,
}

struct JsonTaggedMapSerializer {
    tag: String,
    map_serializer: JsonMapSerializer,
}

impl TransitTaggedArraySerializer for JsonTaggedArraySerializer {
    type Output = JsVal;

    fn serialize_item<T: TransitSerialize>(&mut self, v: &T) {
        self.array_serializer.serialize_item(v);
    }

    fn end(self) -> Self::Output {
        let val = self.array_serializer.end();
        let mut m = JsMap::with_capacity(1);
        m.insert(self.tag, val);
        JsVal::Object(m)
    }
}

impl TransitTaggedMapSerializer for JsonTaggedMapSerializer {
    type Output = JsVal;

    fn serialize_pair<K: TransitSerialize, V: TransitSerialize>(&mut self, k: &K, v: &V) {
        self.map_serializer.serialize_pair(k, v);
    }

    fn end(self) -> Self::Output {
        let val = self.map_serializer.end();
        let mut m = JsMap::with_capacity(1);
        m.insert(self.tag, val);
        JsVal::Object(m)
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
