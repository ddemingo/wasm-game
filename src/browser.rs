use anyhow::{anyhow, Result};
use wasm_bindgen::{closure::WasmClosureFnOnce, prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlImageElement, Response, Window,
};

macro_rules! log {
    ( $( $t:tt )* ) => {
    web_sys::console::log_1(&format!( $( $t )*
    ).into());
    }
   }

// https://h3manth.com/content/html5-canvas-full-screen-and-full-page
pub fn canvas(element_id: &str) -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id(element_id)
        .ok_or_else(|| anyhow!("No Canvas Element found with ID 'canvas'"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|element| {
            anyhow!(
                "Error converting {:#?}
    to HtmlCanvasElement",
                element
            )
        })
}

pub fn closure_once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
where
    F: 'static + WasmClosureFnOnce<A, R>,
{
    Closure::once(fn_once)
}

pub fn context(element_id: &str) -> Result<CanvasRenderingContext2d> {
    canvas(element_id)?
        .get_context("2d")
        .map_err(|js_value| {
            anyhow!(
                "Error getting 2d 
    context {:#?}",
                js_value
            )
        })?
        .ok_or_else(|| anyhow!("No 2d context found"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|element| {
            anyhow!(
                "Error converting {:#?} to
    CanvasRenderingContext2d",
                element
            )
        })
}

pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or_else(|| anyhow!("No Document Found"))
}

pub async fn fetch_with_str(resource: &str) -> Result<JsValue> {
    JsFuture::from(window()?.fetch_with_str(resource))
        .await
        .map_err(|err| anyhow!("error fetching {:#?}", err))
}

pub async fn fetch_json(json_path: &str) -> Result<JsValue> {
    let resp_value = fetch_with_str(json_path).await?;
    let resp: Response = resp_value.dyn_into().map_err(|element| {
        anyhow!(
            "Error converting {:#?}
 to Response",
            element
        )
    })?;
    JsFuture::from(resp.json().map_err(|err| {
        anyhow!(
            "Could not get JSON from
 response {:#?}",
            err
        )
    })?)
    .await
    .map_err(|err| anyhow!("error fetching JSON {:#?}", err))
}

pub fn new_image() -> Result<HtmlImageElement> {
    HtmlImageElement::new().map_err(|err| anyhow!("Could not create HtmlImageElement: {:#?}", err))
}

// The Function type is a pure JavaScript type and is only available in the js-sys package. While we could
// import that, I'd rather not add another crate dependency if possible; however, we don't actually have to
// use the Function type directly
//
// Instead of taking &Function, our request_animation_frame will take &Closure<dyn FnMut(f64)> as its
// parameter. Then, it will call callback.as_ref().unchecked_ref() when calling the web-sys version of
//  request_animation_frame.
pub type LoopClosure = Closure<dyn FnMut(f64)>;

pub fn request_animation_frame(callback: &LoopClosure) -> Result<i32> {
    window()?
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .map_err(|err| anyhow!("Cannot request animation frame {:#?}", err))
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Window found"))
}
