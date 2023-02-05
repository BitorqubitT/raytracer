use std::array;
use std::ops::{Add, Sub, Mul};
use std::vec::Vec;

// Lets commit this
// Finish writing to ppm
// Still need to finish this
// Current solution seems VERY impractical
// then continue page 20
// Canvas to ppm seems to work.
// Fix value < 0 and > 255 issue.

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Debug)]
pub struct Canvas {
    // Maybe increase this dtype.
    pub width: usize,
    pub height: usize,  
    // Is vec the best solution to hold tuples?
    // Vec needs to know what datatype it will use.
    // Can you use a combination ?
    pixels: Vec<Color>,
}

impl Color{

    pub fn new(red: f64, green: f64, blue: f64) -> Self{
        Color {red, green, blue}
    }
}

impl Add for Color{
    type Output = Self;
// check rhs of self
// rhs vs other, rhs makes sense
    fn add(self, other: Color) -> Self::Output {
        Color::new(self.red + other.red, 
                 self.green + other.green,
                  self.blue + other.blue)
    }
}

impl Sub for Color{
    type Output = Self;
    
    fn sub(self, other: Color) -> Self::Output {
        Color::new(self.red - other.red, 
                 self.green - other.green,
                  self.blue - other.blue)
    }
}

impl Mul<f64> for Color{
    type Output = Self;
    
    fn mul(self, other: f64) -> Self::Output {
        Color::new(self.red * other, 
                 self.green * other,
                  self.blue * other)
    }
}

// Math operations on colors:
impl Color{
    // Why use & here?
    // If the body has no return
    // It returns whatever is in thet ()
    pub fn hadamard_product(&self, other: Color) -> Color{
        Color::new (self.red * other.red,
                  self.green * other.green,
                   self.blue * other.blue
        ) 
    }
}

impl Canvas{
// We dont need to give a canvas to these methods.
// We just use self because we can only call these methods on canvas, which is self.

    pub fn new(width: usize , height: usize) -> Self { 
        // Need to change width and height to usize,why?>??????
        Self {width, height, pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        
        // This is basically vec[calc_pixel(x,y)].
        self.pixels[self.calc_pixel_index(x, y)]
    }

    // We dont give anything back here, we just mut color in vec.
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        // cannot borrow *self as immutable because is is also borrowed as mutable
        // Error if I put self.calc in self.pixels
        let new_index = self.calc_pixel_index(x, y);
        self.pixels[new_index] = color;
    }
    
