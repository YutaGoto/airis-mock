use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShapeCodeList {
    pub shape_codes: Vec<ShapeCode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShapeCode {
    pub classification: Option<String>,
    pub purpose: Option<String>,
    pub code: u32,
    pub body_type: String,
    pub remarks: Option<String>,
}

pub fn read_shape_codes() -> Vec<ShapeCode> {
    let shape_codes = include_str!("../static/vehicle_shape_codes.json");
    let shape_codes: Vec<ShapeCode> = serde_json::from_str(shape_codes).unwrap();
    shape_codes
}
