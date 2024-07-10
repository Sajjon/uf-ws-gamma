use one::One;
use std::sync::Arc;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Two(bool);

uniffi::custom_newtype!(Two, bool);

#[uniffi::export]
pub fn new_two_default() -> Two {
    Two::default()
}

#[uniffi::export]
pub fn new_two(value: bool) -> Two {
    Two(value)
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Record)]
pub struct GammaRecord {
    one: One,
    two: Two,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Object)]
#[uniffi::export(Eq, Debug)]
pub struct GammaObject {
    one: One,
    two: Two,
}

#[uniffi::export]
impl GammaObject {
    #[uniffi::constructor]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[uniffi::constructor]
    pub fn new(one: One, two: Two) -> Self {
        Self { one, two }
    }
}

impl From<GammaObject> for GammaRecord {
    fn from(value: GammaObject) -> Self {
        Self {
            one: value.one,
            two: value.two,
        }
    }
}
impl From<GammaRecord> for GammaObject {
    fn from(value: GammaRecord) -> Self {
        Self {
            one: value.one,
            two: value.two,
        }
    }
}

#[uniffi::export]
pub fn new_record(one: One, two: Two) -> GammaRecord {
    GammaRecord { one, two }
}

#[uniffi::export]
pub fn new_record_default() -> GammaRecord {
    GammaRecord::default()
}

#[uniffi::export]
pub fn record_ref_record(value: &GammaRecord) -> GammaRecord {
    value.clone()
}

#[uniffi::export]
pub fn record_record(value: GammaRecord) -> GammaRecord {
    value
}

#[uniffi::export]
pub fn object_ref_object(value: &GammaObject) -> GammaObject {
    value.clone()
}

#[uniffi::export]
pub fn object_object(value: Arc<GammaObject>) -> Arc<GammaObject> {
    value
}

#[uniffi::export]
pub fn record_object(value: GammaRecord) -> GammaObject {
    value.into()
}

#[uniffi::export]
pub fn object_record(value: Arc<GammaObject>) -> GammaRecord {
    let raw = Arc::<GammaObject>::into_raw(value);
    let inner = unsafe { raw.as_ref() }.unwrap().clone();
    GammaRecord::from(inner)
}
