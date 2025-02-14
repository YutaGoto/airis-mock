use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::models::shape_code::read_shape_codes;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE", rename = "BODYTYPE")]
pub struct BodyType {
    pub name: String,
    pub cd: u32,
}

pub fn get_random_body_type() -> BodyType {
    let shape_codes = read_shape_codes();
    let random_index = rand::rng().random_range(0..shape_codes.len());
    BodyType {
        name: shape_codes[random_index].body_type.clone(),
        cd: shape_codes[random_index].code,
    }
}
