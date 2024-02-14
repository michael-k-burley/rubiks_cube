
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, HtmlImageElement};

/*
    Function that loads a texture from an html img tag on the client side
*/
pub fn load_texture_from_htmlImg(gl: &WebGl2RenderingContext, htmlImg: &HtmlImageElement, textureNumber: u32) -> Result<(), JsValue>

{
    //Get uint id for currently bound shader
    let textureID = gl.create_texture().unwrap(); //.expect("Cannot create gl texture");

    //Set as active new texture id
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&textureID));

    //Set this to true to flip texture image if upsidedown (Possibly Depreciated)
    gl.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, true as i32);

    //Load texture 0 + i for currently bound shader program
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + textureNumber); 

    let target = WebGl2RenderingContext::TEXTURE_2D;
    let level = 0; 
    let internalformat = WebGl2RenderingContext::RGBA as i32; //How WebGL stores data   
    let format = WebGl2RenderingContext::RGBA;                //Format of the data we are giving to WebGL 
    let type_ = WebGl2RenderingContext::UNSIGNED_BYTE; 

    //Attach texture image (ie. localbuffer) to currently bound texture object (on GPU)
    gl.tex_image_2d_with_u32_and_u32_and_html_image_element(target, level, internalformat, format, type_, htmlImg)?;
    //Probably works //gl.tex_image_2d_with_u32_and_u32_and_html_canvas_element(target, level, internalformat, format, type_, source);
    
    //Set the texture wrapping/filtering options (on the currently bound texture object)
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT as i32);

    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);

    //Generate mipmaps for texture (ie. makes smaller versions of itself for rendering distant versions of itself)
    gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

    Ok(())
}
