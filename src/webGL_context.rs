
//https://github.com/cx20/webgl-test/blob/master/examples/rust/cube/src/lib.rs

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::WebGl2RenderingContext;
/*
    Function to set up webGL Context
*/
pub fn init_webgl_context(canvas_id: &str) -> Result<WebGl2RenderingContext, JsValue> {
    
    //Get a reference to the current window's document
    let document = web_sys::window().unwrap().document().unwrap();

    //Get a reference to the current window document's canvas
    let canvas = document.get_element_by_id(canvas_id).unwrap();

    //Dynamically cast element into rust canvas element
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    //Call to get reference to webGL context from canvas element
    let gl: WebGl2RenderingContext = canvas
        .get_context("webgl2")?              //HARD CODED STRING HERE <<<< WHY????
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    //Set webGL viewports size (Note: canvas width is hardcoded in canvas html element in index.html)
    gl.viewport(0,0,canvas.width().try_into().unwrap(),canvas.height().try_into().unwrap());

    Ok(gl)
}