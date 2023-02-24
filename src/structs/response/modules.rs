#![allow(non_camel_case_types)]
use crate::value::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct list {
  pub modules: Vec<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct info {
  pub name: String,
  pub description: String,
  pub license: Vec<String>,
  pub filepath: String,
  pub version: Option<String>,
  pub rank: String,
  pub authors: Vec<String>,
  pub references: Vec<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct options {
  pub r#type: String,
  pub required: bool,
  pub advanced: bool,
  pub evasion: bool,
  pub desc: String,
  pub default: Option<Value>,
  pub enums: Option<Vec<String>>,
}
#[derive(Deserialize, Debug)]
pub struct compactible_payloads {
  pub payloads: Vec<String>,
}
#[derive(Deserialize, Debug)]
pub struct compactible_sessions {
  pub sessions: Vec<String>,
}
#[derive(Deserialize, Debug)]
pub struct encode {
  pub encoded: String,
}

#[derive(Deserialize, Debug)]
pub struct execute_non_payloads {
  pub job_id: i32,
}
#[derive(Deserialize, Debug)]
pub struct execute_payloads {
  pub payload: Value,
}

#[derive(Deserialize, Debug)]
pub enum ExecuteResponse {
  Payload(execute_payloads),
  NonPayload(execute_non_payloads),
}
