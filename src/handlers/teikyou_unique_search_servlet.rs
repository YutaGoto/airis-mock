use axum::Form;
use chrono::{DateTime, Duration, FixedOffset};
use rand::Rng;

use crate::models::airis::{
    AirisCommon, AirisData, AirisResponse, GrossWeightType, LoadAgeType,
    TeikyouUniqueSearchServlet, WeightType,
};
use crate::models::body_type::get_body_type;
use crate::models::error_type::{get_error_info, is_error};
use crate::models::xml::Xml;

pub async fn teikyou_unique_search_servlet(
    body: Form<TeikyouUniqueSearchServlet>,
) -> Xml<AirisResponse> {
    let search_date: DateTime<FixedOffset> = DateTime::parse_from_str(
        format!("{} 00:00:00 +0900", body.searchdate).as_str(),
        "%Y%m%d %H:%M:%S %z",
    )
    .unwrap();

    if is_error(body.chassisno.clone()) {
        let errinfo = get_error_info(body.chassisno.clone());
        println!("errinfo: {:?}", errinfo);

        let response = AirisResponse {
            common: AirisCommon {
                orgname: "test".to_string(),
                version: "1.0.0".to_string(),
                searchdate: search_date.format("%Y%m%d").to_string(),
                searchid: body.searchid.clone(),
                seqno: body.seqno.clone(),
                result: "1".to_string(),
                num: "0".to_string(),
            },
            data: None,
            errinfo: Some(errinfo),
        };
        Xml(response)
    } else {
        let response = AirisResponse {
            common: AirisCommon {
                orgname: "test".to_string(),
                version: "1.0.0".to_string(),
                searchdate: search_date.format("%Y%m%d").to_string(),
                searchid: body.searchid.clone(),
                seqno: body.seqno.clone(),
                result: "0".to_string(),
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
                    value_1: rand::rng().random_range(10000..30000),
                    value_2: rand::rng().random_range(10000..30000),
                },
                weight: WeightType {
                    value: rand::rng().random_range(10000..30000),
                },
                grossweight: GrossWeightType {
                    value_1: rand::rng().random_range(10000..30000),
                    value_2: rand::rng().random_range(10000..30000),
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
}
