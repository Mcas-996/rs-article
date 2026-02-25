#[cfg(feature = "web")]
mod web;

#[cfg(feature = "native")]
pub mod native {
    include!("main.rs");
}
