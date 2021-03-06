//! Core of rbatis::core, the rust SQL toolkit. Not intended to be used directly.

// #![warn(missing_docs)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_json;

pub use error::{Error, Result};

pub mod runtime;

#[cfg(feature = "mssql")]
mod mssql;
/// database
#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;
#[macro_use]
pub mod error;
pub mod convert;
pub mod db;
pub mod decode;
pub mod results;
pub mod sync;
pub mod value;
