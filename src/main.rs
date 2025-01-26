use axum::{
    Router,
    http::HeaderValue,
    response::{IntoResponse, Response},
    routing::{get, post},
    Form,
};
use chrono::{DateTime, Duration, FixedOffset};
use quick_xml::se::to_string;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ShapeCode {
    code: String,
    body_type: String,
    remarks: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BodyType {
    name: String,
    cd: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TeikyouUniqueSearchServletRequest {
    searchdate: String,
    searchid: String,
    privacyflg: String,
    seqno: String,
    retryflg: String,
    groupid: String,
    regno: String,
    chassisno: String,
    version: String,
    userid: String,
    pw: String,
    keyword: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TeikyouUniqueSearchServletResponse {
    searchdate: String,
    searchid: String,
    privacyflg: String,
    seqno: String,
    retryflg: String,
    groupid: String,
    regno: String,
    chassisno: String,
    version: String,
    userid: String,
    pw: String,
    keyword: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AirisResponse {
    response: AirisResponseBody,
}

#[derive(Serialize, Deserialize, Debug)]
struct AirisResponseBody {
    common: AirisCommon,
    data: Option<AirisData>,
    errinfo: Option<AirisErrInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AirisCommon {
    orgname: String,
    version: String,
    searchdate: String,
    searchid: String,
    seqno: String,
    result: String,
    num: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AirisErrInfo {
    errid: String,
    errmsg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AirisData {
    regdate: String,
    firstregdate: String,
    purpose: String,
    bodytype: BodyType,
    loadage: LoadAgeType,
    weight: WeightType,
    grossweight: GrossWeightType,
    expirydate: String,
    carid: Option<String>,
    electro_carins: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LoadAgeType {
    value_1: String,
    value_2: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeightType {
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GrossWeightType {
    value_1: String,
    value_2: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ShapeCodeList {
    shape_codes: Vec<ShapeCode>
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/body-types", get(get_body_types))
        .route("/health-check", get(health_check))
        .route("/teikyou/check", post(check_teikyou_unique_search_servlet))
        .route(
            "/teikyou/UniqueSearchServlet",
            post(teikyou_unique_search_servlet),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4567").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}


// Create a wrapper type for XML responses
struct Xml<T>(T);

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

async fn handler() -> Xml<BodyType> {
    Xml(get_random_body_type())
}

async fn get_body_types() -> Xml<ShapeCodeList> {
    Xml(ShapeCodeList {
        shape_codes: read_shape_codes()
    })
}

async fn health_check() -> Response {
    Response::new("OK".into())
}

async fn check_teikyou_unique_search_servlet(
    body: Form<TeikyouUniqueSearchServletRequest>,
) -> Xml<TeikyouUniqueSearchServletResponse> {
    Xml(TeikyouUniqueSearchServletResponse {
        searchdate: body.searchdate.clone(),
        searchid: body.searchid.clone(),
        privacyflg: body.privacyflg.clone(),
        seqno: body.seqno.clone(),
        retryflg: body.retryflg.clone(),
        groupid: body.groupid.clone(),
        regno: body.regno.clone(),
        chassisno: body.chassisno.clone(),
        version: body.version.clone(),
        userid: body.userid.clone(),
        pw: body.pw.clone(),
        keyword: body.keyword.clone(),
    })
}

fn read_shape_codes() -> Vec<ShapeCode> {
    let shape_codes = include_str!("static/vehicle_shape_codes.json");
    let shape_codes: Vec<ShapeCode> = serde_json::from_str(shape_codes).unwrap();
    shape_codes
}

fn get_random_body_type() -> BodyType {
    let shape_codes = read_shape_codes();
    let random_index = rand::thread_rng().gen_range(0..shape_codes.len());
    BodyType {
        name: shape_codes[random_index].body_type.clone(),
        cd: shape_codes[random_index].code.clone(),
    }
}

async fn teikyou_unique_search_servlet(
    body: Form<TeikyouUniqueSearchServletRequest>,
) -> Xml<AirisResponse> {
    let search_date: DateTime<FixedOffset> = DateTime::parse_from_str(
        format!("{} 00:00:00 +0900", body.searchdate).as_str(),
        "%Y%m%d %H:%M:%S %z",
    )
    .unwrap();

    let response = AirisResponse {
        response: AirisResponseBody {
            common: AirisCommon {
                orgname: "test".to_string(),
                version: "1.0.0".to_string(),
                searchdate: search_date.format("%Y%m%d").to_string(),
                searchid: body.searchid.clone(),
                seqno: body.seqno.clone(),
                result: "success".to_string(),
                num: "1".to_string(),
            },
            data: Some(AirisData {
                regdate: (search_date - Duration::days(365))
                    .format("%Y%m%d")
                    .to_string(),
                firstregdate: (search_date - Duration::days(365))
                    .format("%Y%m%d")
                    .to_string(),
                purpose: "事業用".to_string(),
                bodytype: get_random_body_type(),
                loadage: LoadAgeType {
                    value_1: "4000".to_string(),
                    value_2: "3000".to_string(),
                },
                weight: WeightType {
                    value: "2250".to_string(),
                },
                grossweight: GrossWeightType {
                    value_1: "6800".to_string(),
                    value_2: "5980".to_string(),
                },
                expirydate: (search_date + Duration::days(365 * 3))
                    .format("%Y%m%d")
                    .to_string(),
                carid: Some(body.chassisno.clone()),
                electro_carins: "1".to_string(),
            }),
            errinfo: None,
        },
    };
    Xml(response)
}
