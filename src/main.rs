slint::include_modules!();
use serde_json::{self, Value};
use slint::SharedString;
mod translate;
fn main() {
    // let (sender,listener) = tokio::sync::mpsc::unbounded_channel::<String>();
    let ui = AppWindow::new().unwrap();
    let temp = ui.as_weak();
    ui.on_Translate(move || {
        let ui = temp.unwrap();
        let input = ui.get_input().to_string();
        let origin = ui.invoke_GetFrom().to_string();
        let translateto = ui.invoke_Translatefrom().to_string();
        let output = translate::generate_url(vec![input], origin, translateto);
        let translate = {
            if let Some(item) = reqwest::blocking::get(output)
                .and_then(|resp| resp.text())
                .map(|body| serde_json::from_str::<Vec<Value>>(&body))
                .unwrap_or_else(|_| Ok(vec![]))
                .unwrap_or_else(|_| vec![])
                .first()
            {
                item.as_array()
                    .unwrap()
                    .iter()
                    .map(|s| s[0].as_str().unwrap())
                    .collect::<Vec<&str>>()
                    .join(" ")
            } else {
                "Error".to_string()
            }

        };
        ui.invoke_Update(SharedString::from(translate));
    });
    ui.run().unwrap();
}
