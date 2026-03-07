#[path = "error.rs"]
mod error_mod;

#[path = "img.rs"]
mod img_mod;

pub use error_mod::error;
pub use img_mod::img;
