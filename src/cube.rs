
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation, WebGlVertexArrayObject};
use webgl_matrix::{Matrix, Mat4, Vec4};

use std::{cell::RefCell, f32::consts};

pub struct Cube 
{
    VAO: WebGlVertexArrayObject,
    // VBO: WebGlBuffer,
    // IBO: WebGlBuffer,

    position: Vec4, 
    rotationMatrix: Mat4, 
    numberOfVertices: i32,
}

impl Cube {

    pub fn new(gl: &WebGl2RenderingContext, shader_program: &WebGlProgram, position: Vec4, sideColourIndices: [u32; 6]) -> Self
    {
        //Create VAO for cube
        let vao = gl.create_vertex_array().unwrap();

        //Bind VAO to current webGL context
        gl.bind_vertex_array(Some(&vao));

        //Get coordinate, colour and texture arrays
        let coordinates = Self::get_coordinates();
        //let colours = Self::get_colours();                        
        let textureCoords = Self::get_texture_coords(sideColourIndices);

        // Note that `Float32Array::view` is somewhat dangerous (hence the `unsafe`!). This is creating a raw view into our
        // module's `WebAssembly.Memory` buffer, but if we allocate more pages for ourself (aka do a memory allocation in Rust) 
        // it'll cause the buffer to change, causing the `Float32Array` to be invalid. As a result, 
        // after `Float32Array::view` we have to be very careful not to do any memory allocations before it's dropped.
        
        // COORDINATES
        let coordinates_array = unsafe { js_sys::Float32Array::view(&coordinates) };
        let coordinate_buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&coordinate_buffer));
        gl.buffer_data_with_array_buffer_view( WebGl2RenderingContext::ARRAY_BUFFER, &coordinates_array, WebGl2RenderingContext::STATIC_DRAW);
    
        let coordinates_location = gl.get_attrib_location(&shader_program, "a_coords") as u32;
        gl.vertex_attrib_pointer_with_i32( coordinates_location, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(coordinates_location);

        // TEXTURE COORDINATES
        let textCoords_array = unsafe { js_sys::Float32Array::view(&textureCoords) };
        let textCoords_buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&textCoords_buffer));
        gl.buffer_data_with_array_buffer_view( WebGl2RenderingContext::ARRAY_BUFFER, &textCoords_array, WebGl2RenderingContext::STATIC_DRAW);
   
        let textCoords_location = gl.get_attrib_location(&shader_program, "a_textCoord") as u32;
        gl.vertex_attrib_pointer_with_i32( textCoords_location, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(textCoords_location);

        Self {
            VAO: vao,
            position: position,
            rotationMatrix: Mat4::identity(),
            numberOfVertices: 108 / 3, // == 36, since 6 vertices per side & 6 sides
        }

    }

    // Method to swap the pointers of cubies rotation matrices (since cubies are rotated in place and not translated)
    pub fn swapCubesRotationMatrices(&mut self, other: &mut Cube)
    {
        use std::mem;

        //Swap pointers of 2 cubies rotation matrices
        mem::swap(&mut self.rotationMatrix, &mut other.rotationMatrix); 
    }

    // Method to rotate a cubies matrix along a given axis //Note: angle is always PI/2 since this is final step
    pub fn setCubeRotationMatrix(&mut self, axis: &[f32; 3])
    {
        // Multiply axis by matrix to get the axis along which we want to rotate
        self.rotationMatrix.rotate(consts::PI / 2.0, &self.rotationMatrix.mul_vector(axis) );
    }

    //Function for object to render itself
    pub fn draw(&self, 
            gl_refcell: &RefCell<WebGl2RenderingContext>,
            shader_program: &WebGlProgram,
            model_view_matrix: &mut [f32; 16], 
            model_view_matrix_location: &WebGlUniformLocation,
        )
        {//NOTE: webGL matrices are column major, printed as row major

            //Set position for cubie within rubix cube
            model_view_matrix.translate(&self.position);

            //Clone to avoid mutating rotation matrix when matrix multiplying later (which modifies left param)
            let mut klone = self.rotationMatrix.clone();

            //Deref ptr to matrix multiplication and put into model matrix ptr
            *model_view_matrix = *klone.mul(&model_view_matrix); 

            //Convert model matrix to vector
            let vec_model_view_matrix = model_view_matrix.iter().map(|v| *v).collect::<Vec<_>>();
        
            //Bind shader to current webGL context
            gl_refcell.borrow().use_program(Some(&shader_program));

            //Set model_view matrix uniform for currently bound shader program
            gl_refcell.borrow().uniform_matrix4fv_with_f32_array(Some(model_view_matrix_location),false,&vec_model_view_matrix);
        
            //Bind this cube's VAO to current webGL context
            gl_refcell.borrow().bind_vertex_array(Some(&self.VAO));

            //Make draw call finally                                                        
            gl_refcell.borrow().draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, self.numberOfVertices); 
        
    }

    fn get_coordinates() -> [f32; 108]
    {
        let coordinates = [  //back, front, left, right, bottom, top
            -0.5, -0.5, -0.5,        
             0.5, -0.5, -0.5,        
             0.5,  0.5, -0.5,        
             0.5,  0.5, -0.5,        
            -0.5,  0.5, -0.5,        
            -0.5, -0.5, -0.5,        

            -0.5, -0.5,  0.5,        
             0.5, -0.5,  0.5,        
             0.5,  0.5,  0.5,        
             0.5,  0.5,  0.5,        
            -0.5,  0.5,  0.5,        
            -0.5, -0.5,  0.5,        

            -0.5,  0.5,  0.5,        
            -0.5,  0.5, -0.5,        
            -0.5, -0.5, -0.5,        
            -0.5, -0.5, -0.5,        
            -0.5, -0.5,  0.5,        
            -0.5,  0.5,  0.5,        

             0.5,  0.5,  0.5,        
             0.5,  0.5, -0.5,        
             0.5, -0.5, -0.5,        
             0.5, -0.5, -0.5,        
             0.5, -0.5,  0.5,        
             0.5,  0.5,  0.5,        

            -0.5, -0.5, -0.5,        
             0.5, -0.5, -0.5,        
             0.5, -0.5,  0.5,        
             0.5, -0.5,  0.5,        
            -0.5, -0.5,  0.5,        
            -0.5, -0.5, -0.5,        

            -0.5,  0.5, -0.5,        
             0.5,  0.5, -0.5,        
             0.5,  0.5,  0.5,        
             0.5,  0.5,  0.5,        
            -0.5,  0.5,  0.5,        
            -0.5,  0.5, -0.5,        
        ];
    
        coordinates
    }
    

    fn get_texture_coords(indices: [u32; 6]) -> [f32; 72] // indices for sprite sheet
    {
        let mut textCoords = [
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            
            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0,
            1.0, 0.0,
            0.0, 0.0,
            0.0, 1.0,
            
            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0,
            1.0, 0.0,
            0.0, 0.0,
            0.0, 1.0,
        ];

        let cols = indices.len();

        //Loop to adjust texture coordinate array to correctly access sprite sheet
        for (i, coord) in textCoords.into_iter().enumerate() {
            
            //vec2((v_textCoord.x + spriteIndex.x) * SPRITE_COLS, (v_textCoord.y + spriteIndex.y) * SPRITE_ROWS)
            if i % 2 == 0 {
                textCoords[i] = (coord + (indices[(i/(2*cols)) % cols] as f32)) * (1.0 / cols as f32);
            }else{
                textCoords[i] = coord as f32;
            } 
        }
        
        textCoords
    }

}

