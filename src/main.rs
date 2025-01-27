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
#[serde(rename = "RESPONSE")]
struct AirisResponse {
    COMMON: AirisCommon,
    DATA: Option<AirisData>,
    ERRINFO: Option<AirisErrInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "COMMON")]
struct AirisCommon {
    ORGNAME: String,
    VERSION: String,
    SEARCHDATE: String,
    SEARCHID: String,
    SEQNO: String,
    RESULT: String,
    NUM: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "ERRINFO")]
struct AirisErrInfo {
    ERRID: String,
    ERRMSG: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "DATA")]
struct AirisData {
    REGDATE: String,
    FIRSTREGDATE: String,
    PURPOSE: String,
    BODYTYPE: BodyType,
    LOADAGE: LoadAgeType,
    WEIGHT: WeightType,
    GROSSWEIGHT: GrossWeightType,
    EXPIRYDATE: String,
    CARID: Option<String>,
    ELECTRO_CARINS: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "LOADAGE")]
struct LoadAgeType {
    VALUE_1: String,
    VALUE_2: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "WEIGHT")]
struct WeightType {
    VALUE: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "GROSSWEIGHT")]
struct GrossWeightType {
    VALUE_1: String,
    VALUE_2: String,
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
        COMMON: AirisCommon {
            ORGNAME: "test".to_string(),
            VERSION: "1.0.0".to_string(),
            SEARCHDATE: search_date.format("%Y%m%d").to_string(),
            SEARCHID: body.searchid.clone(),
            SEQNO: body.seqno.clone(),
            RESULT: "success".to_string(),
            NUM: "1".to_string(),
        },
        DATA: Some(AirisData {
            REGDATE: (search_date - Duration::days(365))
                .format("%Y%m%d")
                .to_string(),
            FIRSTREGDATE: (search_date - Duration::days(365))
                .format("%Y%m%d")
                .to_string(),
            PURPOSE: "事業用".to_string(),
            BODYTYPE: get_random_body_type(),
            LOADAGE: LoadAgeType {
                VALUE_1: "4000".to_string(),
                VALUE_2: "3000".to_string(),
            },
            WEIGHT: WeightType {
                VALUE: "2250".to_string(),
            },
            GROSSWEIGHT: GrossWeightType {
                VALUE_1: "6800".to_string(),
                VALUE_2: "5980".to_string(),
            },
            EXPIRYDATE: (search_date + Duration::days(365 * 3))
                .format("%Y%m%d")
                .to_string(),
            CARID: Some(body.chassisno.clone()),
            ELECTRO_CARINS: "1".to_string(),
        }),
        ERRINFO: None,
    };
    Xml(response)
}
