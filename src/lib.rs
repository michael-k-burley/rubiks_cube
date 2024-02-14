
#![allow(non_snake_case)]

mod webGL_context;
mod shaders;
mod rubix;
mod cube;   
mod texture; 

use std::rc::Rc;          //Reference counter ie. smart pointer
use std::cell::RefCell;   //Reference cell ie. shared mutable memory

use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram};
use webgl_matrix::{ProjectionMatrix, Mat4};

extern crate js_sys;

//JS imported functions for trouble shooting
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)] 
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

//Rust wrappers for JS side functions
fn get_current_time() -> f64 { 
    js_sys::Date::now() / 1000.0    //Get time in seconds
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// Create struct to keep track of delta time for each frame
struct DeltaTime {
    past: f64,
    current: f64,
}
   
#[wasm_bindgen]
pub fn rubix_cube_simulation() -> Result<(), JsValue> {
    
    //Get webGL context from Canvas element
    let gl = webGL_context::init_webgl_context("canvasID").unwrap();

    //Get shader files as a string
    let vertex_shader_str = include_str!("../assets/shaders/shad.vs");
    let fragment_shader_str = include_str!("../assets/shaders/shad.fs");

    //Assign shaders to current webGL context
    let shader_program: WebGlProgram = shaders::setup_shaders(&gl, vertex_shader_str, fragment_shader_str).unwrap();

    //Alternative to above is to get image from index.html img tag or canvas tag
    let document = web_sys::window().unwrap().document().unwrap();
    let imageEle = document.get_element_by_id("imageID").unwrap();
    let imageHtml = imageEle.dyn_into::<web_sys::HtmlImageElement>().unwrap();
    
    //Load image for texture atlas for cube
    if let Err(e) = texture::load_texture_from_htmlImg(&gl, &imageHtml, 0){ 
        log(&format!("Error: Main:: Cannot load texture(s)."));
        return Err(e);
    }

    //Create rubixs cube, then get ref cell wrapped smart pointer
    let rubix = rubix::Rubix::new(&gl, &shader_program, [0.0, 0.0, -15.0]);
    let rubix_refcell = Rc::new(RefCell::new(rubix));

    //Add event listener for keyboard input
    if let Err(e) = addKeyboardEventListener(Rc::clone(&rubix_refcell)){
        log(&format!("Error: Main:: Keyboard Event Listener {}", &e.as_string().unwrap()));
    };

    //Add event listener for button input
    if let Err(e) = addButtonEventListener(Rc::clone(&rubix_refcell)) {
        log(&format!("Error: Main:: Button Event Listener {}", &e.as_string().unwrap())); 
    };

    //Get uniform locations for matricies for curretnly bound shader program
    let projection_matrix_location = gl.get_uniform_location(&shader_program, "u_projection").unwrap();
    let model_view_matrix_location = gl.get_uniform_location(&shader_program, "u_model").unwrap();

    //Set projection matrix for currently bound shader program
    let canvas = gl.canvas().unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let field_of_view = 45.0 * std::f32::consts::PI / 180.0; 
    let aspectRatio = canvas.client_width() as f32 / canvas.client_height() as f32;
    let z_near = 0.1;
    let z_far = 100.0;

    //Ok to be done outside of render loop since only 1 shader program
    let projection_matrix = Mat4::create_perspective(field_of_view, aspectRatio, z_near, z_far);
    let vec_projection_matrix = projection_matrix.iter().map(|v| *v).collect::<Vec<_>>();

    //Set shader program uniform(s) for currently bound shader program
    gl.uniform_matrix4fv_with_f32_array(Some(&projection_matrix_location),false,&vec_projection_matrix);

    //Set gl context flags
    gl.enable(WebGl2RenderingContext::DEPTH_TEST);  
    gl.depth_func(WebGl2RenderingContext::LEQUAL);

    //Get mutable reference to gl context
    let gl_refcell = RefCell::new(gl); 

    //Beginning of render loop
    let mut delta = DeltaTime { past : 0.0, current: get_current_time() };
    
    //Create rc to refcell of closure which is the render loop
    let f = Rc::new( RefCell::new( None ) );
    let g = Rc::clone(&f);
    
    *g.borrow_mut() = Some( Closure::wrap( Box::new( move || {  // Closure struct comes from wasm-bindgen crate
                                                                // converts rust closure to javascript closure
        //Clear frame to clear colour
        gl_refcell.borrow().clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        
        //Update delta time
        delta.past = delta.current;
        delta.current = get_current_time();
        let delta = delta.current - delta.past;
        
        //Render rubik's cube
        rubix_refcell.borrow_mut().draw(&gl_refcell, &shader_program, &model_view_matrix_location, delta);

        //Call self again so as to perpetually loop 
        request_animation_frame(f.borrow().as_ref().unwrap());

    }) as Box<dyn FnMut()>));

    //Make first animation call
    request_animation_frame(g.borrow().as_ref().unwrap());
    
    Ok(())
}


/*
 * Function to add event listener for keypresses
 */
fn addKeyboardEventListener(rubix_refcell: Rc<RefCell<rubix::Rubix>>) -> Result<(), JsValue>
{

    let document = web_sys::window().unwrap().document().unwrap();
                
    let callback = Closure::<dyn FnMut(_)>::new( move |event: web_sys::KeyboardEvent| {

        event.prevent_default(); 

        match event.code().as_str() {
            
            "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown" => rubix_refcell.borrow_mut().rotateCube( event.code() ),
            "KeyR" | "KeyO" | "KeyY" | "KeyG" | "KeyB" | "KeyW"  => rubix_refcell.borrow_mut().rotateFace( event.code() ),
            _ => (), //Needed to satisfy non-exhaustive pattern complaint
        }
    });

    //Add event listener for keyboard input
    document.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())?;
    
    //This leaks memory in rust. Use sparingly
    callback.forget(); 

    Ok(())

}

/*
 * Function to add event listener to button to change direction of rotation of rubik's cube faces
 */
fn addButtonEventListener(rubix_refcell: Rc<RefCell<rubix::Rubix>>) -> Result<(), JsValue>
{
    use rubix::RotationDirection;

    let document = web_sys::window().unwrap().document().unwrap();
                
    let callback = Closure::<dyn FnMut(_)>::new( move |event: web_sys::InputEvent| {

        event.prevent_default(); 

        let document = event.current_target().unwrap().dyn_into::<web_sys::HtmlDocument>().unwrap();
        let button = document.get_element_by_id("buttonID").unwrap().dyn_into::<web_sys::HtmlButtonElement>().unwrap();

        match rubix_refcell.borrow_mut().changeRotationDirection() {
            
            RotationDirection::Clockwise        => button.set_inner_text("Clockwise"),
            RotationDirection::CounterClockwise => button.set_inner_text("Counter-Clockwise"),
        }
    });

    //Add event listener for button press
    document.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())?;
    
    //This leaks memory in rust. Use sparingly
    callback.forget(); 

    Ok(())

}
