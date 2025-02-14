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

const EDGES: [(usize, usize); 12] = [
    (0, 1), (1, 3), (3, 2), (2, 0), // Top face (A-B-D-C)
    (4, 5), (5, 7), (7, 6), (6, 4), // Bottom face (E-F-H-G)
    (0, 4), (1, 5), (2, 6), (3, 7), // Vertical edges (A-E, B-F, C-G, D-H)
];

fn draw_line(screen: &mut Vec<Vec<char>>, x0: usize, y0: usize, x1: usize, y1: usize) {
   let mut x0 = x0 as isize;
   let mut y0 = y0 as isize;
   let x1 = x1 as isize;
   let y1 = y1 as isize;

   let dx = (x1 - x0).abs();
   let dy = -(y1 - y0).abs();
   let sx = if x0 < x1 { 1 } else { -1 };
   let sy = if y0 < y1 { 1 } else { -1 };
   let mut err = dx + dy;

   loop {
       if x0 >= 0 && x0 < SCREEN_WIDTH as isize && y0 >= 0 && y0 < SCREEN_HEIGHT as isize {
           screen[y0 as usize][x0 as usize] = '*';
       }
       if x0 == x1 && y0 == y1 { break; }
       let e2 = 2 * err;
       if e2 >= dy {
           err += dy;
           x0 += sx;
       }
       if e2 <= dx {
           err += dx;
           y0 += sy;
       }
   }
}

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

fn map_to_screen(ndc: [f32; 3]) -> (usize, usize) {
   let x_screen = ((ndc[0] + 1.0) * 0.5 * SCREEN_WIDTH) as usize;
   let y_screen = ((1.0 - ndc[1]) * 0.5 * SCREEN_HEIGHT) as usize;
   (x_screen, y_screen)
}

fn translate_vector(vector: Vector, z_offset: f32) -> Vector {
   Vector([vector.0[0], vector.0[1], vector.0[2] - z_offset, vector.0[3]])
}

// Resolution of terminal
const SCREEN_WIDTH: f32 = 120.0;
const SCREEN_HEIGHT: f32 = 30.0;
const ASPECT_RATIO: f32 = SCREEN_WIDTH / SCREEN_HEIGHT;
// Far and near values (determine depth of object in scene)
const FAR: f32 = 100.0;
const NEAR: f32 = 1.0;
const FOV: f32 = 90.0;

fn main() {
   let perspective = perspective_matrix(FOV, ASPECT_RATIO, NEAR, FAR);
   let mut screen = vec![vec![' '; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

   let mut transformed_vertices = Vec::new();

   // Transform all vertices and store their screen coordinates
   for vector in VERTICIES.iter() {
       let translated_vector = translate_vector(*vector, 3.0);  // Move cube back
       let transformed_vector = homogeneous_divide(transform_vector(translated_vector, perspective));

       if transformed_vector[0] >= -1.0 && transformed_vector[0] <= 1.0 &&
          transformed_vector[1] >= -1.0 && transformed_vector[1] <= 1.0 &&
          transformed_vector[2] >= -1.0 && transformed_vector[2] <= 1.0 {
           let (x_screen, y_screen) = map_to_screen(transformed_vector);
           transformed_vertices.push((x_screen, y_screen));
       } else {
           transformed_vertices.push((usize::MAX, usize::MAX)); // Mark invalid vertices
       }
   }

   // Draw edges
   for &(start, end) in EDGES.iter() {
       let (x0, y0) = transformed_vertices[start];
       let (x1, y1) = transformed_vertices[end];

       if x0 != usize::MAX && y0 != usize::MAX && x1 != usize::MAX && y1 != usize::MAX {
           draw_line(&mut screen, x0, y0, x1, y1);
       }
   }

   // Print the screen buffer
   for row in screen {
       let line: String = row.into_iter().collect();
       println!("{}", line);
   }
}
 