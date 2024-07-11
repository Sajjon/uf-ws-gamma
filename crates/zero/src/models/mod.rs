#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Record)]
pub struct Zero {
    value: bool,
}

#[uniffi::export]
pub fn new_zero(value: bool) -> Zero {
    Zero { value }
}

#[uniffi::export]
pub fn new_zero_default() -> Zero {
    Zero::default()
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Object)]
pub struct ZeroObj {
    value: Zero,
}

#[uniffi::export]
impl ZeroObj {
    #[uniffi::constructor]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[uniffi::constructor]
    pub fn new(value: Zero) -> Self {
        Self { value }
    }
}
