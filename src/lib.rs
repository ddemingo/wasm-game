use futures::prelude::*;
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

    //let document = browser::document().expect("No Document found");

    wasm_bindgen_futures::spawn_local(async move {
        let context = browser::context("canvas").expect("");

        let (tx, rx) = futures::channel::oneshot::channel::<()>();

        let image = web_sys::HtmlImageElement::new().unwrap();

        let callback = Closure::once(move || {
            tx.send(()).unwrap();
        });
        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
        image.set_src("Stone.png");
        rx.await.unwrap();

        context
            .draw_image_with_html_image_element(&image, 0.0, 0.0)
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
