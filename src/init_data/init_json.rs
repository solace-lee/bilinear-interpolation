extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::BufReader;


#[derive(Serialize, Deserialize, Debug)]
pub struct ImageData {
  pub data: Vec<Vec<Vec<f32>>>,
  pub width: u32,
  pub height: u32,
  pub dst_width: u32,
  pub dst_height: u32,
}

impl ImageData {
  pub fn new(path: &str) -> Result<ImageData> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let v: ImageData = serde_json::from_reader(reader)?;
    Ok(v)
  }
}