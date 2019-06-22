mod impls;
//pub mod json;
pub mod json_verbose;

pub enum TransitType<T> {
    Scalar(T),
    Composite(T),
}

impl<T> TransitType<T> {
    pub fn unpack(self) -> T {
        match self {
            TransitType::Scalar(x) => x,
            TransitType::Composite(x) => x,
        }
    }
}

pub trait TransitSerialize {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output>;
    fn transit_serialize_key<S: TransitSerializer>(&self, serializer: &S)
        -> TransitType<S::Output>;
}

/// Trait for creation of final representation
/// because Transit is generic over JSON and MessagePack (both Verbose and not)
pub trait TransitSerializer {
    type Output;

    fn serialize_null(&self) -> TransitType<Self::Output>;
    fn serialize_string(&self, v: &str) -> TransitType<Self::Output>;
    fn serialize_bool(&self, v: bool) -> TransitType<Self::Output>;
    fn serialize_int(&self, v: i64) -> TransitType<Self::Output>;
    fn serialize_float(&self, v: f64) -> TransitType<Self::Output>;

    fn serialize_array<'t, T, I>(&self, v: I) -> TransitType<Self::Output>
    where
        T: TransitSerialize + 't,
        I: Iterator<Item = &'t T>;

    fn serialize_map<'t, K, V, I>(&self, v: I) -> TransitType<Self::Output>
    where
        K: TransitSerialize + 't,
        V: TransitSerialize + 't,
        I: Iterator<Item = (&'t K, &'t V)>;

    fn serialize_tagged_array<'t, T, I>(&self, tag: &str, v: I) -> TransitType<Self::Output>
    where
        T: TransitSerialize + 't,
        I: Iterator<Item = &'t T>;

    fn serialize_tagged_map<'i, 't, K, V, I>(&self, tag: &str, v: I) -> TransitType<Self::Output>
    where
        K: TransitSerialize + 't,
        V: TransitSerialize + 't,
        I: Iterator<Item = (&'t K, &'t V)>;
}
