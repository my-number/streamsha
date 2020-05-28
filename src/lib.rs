//! # StreamSha
//! Streamed, resumable Secure Hashing Algorithm(SHA) library
//!
//! ## Example
//! ```rust
//! # extern crate streamsha;
//! use streamsha::Sha256;
//! use streamsha::traits::{
//!   StreamHasher, Resumable
//! };
//! # fn main() {
//! // Simple hashing
//! let mut hasher = Sha256::new();
//! hasher.update(b"pien");
//! hasher.update(b"paon");
//! let hash1 = hasher.finish();
//!
//! // Pause
//! let mut hasher1 = Sha256::new();
//! hasher1.update(b"pien");
//! let state1 = hasher1.pause();
//! 
//! // Resume on the other instance
//! let mut hasher2 = Sha256::resume(state1).unwrap();
//! hasher2.update(b"paon");
//! let hash2 = hasher2.finish();
//! 
//! assert_eq!(hash1, hash2);
//! # }
//! ```

#![no_std]
#[macro_use]
mod utils;
mod consts;
pub mod traits;
mod sha1;
mod sha256;
mod sha512;
pub mod hash_state;

mod arith;
pub use self::sha1::Sha1;
pub use self::sha256::Sha256;
pub use self::sha512::Sha512;
