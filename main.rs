//GraphicsUsingOOP - Spinning Boxes
//Max Edward | 4/6/23

use minifb::{Key, Window, WindowOptions};   //Minifb library for 2D graphics.
                                            //Key: maps keyboard input to check for keys pressed.
                                            //Window: window on users screen, provides methods for creating, updating, closing window
                                            //WindowOptions: configuration for window settings.

use rand::prelude::*;   //Rand library for generating random numbers.
                        //Prelude is a module that contains commonly used features so Rust doesn't import everything.
                        //The "*" is shorthand for "import everything", meaning that it is going to get everything from the prelude module.


const width: usize = 640; //Width of window (pixels)
const height: usize = 480; //Height of window (pixels)
const num_boxes: usize = 10; //Number of boxes to include

//Main Function
fn main() {
    // Create the window
    let mut window = Window::new(
        "Spinning Boxes - Press Esc to exit", //Name of window
        width,                    //Sets width as width variable
        height,                   //Sets height as height variable
        WindowOptions::default(), //Sets the window options to defaults
    )
    //Panic method to handle errors when working with result object
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Create a buffer to store the window's pixels
    let mut buffer: Vec<u32> = vec![0; width * height];

    // Initialize the boxes' positions, rotations, and colors
    let mut boxes: Vec<(f32, f32, f32, u32)> = vec![];
    let mut rng = thread_rng();
    for i in 0..num_boxes {
        let x = (width / 2) as f32;
        let y = (height / 2) as f32;
        let angle = i as f32 * 360.0 / num_boxes as f32;
        let color = rng.gen_range(0..=0xFFFFFF);
        boxes.push((x, y, angle, color));
    }

    let start_color: u32 = 0x000000;
    let end_color: u32 = 0xFF00FF;

    fn lerp(start: u32, end: u32, frac: f32) -> u32 {
        let red_start = (start >> 16) & 0xFF;
        let green_start = (start >> 8) & 0xFF;
        let blue_start = start & 0xFF;

        let red_end = (end >> 16) & 0xFF;
        let green_end = (end >> 8) & 0xFF;
        let blue_end = end & 0xFF;

        let red = lerp_channel(red_start, red_end, frac);
        let green = lerp_channel(green_start, green_end, frac);
        let blue = lerp_channel(blue_start, blue_end, frac);

        (red << 16) | (green << 8) | blue
    }

    fn lerp_channel(start: u32, end: u32, frac: f32) -> u32 {
        (start as f32 + frac * (end as f32 - start as f32)).round() as u32
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer to the gradient background color
        for y in 0..height {
            let frac = y as f32 / height as f32;
            let color = lerp(start_color, end_color, frac);
            for x in 0..width {
                let index = y * width + x;
                buffer[index] = color;
            }
        }

        // Draw the boxes
        let size = 50;
        let half_size = size / 2;
        for box_data in &boxes {
            let (x, y, angle, color) = *box_data;
            let angle_rad = angle * std::f32::consts::PI / 180.0;
            let cos = angle_rad.cos();
            let sin = angle_rad.sin();
            let x_box = x + (width / 4) as f32 * cos;
            let y_box = y + (height / 4) as f32 * sin;
            for i in (-half_size)..half_size {
                for j in (-half_size)..half_size {
                    let x_rot = i as f32 * cos - j as f32 * sin;
                    let y_rot = i as f32 * sin + j as f32 * cos;
                    let x_pix = (x_box + x_rot) as isize;
                    let y_pix = (y_box + y_rot) as isize;
                    if x_pix >= 0 && x_pix < width as isize && y_pix >= 0 && y_pix < height as isize
                    {
                        let index = (y_pix * width as isize + x_pix) as usize;
                        buffer[index] = color;
                    }
                }
            }
        }

        // Update the boxes' positions and rotations
        for box_data in &mut boxes {
            let (_, _, angle, _) = *box_data;
            let new_angle = angle + 1.0;
            let new_angle = if new_angle >= 360.0 {
                new_angle - 360.0
            } else {
                new_angle
            };
            box_data.2 = new_angle;
        }

        // Display the buffer in the window
        window
            .update_with_buffer(&buffer, width, height)
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
    }
}
