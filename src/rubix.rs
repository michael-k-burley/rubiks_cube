
use crate::cube::Cube;
use std::{cell::RefCell, f32::consts, array};   

use web_sys::{WebGl2RenderingContext, WebGlProgram,  WebGlUniformLocation};
use webgl_matrix::{Matrix, Mat4, Vec3};

const CUBE_ROTATION_SPEED: f32 = 0.15; // Arbitrary
const FACE_ROTATION_SPEED: f32 = 1.5; // If set to 1.5 then will rotate faces 1.5 times as fast as the delta

pub struct Rubix 
{
    cubePosition: Vec3,    
    cubeRotation: [f32; 3], //Maybe try making this quaternions
    cubes: Vec<Cube>, 
    rotatingFace: Option<RotationFace>,
    rotationDirection: RotationDirection,
}

#[derive(Clone, Copy)] //Clone needed for Copy, Copy needed for rotateFace method
pub enum RotationDirection {
    Clockwise = 1,
    CounterClockwise = -1,
}

impl Rubix {

    pub fn new(gl: &WebGl2RenderingContext, shader_program: &WebGlProgram, position: Vec3) -> Self
    {
        let mut cubes = Vec::new();

        //Create rubix cube
        for i in (0..27).into_iter() { //Magik const alert. 27 is number of cubes

            //Cubie position within rubix cube // -1 since cubies are at origin and we want rubik's center to be at origin
            let cube_pos = [(i%3) -1, ((i/3)%3) -1, (i/9) -1, 1].map(|x| x as f32);  
            
            //Set colour of each cubes side (back, front, left, right, bottom, top)
            let sideColour = [0,1,3,4,5,2]; 
            // Red 0, Orange 1, Yellow 2, Green 3, Blue 4, White 5

            //Create cube and put into rubix cube array
            cubes.push( Cube::new(gl, shader_program, cube_pos, sideColour));
        }

        Self {
            cubePosition: position,
            cubeRotation: [0.0, 0.0, 0.0], 
            cubes: cubes,
            rotatingFace: None,
            rotationDirection: RotationDirection::Clockwise, 
        }
    }

    //Function to rotate cubes on faces of rubik's cube
    fn rotate(&mut self, angle: f32) 
    {
        //Function is only called if RotationFace is not none
        let face = self.rotatingFace.as_mut().unwrap(); 

        //If animation hasn't gone 90 degrees, rotate by angle
        if face.lt_half_pi() {
            face.rotate(angle);

        }else{ //Else animation has gone 90 degrees, then stop rotating a side and instead flip the relevant cubies

            let qb_ids = face.get_cubie_ids();

            //Same for cw && ccw, since to rotate face 90 degrees, can first swap top & btm row and then across diagonal
            for i in 0..3 {        
                let (qb1, qb2) = self.cubes.split_at_mut(qb_ids[i + 6]); //9 cubes per face
                qb1[ qb_ids[i] ].swapCubesRotationMatrices( &mut qb2[0] );                // +6 offset from top to btm row
            }

            //Closure to rotate relevant cubies to complete rotation
            let mut rotate = |direction| { 

                //Pairs of indices to be swapped accross diagonal of face  //Clockwise  //Counter-Clockwise
                let ranges = if direction { [(1,3),(2,6),(5,7)] } else { [(0,8),(1,5),(3,7)] };

                //Swap 3 pairs of cubies across the diagonal of the rotating face
                for (i,j) in ranges {
                    let (qb1, qb2) = self.cubes.split_at_mut(qb_ids[j]);
                    qb1[ qb_ids[i] ].swapCubesRotationMatrices( &mut qb2[0] );
                };
            };
            
            //Get rotation direction from cube
            let dir = match self.rotationDirection {
                RotationDirection::Clockwise => true,
                RotationDirection::CounterClockwise => false,
            };

            //Find axis to rotate cubes around //Note: negation of alternating faces required to properly rotate
            let axis = match face {          
                RotationFace::Front(_, axis)  => { rotate( dir); axis }, //R
                RotationFace::Back(_, axis)   => { rotate(!dir); axis }, //O
                RotationFace::Left(_, axis)   => { rotate(!dir); axis }, //G
                RotationFace::Right(_, axis)  => { rotate( dir); axis }, //B
                RotationFace::Up(_, axis)     => { rotate( dir); axis }, //Y
                RotationFace::Down(_, axis)   => { rotate(!dir); axis }, //W
            };

            //Rotate cubies for the relevant face of the cube
            for i in qb_ids {
                self.cubes[i].setCubeRotationMatrix(axis)
            }

            //Set to None after rotation is finished
            self.rotatingFace = None;          
        }

    }


