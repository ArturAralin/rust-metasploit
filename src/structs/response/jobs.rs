#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Data {
  pub EnableContextEncoding: Option<bool>,
  pub DisablePayloadHandler: Option<bool>,
  pub SSL: Option<bool>,
  pub SSLVersion: Option<String>,
  pub PAYLOAD: Option<String>,
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
