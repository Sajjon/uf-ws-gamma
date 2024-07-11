mod models;

pub mod prelude {
    pub use crate::models::*;
    pub use zero::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("one");
