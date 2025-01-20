/*

         A               B
          o-------------o
         /|            /|            Y
        / |           / |            |1
     C o-------------o D|            |
       |  |    +     |  |            |_________X
       |E o----------|--o F         /0        1
       | /           | /           /
       |/            |/          1/
       o-------------o           Z
      G              H


*/

#[derive(Debug, Clone, Copy)]
struct Vector([f32; 4]);
//          X-----Y-----Z-----W
const VERTICIES: [Vector; 8] = [
    Vector([-1.0, 1.0, -1.0, 1.0]),  // A
    Vector([1.0, 1.0, -1.0, 1.0]),   // B
    Vector([-1.0, 1.0, 1.0, 1.0]),   // C
    Vector([1.0, 1.0, 1.0, 1.0]),    // D
    Vector([-1.0, -1.0, -1.0, 1.0]), // E
    Vector([1.0, -1.0, -1.0, 1.0]),  // F
    Vector([-1.0, -1.0, 1.0, 1.0]),  // G
    Vector([1.0, -1.0, 1.0, 1.0]),   // H
];

/* 
fn matrix_transformation() {
    let mut view_matrix: [[f32; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}
*/


fn transform_vector(vector: Vector, matrix: [[f32; 4]; 4]) -> Vector {

   let mut result = [0.0; 4];

   for row in 0..4 {}

   let x = vector.0[0] * matrix[0][0];
   let y = vector.0[1] * matrix[0][1];
   let z = vector.0[2] * matrix[0][2];
   let w = vector.0[3] * matrix[0][3];

   Vector([x, y, z, w])
}

fn perspective_matrix(fov_degrees: f32, aspect_ratio: f32, 
                        near: f32, far: f32) -> [[f32; 4]; 4] {
   
   // Convert degrees to radians for FOV
   let fov_radians = fov_degrees * std::f32::consts::PI / 180.0;
   // Calulate focal point
   let focal_point = 1.0 / f32::tan(fov_radians / 2.0);
   
   // Return perspective matrix
   [
      [focal_point / aspect_ratio, 0.0, 0.0 ,0.0],
      [0.0, focal_point, 0.0 ,0.0],
      [0.0, 0.0, (far + near) / (near - far), (2.0 * far * near) / (near - far)],
      [0.0, 0.0, -1.0 ,0.0],
   ]
}

// Resolution of terminal
const SCREEN_WIDTH: f32 = 30.0;
const SCREEN_HEIGHT: f32 = 120.0;
const ASPECT_RATIO: f32 = SCREEN_WIDTH / SCREEN_HEIGHT;
// Far and near values (determine depth of object in scene)
const FAR: f32 = 100.0;
const NEAR: f32 = 1.0;
const FOV: f32 = 90.0;

fn main() {
   let perspective = perspective_matrix(FOV, ASPECT_RATIO, NEAR, FAR);

   // For loop to run all verticies through transformation (perspective * vectors)
   for vector in VERTICIES.iter() {
      let transformed_vector = transform_vector(*vector, perspective);
      println!("{:?}", transformed_vector);
   }
}
 