    // Method for Rubik's cube to render itself
    pub fn draw(&mut self, 
        gl_refcell: &RefCell<WebGl2RenderingContext>,
        shader_program: &WebGlProgram,
        model_view_matrix_location: &WebGlUniformLocation,
        delta: f64,
    )
    {
        //If a side of the cube is currently rotating then will match
        if self.rotatingFace.is_some() { 
            self.rotate(FACE_ROTATION_SPEED * delta as f32);
        }
     
        //Loop through every cube in rubix and call draw on it
        for (i, cube) in self.cubes.iter_mut().enumerate() {

            //Create model matrix to position element in world space
            let mut model_view_matrix = Mat4::identity();

            //Move each object to a common position in the world (ie. Shift from origin to worldspace)
            model_view_matrix.translate(&self.cubePosition);  //NOTE: (This operation will be applied last);

            //Rotate entire cube
            model_view_matrix.rotate( self.cubeRotation[0], &vec![0.0, 1.0, 0.0]);
            model_view_matrix.rotate( self.cubeRotation[1], &vec![1.0, 0.0, 0.0]);

            //Animation for rotating a face of the cube if one has been set in motion
            if let Some(face) = self.rotatingFace.as_ref() { 

                match face {
                    RotationFace::Front(angle, axis)  if (0..9).contains(&i)    =>  { model_view_matrix.rotate(*angle, axis); }, 
                    RotationFace::Back(angle, axis)   if (18..27).contains(&i)  =>  { model_view_matrix.rotate(*angle, axis); }, 
                    RotationFace::Left(angle, axis)   if i % 3 == 2                   =>  { model_view_matrix.rotate(*angle, axis); }, 
                    RotationFace::Right(angle, axis)  if i % 3 == 0                   =>  { model_view_matrix.rotate(*angle, axis); }, 
                    RotationFace::Up(angle, axis)     if (6..9).contains(&(i%9)) => { model_view_matrix.rotate(*angle, axis); }, 
                    RotationFace::Down(angle, axis)   if (0..3).contains(&(i%9)) => { model_view_matrix.rotate(*angle, axis); }, 
                    _ => (), //Needed for exhaustive pattern match
                }
            }

            //Make draw call on individual cubie
            cube.draw(gl_refcell, shader_program, &mut model_view_matrix, model_view_matrix_location);
        }

    }

    // Method to modify angle of rotation for entire cube 
    pub fn rotateCube(&mut self, keyCode: String){
        
        match keyCode.as_str() {

            "ArrowLeft"  =>  self.cubeRotation[0] -= CUBE_ROTATION_SPEED,
            "ArrowRight" =>  self.cubeRotation[0] += CUBE_ROTATION_SPEED,
            "ArrowUp"    =>  self.cubeRotation[1] -= CUBE_ROTATION_SPEED,
            "ArrowDown"  =>  self.cubeRotation[1] += CUBE_ROTATION_SPEED,
            _ => (), //Needed to satisfy non-exhaustive pattern complaint
        }
    }

