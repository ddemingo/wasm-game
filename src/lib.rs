use std::collections::HashMap;

use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[macro_use]
mod browser;
mod sierpinski;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    //#[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let context = browser::context("canvas").expect("");

    wasm_bindgen_futures::spawn_local(async move {
        let json = browser::fetch_json("rhb.json")
            .await
            .expect("Could not fetch JSON");
        let sheet: Sheet = json
            .into_serde()
            .expect("Could not convert rhb.json into a Sheet structure");

        // load image
        let (tx, rx) = futures::channel::oneshot::channel::<()>();
        let image = web_sys::HtmlImageElement::new().unwrap();
        let callback = Closure::once(move || {
            tx.send(()).unwrap();
        });
        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
        image.set_src("rhb.png");
        rx.await.unwrap();

        let sprite = sheet.frames.get("Run (1).png").expect("Cell not found");
        context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                sprite.frame.x.into(),
                sprite.frame.y.into(),
                sprite.frame.w.into(),
                sprite.frame.h.into(),
                300.0,
                300.0,
                sprite.frame.w.into(),
                sprite.frame.h.into(),
            )
            .unwrap();
    });

    let context = browser::context("canvas").expect("");
    sierpinski::draw(
        &context,
        [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)],
        (0, 255, 0),
        5,
    );

    Ok(())
}

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}
#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}
