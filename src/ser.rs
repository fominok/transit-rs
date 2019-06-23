mod impls;
//pub mod json;
pub mod json_verbose;

pub trait TransitSerialize {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output;
    fn transit_serialize_key<S: TransitSerializer>(&self, serializer: &S) -> Option<S::Output>;
}

/// Trait for creation of final representation
/// because Transit is generic over JSON and MessagePack (both Verbose and not)
pub trait TransitSerializer {
    type Output;

    fn serialize_null(&self) -> Self::Output;
    fn serialize_string(&self, v: &str) -> Self::Output;
    fn serialize_bool(&self, v: bool) -> Self::Output;
    fn serialize_int(&self, v: i64) -> Self::Output;
    fn serialize_float(&self, v: f64) -> Self::Output;

    fn serialize_array<'t, T, I>(&self, v: I) -> Self::Output
    where
        T: TransitSerialize + 't,
        I: Iterator<Item = &'t T>;

    fn serialize_map<'t, K, V, I>(&self, v: I) -> Self::Output
    where
        K: TransitSerialize + 't,
        V: TransitSerialize + 't,
        I: Iterator<Item = (&'t K, &'t V)>;

    fn serialize_tagged_array<'t, T, I>(&self, tag: &str, v: I) -> Self::Output
    where
        T: TransitSerialize + 't,
        I: Iterator<Item = &'t T>;

    fn serialize_tagged_map<'i, 't, K, V, I>(&self, tag: &str, v: I) -> Self::Output
    where
        K: TransitSerialize + 't,
        V: TransitSerialize + 't,
        I: Iterator<Item = (&'t K, &'t V)>;
}
