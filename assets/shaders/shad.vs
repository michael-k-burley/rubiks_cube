
attribute vec3 a_coords;
attribute vec2 a_textCoord;

varying vec2 v_textCoord; 

uniform mat4 u_model;
uniform mat4 u_projection;

void main(void) {
    v_textCoord = a_textCoord;
    gl_Position = u_projection * u_model * vec4(a_coords, 1.0); 
}
