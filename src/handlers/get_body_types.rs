use crate::models::shape_code::{read_shape_codes, ShapeCodeList};
use crate::models::xml::Xml;

pub async fn get_body_types() -> Xml<ShapeCodeList> {
    Xml(ShapeCodeList {
        shape_codes: read_shape_codes(),
    })
}
