#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Record)]
pub struct ZeroRecord {
    value: bool,
}

#[uniffi::export]
pub fn new_zero(value: bool) -> ZeroRecord {
    ZeroRecord { value }
}

#[uniffi::export]
pub fn new_zero_default() -> ZeroRecord {
    ZeroRecord::default()
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Object)]
pub struct ZeroObject {
    value: ZeroRecord,
}

#[uniffi::export]
impl ZeroObject {
    #[uniffi::constructor]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[uniffi::constructor]
    pub fn new(value: ZeroRecord) -> Self {
        Self { value }
    }
}
