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
