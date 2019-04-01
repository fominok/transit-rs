use serde_json::{Value as JsVal, json};
use std::collections::BTreeMap;

fn to_transit_json<T: TfSerialize>(v: T) -> JsVal {
    v.transit_serialize()
}

trait TfSerialize {
    fn transit_serialize(&self) -> JsVal;
}

enum TfType {
    Scalar,
    Composite,
}

trait HasTfType {
    fn get_transit_type(&self) -> TfType;
}

macro_rules! impl_comp {
    ($name:ty, $($type_params:tt),+) => {
        impl<$($type_params,)+> HasTfType for $name {
            fn get_transit_type(&self) -> TfType {
                TfType::Composite
            }
        }
    };
    ($name:ty) => {
        impl HasTfType for $name {
            fn get_transit_type(&self) -> TfType {
                TfType::Composite
            }
        }
    };
}

macro_rules! impl_scalar {
    ($name:ty) => {
        impl HasTfType for $name {
            fn get_transit_type(&self) -> TfType {
                TfType::Scalar
            }
        }
    };
}

impl_scalar!(i32);
impl_comp!(BTreeMap<K,V>, K, V);

impl<K: HasTfType, V: TfSerialize> TfSerialize for BTreeMap<K, V> {
    fn transit_serialize(&self) -> JsVal {

    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scalar_map() {
        let mut m = BTreeMap::new();
        m.insert(4, "yolo");
        m.insert(-6, "swag");

        let tr = to_transit_json(m);
        assert_eq!(json!({
            "~i4": "yolo",
            "~i-6": "swag"
        }), tr);

    }
}
