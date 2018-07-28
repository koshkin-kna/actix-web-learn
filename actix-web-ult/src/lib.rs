//! Actix web is a small, pragmatic, and extremely fast web framework
//! for Rust.
//!
//! ```rust
//! fn main() {
//! }
//! ```
//!
//! ## Documentation & community resources
//!
//! Besides the API documentation (which you are currently looking
//!
//! * [Link base actix-web](https://actix.rs/)
//!

extern crate actix_web;
extern crate regex;
extern crate tera;

pub mod tmp_engine;
pub mod middleware;
pub mod handlers;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