    // Why the &self here
    // Does this even make sense?
    pub fn calc_pixel_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    // Function to convert the canvas to a ppm file
    // Maybe use self instead of &self?
    pub fn canvas_to_ppm(self) -> String{
        let header = format!("P3\n\
                                      {} {}\n\
                                      255", self.width, self.height );

        // Colors now would be numbers between 0 and 1.
        // However we need to scale between 0 and max in header file.
        // 255 should be put in a variable or param.

        // TODO:
        // Write new vector to ppm
        // How do we deal with too long lines?, I dont understand
        // What if our image exceeds the 70/3 width
        
        // let mut hold_new_rgb = vec![0.0; self.width * self.height * 3];

        // Hold the normalised values.
        let mut normalised_rgb = Vec::new();
        let mut counter = 0;
        println!("lalalalaa,{},{}", self.width, self.height);
        // 0..self.height-1
        
        for j in 0..self.height {
            for i in 0..self.width {
                println!("ffffff, {},{},{}", counter, i, j);

                // Can change this to pixel_at.red etc
                let m = self.pixel_at(i, j);
                let red = m.red * 255.0;
                let green = m.green * 255.0;
                let blue = m.blue * 255.0;

                // Here we should create something which check upper and lower bound before pushing.
                // What is the most logical fix for this?
                // I shouldnt do this for every color
                // Can i loop over red green blue?
                // Or generalise it somehow
                if red > 255.0 {
                    println!("red is, {}", red);
                    let red = 255.0;
                } else if red < 0.0 {
                    let red = 0.0;
                }

                normalised_rgb.push(red);
                normalised_rgb.push(green);
                normalised_rgb.push(blue);
                counter += 1;
            }
        }
        
        println!("Normal:, {:?}", normalised_rgb);

        return header;        
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color(){
        let nice_color = Color::new(0.3, 4.0, 2.1);

        assert!(nice_color.red == 0.3);
        assert!(nice_color.green == 4.0);
        assert!(nice_color.blue == 2.1);
    }

    #[test]
    fn add_color(){
        let spice_color = Color::new(0.4, 2.0, 3.9);
        let vspice_color = Color::new(3.0, 4.0, 10.9);
        let combo_color = Color::new(3.4, 6.0, 14.8);
        let added_color = spice_color + vspice_color;

        assert!(added_color.red == combo_color.red);
        assert!(added_color.green == combo_color.green);
        assert!(added_color.blue == combo_color.blue);
    }

    #[test]
    fn sub_color(){
        let spice_color = Color::new(0.4, 2.0, 13.9);
        let vspice_color = Color::new(3.0, 4.0, 10.9);
        let combo_color = Color::new(-2.6, -2.0, 3.0);
        let added_color = spice_color - vspice_color;

        assert!(added_color.red == combo_color.red);
        assert!(added_color.green == combo_color.green);
        assert!(added_color.blue == combo_color.blue);
    }

    #[test]
    fn mul_color(){
        // * by 0.x gives "weird" rounding due too precision and maximum bits.
        // Keep this in mind, how to deal witht his.
        let spice_color = Color::new(4.0, 2.0, 3.9);
        let combo_color = Color::new(12.0, 6.0, 11.7);
        let added_color = spice_color * 3.0;
        assert!(added_color.red == combo_color.red);
        assert!(added_color.green == combo_color.green);
        assert!(added_color.blue == combo_color.blue);
    }

    #[test]
    fn hadamard_product(){
        let spice_color = Color::new(4.0, 2.0, 13.0);
        let vspice_color = Color::new(3.0, 4.0, 10.0);
        let added_color = spice_color.hadamard_product(vspice_color);

        let had_color = Color::new(12.0, 8.0, 130.0);

        assert!(added_color.red == had_color.red);
        assert!(added_color.green == had_color.green);
        assert!(added_color.blue == had_color.blue);
    }

    #[test]
    fn create_canvas(){
        let new_canvas = Canvas::new(10, 20);
        // For every pixel we have to check if it is initiated to 0,0,0.
        // use function that returns the pixel in a canvas at x,y.
        // Need to check every color in canvas.
        for i in 0..10 {
            for j in 0..20 {
                let pixel = new_canvas.pixel_at(i, j);
                assert!(pixel.red == 0.0);
                assert!(pixel.green == 0.0);
                assert!(pixel.blue == 0.0);
            }
        }
    }

    #[test]
    fn write_color_to_canvas(){
        // Does this need to be mut?
        let mut new_canvas = Canvas::new(10, 20);

        // change pixel at x,y
        // recall that pixel
        let rood = Color::new(24.0, 250.0, 20.0);
        new_canvas.write_pixel(6, 6, rood);

        // prob want a better/easier way to just check, is pixel red or not
        // Compare the whole vector
        let pixel = new_canvas.pixel_at(6, 6);
        assert!(pixel.red == 24.0);
        assert!(pixel.green == 250.0);
        assert!(pixel.blue == 20.0);
    }

    #[test]
    fn canvas_to_ppm_header() {
        let new_canvas = Canvas::new(10, 20);

        let test_header = format!("P3\n\
                                          10 20\n\
                                          255");
        // Can use cargo test canvas -- --nocapture
        // This shows prints
        let header = new_canvas.canvas_to_ppm();
        assert!(test_header == header);
    }

    #[test]
    fn canvas_to_ppm_pixel_data() {
        let mut new_canvas = Canvas::new(5, 3);

        let rood = Color::new(1.5, 0.0, 0.0);
        let not_rood = Color::new(0.0, 0.5, 0.0);
        let defo_not_rood = Color::new(-0.5, 0.0, 1.0);

        new_canvas.write_pixel(0, 0, rood);
        new_canvas.write_pixel(2, 1, not_rood);
        new_canvas.write_pixel(4, 2, defo_not_rood);
        println!("{:?}", new_canvas);
        let pixels_boys = new_canvas.canvas_to_ppm();
        println!("{:?}", pixels_boys);
    }

}





