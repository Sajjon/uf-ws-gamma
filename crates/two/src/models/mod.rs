use one::{One, OneObj};
use std::sync::Arc;
use zero::{Zero, ZeroObj};

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

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Record)]
pub struct ComplexRecord {
    a: Zero,
    b: One,
    c: GammaRecord,
    x: Arc<ZeroObj>,
    y: Arc<OneObj>,
    z: Arc<GammaObject>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Object)]
pub struct ComplexObject {
    a: Zero,
    b: One,
    c: GammaRecord,
    x: Arc<ZeroObj>,
    y: Arc<OneObj>,
    z: Arc<GammaObject>,
}

#[uniffi::export]
pub fn complex_record(
    a: Zero,
    b: One,
    c: GammaRecord,
    x: Arc<ZeroObj>,
    y: Arc<OneObj>,
    z: Arc<GammaObject>,
) -> ComplexRecord {
    ComplexRecord { a, b, c, x, y, z }
}

#[uniffi::export]
pub fn complex_object(value: ComplexRecord) -> ComplexObject {
    ComplexObject {
        a: value.a,
        b: value.b,
        c: value.c,
        x: value.x,
        y: value.y,
        z: value.z,
    }
}
