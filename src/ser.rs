use std::collections::BTreeMap;
use serde_json::Value as JsVal;

trait TfSerialize {}


// Use this later for generic tagged val serilization
// impl<T: std::fmt::Debug> TfSerializeKey for T {
//     fn serialize_key(&self) -> String {
//         format!("{:?}", self)
//     }
// }

// trait TfSerializeMap {
//     fn serialize_map<K, V>
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_key() {
        // assert_eq!("5", 5_i32.serialize_key());
    }
}
