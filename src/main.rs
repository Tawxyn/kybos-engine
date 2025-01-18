#[derive(Debug, Clone, Copy)]
struct Matrix([[f32; 4]; 4]);

#[derive(Debug, Clone, Copy)]
struct Vector([f32; 4]);

const VERTICES: [Vector; 8] = [
    Vector([-1.0, -1.0, -1.0, 1.0]),
    Vector([-1.0, -1.0, 1.0, 1.0]),
    Vector([1.0, -1.0, -1.0, 1.0]),
    Vector([1.0, -1.0, 1.0, 1.0]),
    Vector([-1.0, 1.0, -1.0, 1.0]),
    Vector([-1.0, 1.0, 1.0, 1.0]),
    Vector([1.0, 1.0, -1.0, 1.0]),
    Vector([1.0, 1.0, 1.0, 1.0]),
];

fn matrix_times_vector(m: &Matrix, v: &Vector) -> Vector {
    let [mx, my, mz, mw] = &m.0;
    let [x, y, z, w] = v.0;
    // The product is the weighted sum of the columns.
    Vector([
        x * mx[0] + y * my[0] + z * mz[0] + w * mw[0],
        x * mx[1] + y * my[1] + z * mz[1] + w * mw[1],
        x * mx[2] + y * my[2] + z * mz[2] + w * mw[2],
        x * mx[3] + y * my[3] + z * mz[3] + w * mw[3],
    ])
}

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 40;
const OFFSET_X: f32 = SCREEN_WIDTH as f32 * 0.5;
const OFFSET_Y: f32 = SCREEN_HEIGHT as f32 * 0.5;
const SCALE_X: f32 = SCREEN_WIDTH as f32 * 0.5;
const SCALE_Y: f32 = SCREEN_HEIGHT as f32 * 0.5;

fn main() {
    for frame_number in 0.. {
        let mut frame = [[b' '; SCREEN_WIDTH]; SCREEN_HEIGHT];

        let t = frame_number as f32 * 0.01;
        let (c, s) = (t.cos(), t.sin());

        let cube_to_world = Matrix([
            // Each row is a column of a matrix.
            [c, 0.0, s, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-s, 0.0, c, 0.0],
            [0.0, 0.0, -2.5, 1.0],
        ]);
    }
}
