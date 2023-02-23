#![allow(non_camel_case_types)]
use serde::Serialize as se;
use std::collections::HashMap;

#[derive(se)]
pub struct load(
  pub String,
  pub String,
  pub String,
  pub HashMap<String, String>,
);
#[derive(se)]
pub struct unload(pub String, pub String, pub String);
#[derive(se)]
pub struct loaded(pub String, pub String);
