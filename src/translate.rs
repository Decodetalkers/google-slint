pub fn generate_url(v: Vec<String>, source: String, target: String) -> String {
    let base_url = "https://translate.googleapis.com/translate_a/single";
    let q = v.join(" ");
    let url = "?client=gtx&ie=UTF-8&oe=UTF-8&dt=t";
    let source = format!("{}{}", "&sl=", source);
    let target = format!("{}{}", "&tl=", target);
    let input = format!("&q={}", q);
    format!("{}{}{}{}{}", base_url, url, source, target, input)
}
