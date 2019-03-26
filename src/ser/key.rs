/// Serialize data at Map/Struct key position.
///
/// As correct JSON object may have only string
/// keys whereas Transit format has no such restriction
/// `serialize_key` will be called on data to be placed
/// as key.
trait TfSerializeKey {
    fn serialize_key(&self) -> String;
}

impl TfSerializeKey for String {
    fn serialize_key(&self) -> String {
        self.clone()
    }
}

impl TfSerializeKey for bool {
    fn serialize_key(&self) -> String {
        if *self {
            "~?t".to_owned()
        } else {
            "~?f".to_owned()
        }
    }
}

impl TfSerializeKey for i32 {
    fn serialize_key(&self) -> String {
        format!("~i{}", self)
    }
}
impl TfSerializeKey for i64 {
    fn serialize_key(&self) -> String {
        format!("~i{}", self)
    }
}
impl TfSerializeKey for u32 {
    fn serialize_key(&self) -> String {
        format!("~i{}", self)
    }
}

impl TfSerializeKey for f32 {
    fn serialize_key(&self) -> String {
        format!("~d{}", self)
    }
}
impl TfSerializeKey for f64 {
    fn serialize_key(&self) -> String {
        format!("~d{}", self)
    }
}
