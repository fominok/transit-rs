use serde_json::{map::Map as JsMap, Value as JsVal};
use std::collections::BTreeMap;

pub fn to_transit_json<T: TransitSerialize>(v: T) -> JsVal {
    v.transit_serialize(JsonSerializer::top())
}

#[derive(PartialEq)]
pub enum TransitType {
    Scalar,
    Composite,
}

// FIXME: remove Clone
pub trait TransitSerialize: Clone {
    const TF_TYPE: TransitType;
    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output;
    fn transit_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output>;
}

impl TransitSerialize for bool {
    const TF_TYPE: TransitType = TransitType::Scalar;
    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        serializer.serialize_bool(*self)
    }

    fn transit_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output> {
        let s = if *self {
            "~?t".to_owned()
        } else {
            "~?f".to_owned()
        };

        Some(serializer.serialize_string(&s))
    }
}

impl<K: TransitSerialize, V: TransitSerialize> TransitSerialize for BTreeMap<K, V> {
    const TF_TYPE: TransitType = TransitType::Composite;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        let mut ser_map = serializer.serialize_map(Some(self.len()));
        for (k, v) in self.iter() {
            ser_map.serialize_pair((*k).clone(), (*v).clone());
        }
        ser_map.end()
    }

    fn transit_key<S: TransitSerializer>(&self, _serializer: S) -> Option<S::Output> {
        None
    }
}

impl TransitSerialize for i32 {
    const TF_TYPE: TransitType = TransitType::Scalar;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        serializer.serialize_int((*self).into())
    }

    fn transit_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output> {
        Some(serializer.serialize_string(&format!("~i{}", self)))
    }
}

impl TransitSerialize for String {
    const TF_TYPE: TransitType = TransitType::Scalar;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        serializer.serialize_string(self)
    }

    fn transit_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output> {
        Some(serializer.serialize_string(self))
    }
}

impl TransitSerialize for &str {
    const TF_TYPE: TransitType = TransitType::Scalar;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        serializer.serialize_string(self)
    }

    fn transit_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output> {
        Some(serializer.serialize_string(self))
    }
}

/// Trait for creation of final representation
/// because Transit is generic over JSON and MessagePack (both Verbose and not)
pub trait TransitSerializer {
    type Output;
    type SerializeArray: SerializeArray<Output = Self::Output>;
    type SerializeMap: SerializeMap<Output = Self::Output>;

    fn serialize_null(self) -> Self::Output;
    fn serialize_string(self, v: &str) -> Self::Output;
    fn serialize_bool(self, v: bool) -> Self::Output;
    fn serialize_int(self, v: i64) -> Self::Output;
    fn serialize_float(self, v: f64) -> Self::Output;
    fn serialize_array(self, len: Option<usize>) -> Self::SerializeArray;
    fn serialize_map(self, len: Option<usize>) -> Self::SerializeMap;
}

/// Array-specific serialization
pub trait SerializeArray {
    type Output;

    fn serialize_item<T: TransitSerialize>(&mut self, v: T);
    fn end(self) -> Self::Output;
}

/// Map-specific serialization
pub trait SerializeMap {
    type Output;

    fn serialize_pair<K: TransitSerialize, V: TransitSerialize>(&mut self, k: K, v: V);
    fn end(self) -> Self::Output;
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
            .push(k.transit_key(JsonSerializer::default()));
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

impl TransitSerializer for JsonSerializer {
    type Output = JsVal;
    type SerializeArray = JsonArraySerializer;
    type SerializeMap = JsonMapSerializer;

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
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

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
    fn test_quote() {
        let tr = to_transit_json(5i32);
        assert_eq!(
            json!({
                "~#": 5
            }),
            tr
        );
    }
}
