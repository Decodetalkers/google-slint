use url::Url;

const BASE_URL: &str = "https://translate.googleapis.com/translate_a/single";

pub fn generate_url(v: Vec<String>, source: String, target: String) -> Url {
    let mut base_url = Url::parse(BASE_URL).unwrap();
    let q = v.join(" ");
    base_url
        .query_pairs_mut()
        .append_pair("client", "gtx")
        .append_pair("ie", "UTF-8")
        .append_pair("oe", "UTF-8")
        .append_pair("dt", "t")
        .append_pair("sl", &source)
        .append_pair("tl", &target)
        .append_pair("q", &q);
    base_url
}
