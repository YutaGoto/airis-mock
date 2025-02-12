use axum::{
    response::Response,
    routing::{get, post},
    Form, Router,
};
use chrono::{DateTime, Duration, FixedOffset};

mod models;

use crate::models::airis::{
    AirisCommon, AirisData, AirisResponse, GrossWeightType, LoadAgeType,
    TeikyouUniqueSearchServlet, WeightType,
};
use crate::models::body_type::{get_body_type, get_random_body_type, BodyType};
use crate::models::shape_code::{read_shape_codes, ShapeCodeList};
use crate::models::xml::Xml;

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

    // if debug mode, print the server is running on http://localhost:4567
    #[cfg(debug_assertions)]
    println!("Server is running on http://localhost:4567");

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Xml<BodyType> {
    Xml(get_random_body_type())
}

async fn get_body_types() -> Xml<ShapeCodeList> {
    Xml(ShapeCodeList {
        shape_codes: read_shape_codes(),
    })
}

async fn health_check() -> Response {
    Response::new("OK".into())
}

async fn check_teikyou_unique_search_servlet(
    body: Form<TeikyouUniqueSearchServlet>,
) -> Xml<TeikyouUniqueSearchServlet> {
    Xml(TeikyouUniqueSearchServlet {
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

async fn teikyou_unique_search_servlet(
    body: Form<TeikyouUniqueSearchServlet>,
) -> Xml<AirisResponse> {
    let search_date: DateTime<FixedOffset> = DateTime::parse_from_str(
        format!("{} 00:00:00 +0900", body.searchdate).as_str(),
        "%Y%m%d %H:%M:%S %z",
    )
    .unwrap();

    let response = AirisResponse {
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
            bodytype: get_body_type(body.chassisno.clone()),
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
    };
    Xml(response)
}
