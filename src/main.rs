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

fn transform_vector(vector: Vector, matrix: [[f32; 4]; 4]) -> Vector {

   let mut result = [0.0; 4];

   for row in 0..4 {

      result[row] = vector.0[0] * matrix[row][0]
                  + vector.0[1] * matrix[row][1]
                  + vector.0[2] * matrix[row][2]
                  + vector.0[3] * matrix[row][3];
   }
      Vector(result)
}

fn perspective_matrix(fov_degrees: f32, aspect_ratio: f32, 
                        near: f32, far: f32) -> [[f32; 4]; 4] {
   
   // Convert degrees to radians for FOV
   let fov_radians = fov_degrees.to_radians();
   // Calulate focal point
   let f = 1.0 / f32::tan(fov_radians / 2.0);
   
   // Return perspective matrix
   [
      [f / aspect_ratio, 0.0, 0.0 ,0.0],
      [0.0, f, 0.0 ,0.0],
      [0.0, 0.0, (far + near) / (near - far), (2.0 * far * near) / (near - far)],
      [0.0, 0.0, -1.0 ,0.0],
   ]
}

fn homogeneous_divide(vector: Vector) -> [f32; 3] {

   let mut result = [0.0; 3];

   result[0] = vector.0[0] / vector.0[3];  // x / w
   result[1] = vector.0[1] / vector.0[3];  // y / w
   result[2] = vector.0[2] / vector.0[3];  // z / w
   
   result
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
      let transformed_vector = homogeneous_divide(transform_vector(*vector, perspective));
      println!("{:?}", transformed_vector);
   }
}
 