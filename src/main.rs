//! A spinning text cube
//! 
//! 
//! 4    +------+  6
//!     /|     /| 
//! 5  +------+ |  7
//!    | |    | | 
//! 0  | +----|-+  2
//!    |/     |/   
//! 1  +------+    3

#[derive(Debug, Clone, Copy)]
struct Matrix([[f32; 4]; 4]);

#[derive(Debug, Clone, Copy)]
struct Vector([f32; 4]);

/// Vertices of a cube in 3D space
const VERTICES : [Vector; 8] = [    Vector([-1.0, -1.0, -1.0, 1.0]),
    Vector([-1.0, -1.0,  1.0, 1.0]),
    Vector([ 1.0, -1.0, -1.0, 1.0]),
    Vector([ 1.0, -1.0,  1.0, 1.0]),
    Vector([-1.0,  1.0, -1.0, 1.0]),
    Vector([-1.0,  1.0,  1.0, 1.0]),
    Vector([ 1.0,  1.0, -1.0, 1.0]),
    Vector([ 1.0,  1.0,  1.0, 1.0]),
];

/// Indices of the vertices that make up each face of the cube
const FACES : [[u8; 4]; 6] = [    [1, 5, 7, 3],
    [3, 7, 6, 2],
    [0, 4, 5, 1],
    [2, 6, 4, 0],
    [0, 1, 3, 2],
    [5, 4, 6, 7],
];

/// Performs a matrix-vector multiplication
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

const SCREEN_WIDTH : usize = 80;
const SCREEN_HEIGHT : usize = 40;

/// Offset of the screen in the x direction
const OFFSET_X : f32 = SCREEN_WIDTH as f32 * 0.5;

/// Offset of the screen in the y direction
const OFFSET_Y : f32 = SCREEN_HEIGHT as f32 * 0.5;

/// Scaling factor for the x direction
const SCALE_X : f32 = SCREEN_WIDTH as f32 * 0.5;

/// Scaling factor for the y direction
const SCALE_Y : f32 = SCREEN_HEIGHT as f32 * 0.5;



fn main() {
    for frame_number in 0.. {
        let mut frame = [[b' ';SCREEN_WIDTH]; SCREEN_HEIGHT];

        /// Time elapsed since the beginning of the animation
        let t = frame_number as f32 * 0.01;
        let (c, s) = (t.cos(), t.sin());

        /// Transformation matrix that rotates the cube around the y-axis
        let cube_to_world = Matrix([
            // Each row is a column of a matrix.
            [  c, 0.0,   s, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [ -s, 0.0,   c, 0.0],
            [0.0, 0.0,-2.5, 1.0],
        ]);

        
        ///Transforms the 3D positions of the vertices of the cube into 2D screen coordinates by applying a transformation matrix and constants to the 3D positions to obtain the world coordinates and then projecting the world coordinates onto the 2D screen, storing the resulting 2D coordinates in the screen_pos array.
        let mut screen_pos = [[0.0, 0.0]; 8];
        for (v, s) in VERTICES.iter().zip(screen_pos.iter_mut()) {
            let world_pos = matrix_times_vector(&cube_to_world, v);
            let recip_z = 1.0 /  world_pos.0[2];
            let screen_x = world_pos.0[0] * recip_z * SCALE_X + OFFSET_X;
            let screen_y = world_pos.0[1] * recip_z * SCALE_Y + OFFSET_Y;
            *s = [screen_x, screen_y];
            // frame[screen_y as usize][screen_x as usize] = b'.';
        }


        ///Iterates over the faces of the cube and, for each face, uses the cull function to determine whether the face should be drawn. If the face should be drawn, it uses the draw_line function to draw lines between the vertices of the face to create a wireframe representation of the face on the screen. The end variable is used to store the last vertex of the face, so that lines can be drawn between consecutive vertices of the face in the correct order.
        for face in FACES {
            if !cull(screen_pos[face[0] as usize], screen_pos[face[1] as usize], screen_pos[face[2] as usize]) {
                let mut end = face[3];
                for start in face {
                    draw_line(&mut frame, screen_pos[start as usize], screen_pos[end as usize]);
                    end = start;
                }
            }
        }
        
        ///Iterates over the rows of the frame array, which represents the screen, and prints each row to the console as a string. The row variable is created by converting each row of frame to a string using the from_utf8 function from the str module of the std crate. The unwrap function is used to extract the resulting Result value, which represents the success or failure of the conversion. The resulting string is then printed to the console using the println! macro. This has the effect of printing the contents of the frame array to the console, which represents the wireframe representation of the spinning cube.
        for l in 0..SCREEN_HEIGHT {
            let row = std::str::from_utf8(&frame[l]).unwrap();
            println!("{}", row);
        }


        ///Uses the ANSI escape sequence \x1b[{}A to move the cursor up by SCREEN_HEIGHT lines. The print! macro is used to print this escape sequence to the console without a newline character at the end. This has the effect of moving the cursor up by SCREEN_HEIGHT lines, which is useful for creating an animation where the frame is redrawn in the same location on the screen for each iteration of the loop. Without this code, each frame of the animation would be printed on a new line below the previous frame, causing the animation to scroll down the screen.
        print!("\x1b[{}A;", SCREEN_HEIGHT);

        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}


///Determines whether a triangle formed by three 2D coordinates should be drawn by calculating the cross product of the edges of the triangle. If the cross product is positive, the triangle is culled (not drawn). If the cross product is negative, the triangle is not culled (drawn).
fn cull(p0: [f32; 2], p1: [f32; 2], p2: [f32; 2]) -> bool {
    let dx = [p1[0] - p0[0], p2[0] - p1[0]];
    let dy = [p1[1] - p0[1], p2[1] - p1[1]];
    dx[0] * dy[1] > dx[1] * dy[0]
}

///The draw_line function draws a line between two 2D coordinates in a 2D array of characters representing the screen. It does this by iterating over either the x or y coordinates of the line, calculating the corresponding x or y coordinates, and drawing horizontal or vertical lines in the array at these coordinates.
fn draw_line(frame: &mut [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT], start: [f32; 2], end: [f32; 2]) {
    let [x0, y0] = start;
    let [x1, y1] = end;
    let [dx, dy] = [x1 - x0, y1 - y0];
    if dy.abs() > dx.abs() {
        let ymin = y0.min(y1);
        let ymax = y0.max(y1);
        let iymin = ymin.ceil() as usize;
        let iymax = ymax.ceil() as usize;
        let dxdy = dx / dy;
        for iy in iymin..iymax {
            let ix = ((iy as f32 - y0) * dxdy + x0) as usize;
            frame[iy][ix] = b'|';
        }
    } else {
        let xmin = x0.min(x1);
        let xmax = x0.max(x1);
        let ixmin = xmin.ceil() as usize;
        let ixmax = xmax.ceil() as usize;
        let dydx = dy / dx;
        for ix in ixmin..ixmax {
            let iy = ((ix as f32 - x0) * dydx + y0) as usize;
            frame[iy][ix] = b'-';
        }
    }
}
