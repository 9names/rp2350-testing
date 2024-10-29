/// Mandelbrot code adapted from rosetta code example
/// https://rosettacode.org/wiki/Mandelbrot_set#Rust
use num_complex::Complex;

// Use a framebuffer large enough for pico display pack
// We could (and probably should?) only calculate + draw a line at a time
const WIDTH: usize = 240;
const HEIGHT: usize = 135;

pub fn generate() {
    static mut FB: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    let max_iterations = 256u16;
    let img_side = 800u32;
    let cxmin = -2f32;
    let cxmax = 1f32;
    let cymin = -1.5f32;
    let cymax = 1.5f32;
    let scalex = (cxmax - cxmin) / img_side as f32;
    let scaley = (cymax - cymin) / img_side as f32;

    // Calculate for each pixel
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let cx = cxmin + x as f32 * scalex;
            let cy = cymin + y as f32 * scaley;

            let c = Complex::new(cx, cy);
            let mut z = Complex::new(0f32, 0f32);

            let mut i = 0;
            for t in 0..max_iterations {
                if z.norm() > 2.0 {
                    break;
                }
                z = z * z + c;
                i = t;
            }

            let offset = x + y * WIDTH;
            unsafe {
                FB[offset] = i as u8;
            }
        }
    }
}
