use serde::{Deserialize, Serialize};

use crate::models::body_type::BodyType;

use super::error_type::AirisErrInfo;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "RESPONSE", rename_all = "UPPERCASE")]
pub struct AirisResponse {
    pub common: AirisCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AirisData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errinfo: Option<AirisErrInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "COMMON", rename_all = "UPPERCASE")]
pub struct AirisCommon {
    pub orgname: String,
    pub version: String,
    pub searchdate: String,
    pub searchid: String,
    pub seqno: String,
    pub result: String,
    pub num: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "DATA", rename_all = "UPPERCASE")]
pub struct AirisData {
    pub regdate: String,
    pub firstregdate: String,
    pub purpose: String,
    pub bodytype: BodyType,
    pub loadage: LoadAgeType,
    pub weight: WeightType,
    pub grossweight: GrossWeightType,
    pub expirydate: String,
    pub carid: Option<String>,
    pub electro_carins: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "LOADAGE", rename_all = "UPPERCASE")]
pub struct LoadAgeType {
    pub value_1: u32,
    pub value_2: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "WEIGHT", rename_all = "UPPERCASE")]
pub struct WeightType {
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "GROSSWEIGHT", rename_all = "UPPERCASE")]
pub struct GrossWeightType {
    pub value_1: u32,
    pub value_2: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeikyouUniqueSearchServlet {
    pub searchdate: String,
    pub searchid: String,
    pub privacyflg: String,
    pub seqno: String,
    pub retryflg: String,
    pub groupid: String,
    pub regno: String,
    pub chassisno: String,
    pub version: String,
    pub userid: String,
    pub pw: String,
    pub keyword: String,
}
