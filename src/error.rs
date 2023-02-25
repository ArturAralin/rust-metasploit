//! This module is to handle all error responses from RPC Server and to handle Connection Error.

use serde::Deserialize;
use std::env::var;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

/// This is to handle connection errors.For more information refer [reqwest::Error](https://docs.rs/reqwest/0.7.2/reqwest/struct.Error.html)
pub type ConnectionError = reqwest::Error;

/// Struct to handle RPC error Responses
#[derive(Deserialize, Debug)]
/// ```
/// pub struct MsfError {/*... */}
/// ```
/// Struct to handle error responses from RPC Server
pub struct MsfError {
  /// Boolean value for error
  pub error: bool,
  /// Class of error
  pub error_class: String,
  /// Error description
  pub error_string: String,
  /// Error Message
  pub error_message: String,
  /// Error Backtrace
  pub error_backtrace: Vec<String>,
}

impl Error for MsfError {}

impl Display for MsfError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let err = match var("RUST_BACKTRACE").as_deref() {
      Ok(val) => {
        if val == "1" {
          format!("({},{})", self.error_message, self.error_class)
        } else if val == "full" {
          format!("{:?}", self)
        } else {
          self.error_message.clone()
        }
      }
      Err(_) => self.error_message.clone(),
    };

    write!(f, "{}", err)
  }
}

#[derive(Debug)]
pub enum RpcError {
  MsfError(MsfError),
  InvalidResponse(rmp_serde::decode::Error),
  InvalidRequestArgs(rmp_serde::encode::Error),
}

impl From<rmp_serde::decode::Error> for RpcError {
  fn from(error: rmp_serde::decode::Error) -> Self {
    Self::InvalidResponse(error)
  }
}

impl From<rmp_serde::encode::Error> for RpcError {
  fn from(error: rmp_serde::encode::Error) -> Self {
    Self::InvalidRequestArgs(error)
  }
}
