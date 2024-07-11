use one::{OneObject, OneRecord, ZeroObject, ZeroRecord};
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
    zero_record: ZeroRecord,
    zero_object: Arc<ZeroObject>,
    one_record: OneRecord,
    one_object: Arc<OneObject>,
    two: Two,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Object)]
#[uniffi::export(Eq, Debug)]
pub struct GammaObject {
    zero_record: ZeroRecord,
    zero_object: Arc<ZeroObject>,
    one_record: OneRecord,
    one_object: Arc<OneObject>,
    two: Two,
}

#[uniffi::export]
impl GammaObject {
    #[uniffi::constructor]
    pub fn new(
        zero_record: ZeroRecord,
        zero_object: Arc<ZeroObject>,
        one_record: OneRecord,
        one_object: Arc<OneObject>,
        two: Two,
    ) -> Self {
        Self {
            zero_record,
            zero_object,
            one_record,
            one_object,
            two,
        }
    }

    #[uniffi::constructor]
    pub fn new_default() -> Self {
        Self::default()
    }
}

#[uniffi::export]
pub fn record_to_object(record: GammaRecord) -> GammaObject {
    GammaObject {
        zero_record: record.zero_record,
        zero_object: record.zero_object,
        one_record: record.one_record,
        one_object: record.one_object,
        two: record.two,
    }
}
