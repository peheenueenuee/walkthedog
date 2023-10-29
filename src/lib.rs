#[macro_use]
mod browser;
mod engine;

use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}
#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}
#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    log!("Hello world! in browser");
    let context = browser::context().expect("Could not get browser context");

    browser::spawn_local(async move{
        let sheet: Sheet = browser::fetch_json("rhb.json")
            .await
            .expect("could not fetch rhb.json")
            .into_serde()
            .expect("could not convert rhb.json into a Sheet structure");

        let image = engine::load_image("rhb.png")
            .await
            .expect("Could not create browser image ");

        let mut frame = -1;
        let interval_callback = Closure::wrap(Box::new(move || {
            frame = (frame + 1) % 8;
            let frame_name = format!("Run ({}).png", frame + 1);
            let sprite = sheet.frames.get(&frame_name).expect("cell not found");
            context.clear_rect(0.0, 0.0, 600.0, 600.0);
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                sprite.frame.x.into(),
                sprite.frame.y.into(),
                sprite.frame.w.into(),
                sprite.frame.h.into(),
                300.0,
                300.0,
                sprite.frame.w.into(),
                sprite.frame.h.into(),
                );
        }) as Box<dyn FnMut()>);
        browser::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                interval_callback.as_ref().unchecked_ref(),
                50);
        interval_callback.forget();
    });

    Ok(())
}
