#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
  pub EnableContextEncoding: bool,
  pub DisablePayloadHandler: bool,
  pub SSL: bool,
  pub SSLVersion: String,
  pub PAYLOAD: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct info {
  pub jid: i32,
  pub name: String,
  pub start_time: i32,
  pub uripath: String,
  pub datastore: Data,
}

#[derive(Deserialize, Debug)]
pub struct stop {
  pub result: String,
}
