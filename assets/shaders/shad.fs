
precision mediump float;
varying vec2 v_textCoord; 

uniform sampler2D u_texture0; //spriteSheet

void main(void) { 

    gl_FragColor = texture2D(u_texture0, v_textCoord);
}
        