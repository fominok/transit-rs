use serde_json::{map::Map as JsMap, Value as JsVal};
use std::collections::BTreeMap;

trait TransitDeserializer {}

trait TransitDeserialize {
    fn transit_deserialize<D: TransitDeserializer>(deserializer: D) -> Self;
}

struct JsonDeserializer {
    input: JsVal,
}

impl TransitDeserializer for JsonDeserializer {}

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
