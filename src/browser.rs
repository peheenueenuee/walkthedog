use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, Window, CanvasRenderingContext2d};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No window found"))
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or_else(|| anyhow!("No Document found"))
}

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("No canvas element found with Id 'canvas'"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
}

pub fn context() -> Result<CanvasRenderingContext2d>  {
    canvas()?
        .get_context("2d")
        .map_err(|js_value| anyhow!("Error getting 2d context {:#?}", js_value))?
        .ok_or_else(|| anyhow!("No 2d context"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(
            |element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element)
            )
}
