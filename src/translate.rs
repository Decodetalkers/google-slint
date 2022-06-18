pub fn generate_url(v: Vec<String>, source: String, target: String) -> String {
    let base_url = "https://translate.googleapis.com/translate_a/single";
    let q = v.join(" ");
    format!(
        "{}{}{}{}{}",
        base_url,
        "?client=gtx&ie=UTF-8&oe=UTF-8&dt=t",
        format!("{}{}", "&sl=", source),
        format!("{}{}", "&tl=", target),
        format!("&q={}", q).to_string()
    )
}
