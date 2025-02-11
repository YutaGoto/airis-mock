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
    let random_index = rand::thread_rng().gen_range(0..shape_codes.len());
    BodyType {
        name: shape_codes[random_index].body_type.clone(),
        cd: shape_codes[random_index].code,
    }
}

pub fn get_body_type(chassisno: String) -> BodyType {
    let v = chassisno.split("-").collect::<Vec<&str>>();
    let num = v[0].to_string();
    let shape_codes = read_shape_codes();

    let result = shape_codes.iter().find(|shape_code| {
        let code = shape_code.code.to_string();
        let num_str = num.chars().take(2).collect::<String>();
        code.starts_with(&num_str)
    });

    if let Some(shape_code) = result {
        BodyType {
            name: shape_code.body_type.clone(),
            cd: shape_code.code,
        }
    } else {
        get_random_body_type()
    }
}
