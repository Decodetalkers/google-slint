use reqwest::IntoUrl;
use serde_json::Value;
use url::Url;

const BASE_URL: &str = "https://translate.googleapis.com/translate_a/single";

#[derive(Debug, thiserror::Error)]
pub enum TranslateError {
    #[error("reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("request Failed")]
    ReqwestFailed,
    #[error("Serde error")]
    SerdeError(#[from] serde_json::Error),
}

pub fn get_translate<T: IntoUrl>(url: T) -> Result<String, TranslateError> {
    let response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        return Err(TranslateError::ReqwestFailed);
    }
    let body = response.text()?;

    let arrays: Vec<Value> = serde_json::from_str(&body)?;

    let first_element = arrays
        .first()
        .ok_or(TranslateError::ReqwestFailed)?
        .as_array()
        .ok_or(TranslateError::ReqwestFailed)?;

    let mut res = Vec::new();

    for data in first_element {
        let Some(translate_v) = data.get(0) else {
            continue;
        };
        let Some(translate) = translate_v.as_str() else {
            continue;
        };
        res.push(translate.to_owned());
    }
    Ok(res.join(" "))
}

pub fn generate_url(v: &str, source: &str, target: &str) -> Url {
    let mut base_url = Url::parse(BASE_URL).unwrap();
    base_url
        .query_pairs_mut()
        .append_pair("client", "gtx")
        .append_pair("ie", "UTF-8")
        .append_pair("oe", "UTF-8")
        .append_pair("dt", "t")
        .append_pair("sl", source)
        .append_pair("tl", target)
        .append_pair("q", v);
    base_url
}
