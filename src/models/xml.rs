use axum::http::HeaderValue;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Serialize;
use serde_xml_rs::to_string;

// Create a wrapper type for XML responses
pub struct Xml<T>(pub T);

impl<T> IntoResponse for Xml<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        // Convert the type to XML string
        match to_string(&self.0) {
            Ok(xml) => {
                let mut res = Response::new(xml.into());
                res.headers_mut()
                    .insert("content-type", HeaderValue::from_static("application/xml"));
                res
            }
            Err(err) => {
                // Handle error case with a 500 response
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    err.to_string(),
                )
                    .into_response()
            }
        }
    }
}
