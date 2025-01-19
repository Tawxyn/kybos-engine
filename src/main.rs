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

fn matrix_transformation() {
    let mut view_matrix: [[f32; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

// Resolution
const SCREEN_WIDTH: i32 = 30;
const SCREEN_HEIGHT: i32 = 120;

fn main() {}
