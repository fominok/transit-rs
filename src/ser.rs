mod impls;
pub mod json;
pub mod json_verbose;

#[derive(PartialEq)]
pub enum TransitType {
    Scalar,
    Composite,
}

// FIXME: remove Clone
pub trait TransitSerialize: Clone {
    const TF_TYPE: TransitType;
    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output;
    fn transit_serialize_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output>;
}

/// Trait for creation of final representation
/// because Transit is generic over JSON and MessagePack (both Verbose and not)
pub trait TransitSerializer: Clone {
    type Output;
    type SerializeArray: SerializeArray<Output = Self::Output>;
    type SerializeMap: SerializeMap<Output = Self::Output>;
    type SerializeTag: SerializeTag<Output = Self::Output>;

    fn serialize_null(self) -> Self::Output;
    fn serialize_string(self, v: &str) -> Self::Output;
    fn serialize_bool(self, v: bool) -> Self::Output;
    fn serialize_int(self, v: i64) -> Self::Output;
    fn serialize_float(self, v: f64) -> Self::Output;
    fn serialize_array(self, len: Option<usize>) -> Self::SerializeArray;

    // Like what should be an object semantically
    fn serialize_map(self, len: Option<usize>) -> Self::SerializeMap;

    // Tagged value is not equivalent for object
    fn serialize_tagged(self, tag: &str) -> Self::SerializeTag;
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

/// Tags serialization
pub trait SerializeTag {
    type Output;

    fn serialize_value(&mut self, v: Self::Output) -> Self::Output;
}
