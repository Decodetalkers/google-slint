slint::include_modules!();
use slint::SharedString;
mod translate;

fn main() {
    let ui = AppWindow::new().unwrap();
    let temp = ui.as_weak();

    ui.on_Translate(move || {
        let temp2 = temp.clone();
        let ui = temp.unwrap();
        let input = ui.get_input();
        let origin = ui.invoke_GetFrom();
        let target = ui.invoke_Translatefrom();
        let url = translate::generate_url(&input, &origin, &target);
        std::thread::spawn(move || {
            let translate = translate::get_translate(url).unwrap_or("Error".to_owned());
            slint::invoke_from_event_loop(move || {
                temp2.unwrap().invoke_Update(SharedString::from(translate))
            })
            .ok();
        });
    });

    ui.run().unwrap();
}
