use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "ERRINFO", rename_all = "UPPERCASE")]
pub struct AirisErrInfo {
    pub errid: String,
    pub errmsg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorCode {
    pub errid: String,
    pub errmsg: String,
}

fn read_error_codes() -> Vec<ErrorCode> {
    let error_codes = include_str!("../static/error_codes.json");
    let error_codes: Vec<ErrorCode> = serde_json::from_str(error_codes).unwrap();
    error_codes
}

pub fn is_error(chassisno: String) -> bool {
    chassisno.starts_with("ERR")
}

pub fn get_error_info(chassisno: String) -> AirisErrInfo {
    let v = chassisno.split("-").collect::<Vec<&str>>();
    let errid = v[1].to_string();

    let error_codes = read_error_codes();
    let error_code = error_codes
        .iter()
        .find(|error_code| error_code.errid == errid);
    if let Some(error_code) = error_code {
        AirisErrInfo {
            errid: error_code.errid.clone(),
            errmsg: error_code.errmsg.clone(),
        }
    } else {
        AirisErrInfo {
            errid: "2010".to_string(),
            errmsg: "システムエラーが発生しました。".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_error() {
        assert_eq!(is_error("ERR-1001".to_string()), true);
        assert_eq!(is_error("1234567890".to_string()), false);
    }

    #[test]
    fn test_get_error_info() {
        let errinfo = get_error_info("ERR-1001".to_string());
        assert_eq!(errinfo.errid, "1001");
        assert_eq!(errinfo.errmsg, "該当する車両がありません");
    }
}