    // Method to set in motion the rotation of a given cube face
    pub fn rotateFace(&mut self, keyCode: String){

        if self.rotatingFace.is_some() { return; } //Early return if rotation already in progress

        //Axis of rotation depends on rotation direction // +1.0 for clockwise and -1.0 for counter-clockwise
        let dir = self.rotationDirection as i32 as f32; // Kinda ugly
        let mut axis = [0.0, 0.0, 0.0];
        
        self.rotatingFace = match keyCode.as_str() {

            "KeyR"  => { axis[2] =  dir; Some( RotationFace::Front( 0.0, axis) ) },
            "KeyO"  => { axis[2] = -dir; Some( RotationFace::Back(  0.0, axis) ) }, 
            "KeyG"  => { axis[0] =  dir; Some( RotationFace::Right( 0.0, axis) ) },  
            "KeyB"  => { axis[0] = -dir; Some( RotationFace::Left(  0.0, axis) ) }, 
            "KeyY"  => { axis[1] = -dir; Some( RotationFace::Up(    0.0, axis) ) },   
            "KeyW"  => { axis[1] =  dir; Some( RotationFace::Down(  0.0, axis) ) }, 
            _ => None, //Needed to satisfy non-exhaustive pattern complaint
        }
    }

    // Method to change direction of rotation for the faces of the cube
    pub fn changeRotationDirection(&mut self) -> RotationDirection
    {
        if self.rotatingFace.is_some() { return self.rotationDirection; } //Early return if rotation already in progress

        match self.rotationDirection {

            RotationDirection::Clockwise        => { self.rotationDirection = RotationDirection::CounterClockwise; 
                                                        RotationDirection::CounterClockwise },
            RotationDirection::CounterClockwise => { self.rotationDirection = RotationDirection::Clockwise; 
                                                        RotationDirection::Clockwise },
        }
    }

}


/*
    Enum to keep track of which face is currently rotating and how much it has rotated
*/
#[derive(Debug)] 
enum RotationFace {  
    Front(f32, [f32; 3]),   // Angle - starts at zero and goes to pi/2 ie. 90 degrees
    Back(f32, [f32; 3]),    // Axis of Rotation - changes sign for cw or ccw
    Left(f32, [f32; 3]),
    Right(f32, [f32; 3]),
    Up(f32, [f32; 3]),
    Down(f32, [f32; 3]),
}

impl RotationFace {

    // Method to get the ids of the cubes within a faces relative to its position in the rubik cube array
    fn get_cubie_ids(&self) -> [usize; 9] 
    {
        match self {  
            RotationFace::Front(_, _)  => array::from_fn(|i| i),       // 0..=8
            RotationFace::Back(_, _)   => array::from_fn(|i| i + 18),  // 18..=26
            RotationFace::Left(_, _)   => array::from_fn(|i| i * 3 + 2),     // 3,6,9,..,24
            RotationFace::Right(_, _)  => array::from_fn(|i| i * 3), //5,8,11,..,26
            RotationFace::Up(_, _)     => array::from_fn(|i| (i/3)*9 + i%3 +6), //6,7,8, 15,16,17, 24,25,26
            RotationFace::Down(_, _)   => array::from_fn(|i| (i/3)*9 + i%3),    //0,1,2,  9,10,11, 18,19,20
        }
    }

    // Method to check if rotation is less than half pi
    fn lt_half_pi(&self) -> bool
    {
        match self {  
            RotationFace::Front(angle, _)  => *angle < (consts::PI / 2.0),
            RotationFace::Back(angle, _)   => *angle < (consts::PI / 2.0), 
            RotationFace::Left(angle, _)   => *angle < (consts::PI / 2.0),
            RotationFace::Right(angle, _)  => *angle < (consts::PI / 2.0),
            RotationFace::Up(angle, _)     => *angle < (consts::PI / 2.0),
            RotationFace::Down(angle, _)   => *angle < (consts::PI / 2.0),
        }
    }

    // Method to rotate a rotation by a given arc length
    fn rotate(&mut self, arclen: f32)
    {
        match self {  
            RotationFace::Front(angle, _)  => *angle += arclen,
            RotationFace::Back(angle, _)   => *angle += arclen, 
            RotationFace::Left(angle, _)   => *angle += arclen, 
            RotationFace::Right(angle, _)  => *angle += arclen, 
            RotationFace::Up(angle, _)     => *angle += arclen, 
            RotationFace::Down(angle, _)   => *angle += arclen, 
        }
    }

}
