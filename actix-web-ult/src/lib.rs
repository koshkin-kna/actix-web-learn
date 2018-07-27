//! Actix web is a small, pragmatic, and extremely fast web framework
//! for Rust.
//!
//! ```rust
//! use actix_web::{server, App, Path, Responder};
//! # use std::thread;
//!
//! fn index(info: Path<(String, u32)>) -> impl Responder {
//!     format!("Hello {}! id:{}", info.0, info.1)
//! }
//!
//! fn main() {
//!     # thread::spawn(|| {
//!     server::new(|| {
//!         App::new().resource("/{name}/{id}/index.html", |r| r.with(index))
//!     }).bind("127.0.0.1:8080")
//!         .unwrap()
//!         .run();
//!     # });
//! }
//! ```
//!
//! ## Documentation & community resources
//!
//! Besides the API documentation (which you are currently looking
//! at!), several other resources are available:
//!
//! * [Link base actix-web](https://actix.rs/)
//!
//! To get started navigating the API documentation you may want to
//! consider looking at the following pages:


extern crate actix_web;

pub mod tmp_engine;
pub mod middleware;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
