
# 3-D Rubik's Cube

### This is a simulation of a Rubik's Cube written in Rust and compiled to wasm.  

[Rubiks_Cube_Demo.webm](https://github.com/michael-k-burley/rubiks_cube/assets/71338143/9e85790a-9ee2-4372-8555-c8c8787d0a01)

#### How to use
Press the key which corresponds to the colour of the center cube of the side you want to spin.  
For example, if you want the side with the orange center to spin, press the key for the letter o.

The button shows the direction which all sides of the cube will presently rotate.  
Press the button to change the direction from clockwise to counter-clockwise or vice versa.

#### Intention
I had wanted to try building a rubik's cube for a while.

I built this project in order to gain a better understanding of rust and to gain some insight into how to compile rust to wasm, so as to be able to run it in a browser.

I learned quite a bit about developing applications for the browser and about webGL.  
I also learned about the importance of ordering linear transformations and about rust's notion of interior mutability.  

Full disclosure, I did cut a somewhat obvious corner by making each of the 27 smaller cubes have identically coloured sides. As opposed to having all of the interior sides of a cube coloured black. As they are in real life. This allowed me to rotate the individual cubes for each side without having to also translate these smaller cubes.  

#### Libraries used:
+ wasm_bindgen - to interface between javascript and wasm modules
+ web_sys &emsp;&emsp;&ensp; - to interact with all the standard web platform methods
+ webgl_matrix &ensp; - for vector and matrix operations

#### How to install and run
Can be built with the cmd: wasm-pack build --target web  
Then create a server from the project directory to run program in the browser.

#### Here is a short list of some resources that I found useful:

+ [About WebGL](https://webglfundamentals.org/webgl/lessons/webgl-fundamentals.html)
+ [The Wasm-Bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
+ [WebGL Examples](https://github.com/cx20/webgl-test/tree/master "Specifically: examples/rust/cube/src/lib.rs")
+ [Getting Started with WebGL and Rust](https://blog.logrocket.com/implement-webassembly-webgl-viewer-using-rust/)
