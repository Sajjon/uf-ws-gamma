use std::sync::Arc;

use zero::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Record)]
pub struct OneRecord {
    zero_record: ZeroRecord,
    zero_object: Arc<ZeroObject>,
}

#[uniffi::export]
pub fn new_one_default() -> OneRecord {
    OneRecord::default()
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Object)]
pub struct OneObject {
    one: OneRecord,
    zero_record: ZeroRecord,
    zero_object: Arc<ZeroObject>,
}

#[uniffi::export]
impl OneObject {
    #[uniffi::constructor]
    pub fn new(one: OneRecord, zero_record: ZeroRecord, zero_object: Arc<ZeroObject>) -> Self {
        Self {
            one,
            zero_record,
            zero_object,
        }
    }
}
