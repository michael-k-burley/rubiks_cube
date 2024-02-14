
import init, { rubix_cube_simulation } from "../pkg/rubix.js";

const CANVAS_ID = "canvasID"; 
const canvas = document.getElementById(CANVAS_ID);

async function run() {
  
  await init(); //Init() function auto generated when compiled?

  //Set canvas heigth, width and border dim (can also set in index.html)
  canvas.width = 600;
  canvas.height = 400;
  canvas.style = "border:25px solid #000000;";

  rubix_cube_simulation();  
}

run(); 
