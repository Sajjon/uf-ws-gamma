use zero::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Record)]
pub struct One {
    value: Zero,
}

#[uniffi::export]
pub fn new_one_default() -> One {
    One::default()
}

#[uniffi::export]
pub fn new_one(value: Zero) -> One {
    One { value }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Object)]
pub struct OneObj {
    value: Zero,
}

#[uniffi::export]
impl OneObj {
    #[uniffi::constructor]
    pub fn new_default() -> Self {
        Self::default()
    }
}
