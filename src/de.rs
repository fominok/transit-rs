use serde_json::{
    map::{IntoIter as JsMapIntoIter, Map as JsMap},
    Value as JsVal,
};
use std::collections::BTreeMap;

enum Error {
    DoNotMatch(String),
}

type TResult<T> = Result<T, Error>;

trait TransitDeserialize {
    fn transit_deserialize<D: TransitDeserializer>(deserializer: D) -> Self;
}

trait TransitDeserializer {
    type MapVisitor: Iterator;

    fn visit_map(self) -> TResult<Self::MapVisitor>;
}

trait MapVisitor {}

struct JsonDeserializer {
    input: JsVal,
}

struct JsonMapVisitor {
    input: JsMap<String, JsVal>,
}

impl MapVisitor for JsonMapVisitor {}

impl TransitDeserializer for JsonDeserializer {
    type MapVisitor = JsMapIntoIter;

    fn visit_map(self) -> TResult<Self::MapVisitor> {
        if let JsVal::Object(m) = self.input {
            Ok(m.into_iter())
        } else {
            Err(Error::DoNotMatch(format!("{} is not a map", self.input)))
        }
    }
}

impl<K, V> TransitDeserialize for BTreeMap<K, V> {
    fn transit_deserialize<D: TransitDeserializer>(deserializer: D) -> Self {
        unimplemented!()
    }
}

fn from_transit_json<T: TransitDeserialize>(v: JsVal) -> T {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn scalar_map_btree() {
        let mut m = BTreeMap::new();
        m.insert(4, "yolo");
        m.insert(-6, "swag");

        let tr = from_transit_json(json!({
            "~i4": "yolo",
            "~i-6": "swag"
        }));
        assert_eq!(m, tr);
    }
}
