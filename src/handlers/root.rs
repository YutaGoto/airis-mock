use crate::models::body_type::{get_random_body_type, BodyType};
use crate::models::xml::Xml;

pub async fn root() -> Xml<BodyType> {
    Xml(get_random_body_type())
}
