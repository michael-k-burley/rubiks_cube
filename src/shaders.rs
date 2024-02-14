
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

/*
    Helper function (called by setup_shaders (below)) to compile shaders for current webGL context
*/
fn create_shader(gl: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, JsValue> {
    
    //Create a shader of the given type for the currently webGL context
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| JsValue::from_str("Unable to create shader object"))?;

    //Transfer the shader source code to the currently bound shader
    gl.shader_source(&shader, source);

    //Compile the currently bound shader
    gl.compile_shader(&shader);

    //If shader compiled successfuly then return the shader else JSValue error
    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        crate::log(&format!("ERROR: Unable to Compile Shader")); 

        Err(JsValue::from_str(&gl.get_shader_info_log(&shader).unwrap_or_else(|| "Unknown error creating shader".into()) ))
    }
}

/*
    Function to create adn link shaders. Returns a working shader program or else err
*/
pub fn setup_shaders(gl: &WebGl2RenderingContext, vertex_shader_str: &str, fragment_shader_str: &str) -> Result<WebGlProgram, JsValue> {
    
    //Compile vertex shader via helper function above
    let vertex_shader = create_shader(&gl, WebGl2RenderingContext::VERTEX_SHADER, vertex_shader_str).unwrap();
    
    //Compile vertex shader via helper function above
    let fragment_shader = create_shader(&gl,WebGl2RenderingContext::FRAGMENT_SHADER, fragment_shader_str).unwrap();

    //Create webGL shader program
    let shader_program = gl.create_program().unwrap();

    //Attach vertex and fragment shader to webGL shader program
    gl.attach_shader(&shader_program, &vertex_shader);
    gl.attach_shader(&shader_program, &fragment_shader);

    //Link the shader program to the currently webGL context
    gl.link_program(&shader_program);

    //If shader program successfully linked to currently bounded webGl context then have webGL context use that shader program
    //else return JSValue error
    if gl
        .get_program_parameter(&shader_program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        // Set the shader program as active.
        gl.use_program(Some(&shader_program));
        Ok(shader_program)
        
    } else {
        crate::log(&format!("ERROR: Unable to Link Shader")); 

        return Err(JsValue::from_str(&gl.get_program_info_log(&shader_program)
                                        .unwrap_or_else(|| "Unknown error linking program".into()) ));
    }
}

