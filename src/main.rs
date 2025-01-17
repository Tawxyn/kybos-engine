fn main() {}

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
