use serde_json::{
    map::{IntoIter as JsMapIntoIter, Map as JsMap},
    Value as JsVal,
};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt::Debug;

#[derive(Debug)]
enum Error {
    DoNotMatch(String),
    ItWontFit(String),
}

type TResult<T> = Result<T, Error>;

#[derive(PartialEq)]
pub enum TransitType {
    Scalar,
    Composite,
}

/*// FIXME: remove Clone
pub trait TransitSerialize: Clone {
    const TF_TYPE: TransitType;
    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output;
    fn transit_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output>;
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
}*/

trait TransitDeserialize: Sized {
    fn transit_deserialize<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self>;
}

trait TransitDeserializer: Clone + Debug {
    type Input: Debug;
    type DeserializeArray: IntoIterator<Item = Self::Input>;
    type DeserializeMap;

    //fn deserialize_null(self, v: Self::Input) -> Self::Output;
    fn deserialize_string(self, v: Self::Input) -> TResult<String>;
    fn deserialize_bool(self, v: Self::Input) -> TResult<bool>;
    fn deserialize_int(self, v: Self::Input) -> TResult<i64>;
    fn deserialize_float(self, v: Self::Input) -> TResult<f64>;
    fn deserialize_array(self, v: Self::Input) -> TResult<Self::DeserializeArray>;
    fn deserialize_map(self, v: Self::Input) -> TResult<Self::DeserializeMap>;
}

#[derive(Clone, Debug)]
struct JsonDeserializer;

impl TransitDeserializer for JsonDeserializer {
    type Input = JsVal;
    type DeserializeArray = std::vec::IntoIter<JsVal>;
    type DeserializeMap = i32;

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

    fn deserialize_array(self, v: Self::Input) -> TResult<Self::DeserializeArray> {
        v.as_array()
            .map(|x| x.clone().into_iter())
            .ok_or(Error::DoNotMatch(format!("{} is not array", v)))
    }

    fn deserialize_map(self, v: Self::Input) -> TResult<Self::DeserializeMap> {
        unimplemented!()
    }

    // fn visit_map(self) -> TResult<Self::MapVisitor> {
    //     if let JsVal::Object(m) = self.input {
    //         Ok(m.into_iter())
    //     } else {
    //         Err(Error::DoNotMatch(format!("{} is not a map", self.input)))
    //     }
    // }
}

// impl<K, V> TransitDeserialize for BTreeMap<K, V> {
//     fn transit_deserialize<D: TransitDeserializer>(deserializer: D) -> TResult<Self> {
//         let visitor = deserializer.visit_map()?;
//         unimplemented!()
//     }
// }
//
impl<T: TransitDeserialize> TransitDeserialize for Vec<T> {
    fn transit_deserialize<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        let array_iter = deserializer.clone().deserialize_array(input)?;
        let mut v = Vec::new();
        for x in array_iter {
            v.push(TransitDeserialize::transit_deserialize(
                deserializer.clone(),
                x,
            )?);
        }
        Ok(v)
    }
}

impl TransitDeserialize for i32 {
    fn transit_deserialize<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        Self::try_from(deserializer.deserialize_int(input)?)
            .map_err(|_| Error::ItWontFit(format!("Cannot fit in i32")))
    }
}

fn from_transit_json<T: TransitDeserialize>(v: JsVal) -> TResult<T> {
    TransitDeserialize::transit_deserialize(JsonDeserializer, v)
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn dumb_array() {
        let mut v = vec![1, 2, 3];
        let tr: Vec<i32> = from_transit_json(json!([1, 2, 3])).unwrap();
        assert_eq!(v, tr);
    }

    // #[test]
    // fn scalar_map_btree() {
    //     let mut m = BTreeMap::new();
    //     m.insert(4, "yolo");
    //     m.insert(-6, "swag");

    //     let tr = from_transit_json(json!({
    //         "~i4": "yolo",
    //         "~i-6": "swag"
    //     }));
    //     assert_eq!(m, tr);
    // }
}
