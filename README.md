# 3D Cube Projection in Rust

STILL UNDER DEVELOPMENT

This project simulates a 3D cube projection onto a 2D plane using Rust. 
It applies matrix transformations and a perspective projection to render the cube's vertices. 
The code demonstrates fundamental concepts of 3D graphics, including perspective projection, vertex transformations, and rendering calculations. 
Ideal for exploring 3D programming basics.

### 1. **Homogeneous Coordinates**
The vertices of the cube are represented in 4D homogeneous coordinates, which allow for efficient mathematical transformations. The extra dimension (W) is used to facilitate operations like perspective division, which simulates depth and distance.

### 2. **Matrix Transformations**
To project the 3D cube onto a 2D plane, we use a perspective projection matrix. This matrix transforms each vertex of the cube by scaling its X, Y, and Z coordinates relative to the camera's position and field of view. These transformations simulate the way objects appear smaller as they move further from the viewer.

### 3. **Perspective Projection**
The perspective projection simulates the depth of a 3D scene by converting 3D coordinates into 2D screen space. The field of view (FOV), aspect ratio, and near/far clipping planes determine how the 3D space is "squeezed" into the 2D view. The projection matrix incorporates these parameters to create realistic depth effects.

### 4. **Screen Mapping**
Once the vertices are transformed, they can be mapped to a 2D coordinate space that represents the screen or terminal. This process adjusts the coordinates to fit within the resolution of the output display.

### 5. **Rendering**
Although this project currently outputs the transformed coordinates as text, it lays the groundwork for rendering a visual representation of the cube, such as ASCII art or graphical output.
