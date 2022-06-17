use anyhow::{anyhow, Result};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

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

/*
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
}*/

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Window found"))
}
