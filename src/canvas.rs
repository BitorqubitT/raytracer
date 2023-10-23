use std::ops::{Add, Sub, Mul};
use std::vec::Vec;
use std::fs::File;
use std::io::Write;

// TODO
// Continue page 25
// Check if the "matrix" that I have is logical.
// Can use cargo test canvas -- --nocapture
// This shows prints

// Questions:
// Think about stack vs heap and thus the use of &.
// Am I using & in the right way.
// Check this

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Debug)]
pub struct Canvas {

    // TODO
    // Maybe increase this dtype.
    pub width: usize,
    pub height: usize,  
    pixels: Vec<Color>,
}

// clean up add impl together for the same struct 

impl Color{

    pub fn new(red: f64, green: f64, blue: f64) -> Self{
        Color {red, green, blue}
    }
}

impl Color{
    
    pub fn as_array(&self) -> [f64; 3] {
        [self.red, self.green, self.blue]
    }
}

impl Add for Color{
    type Output = Self;
// rhs vs other, rhs makes sense
    fn add(self, other: Color) -> Self::Output {
        Color::new(self.red + other.red, 
                 self.green + other.green,
                  self.blue + other.blue)
    }
}

//Should I define type aswell Sub<f64> ?
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

impl Color{
    // Why use & here?
    // Is & used to put it on stack
    // Also why use pub?
    // If the body has no return
    // It returns whatever is in the ()
    pub fn hadamard_product(&self, other: Color) -> Color{
        Color::new (self.red * other.red,
                  self.green * other.green,
                   self.blue * other.blue
        ) 
    }
}

// 127.5 is not possible i think. how do we deal with rounding?
impl Color{
    pub fn scale_colors(&mut self) {
        self.red = self.red * 255.0;
        self.blue = self.blue * 255.0;
        self.green = self.green * 255.0;

        if self.red > 255.0 {
            self.red = 255.0;
        } else if self.red < 0.0 {
            self.red = 0.0;
        }

        if self.green > 255.0 {
            self.green = 255.0;
        } else if self.green < 0.0 {
            self.green = 0.0;
        }

        if self.blue > 255.0 {
            self.blue = 255.0;
        } else if self.blue < 0.0 {
            self.blue = 0.0;
        }
    }
}

impl Canvas{
// We dont need to give a canvas to these methods.
// We just use self because we can only call these methods on canvas, which is self.

    pub fn new(width: usize , height: usize) -> Self { 
        // Is there not a better way to implement an matrix
        Self {width, height, pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[self.calc_pixel_index(x, y)]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        // cannot borrow *self as immutable because is is also borrowed as mutable
        // Error if I put self.calc in self.pixels
        let new_index = self.calc_pixel_index(x, y);
        self.pixels[new_index] = color;
    }
    
    // Why the &self here
    // Use this because our matrix is a vector, this calculates when to split row.
    pub fn calc_pixel_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn canvas_to_ppm(self) -> String{
        let mut header = format!("P3\n\
                                      {} {}\n\
                                      255 \n", self.width, self.height );
        println!("{:?}", header);
        
        // Rescale colors and add them to a string
        let mut normalised_rgb = String::from("");

        for j in 0..self.height {
            for i in 0..self.width {
                let mut m = self.pixel_at(i, j);
                m.scale_colors();

                for color_value in m.as_array().iter() {
                    normalised_rgb.push_str(&color_value.to_string());
                    normalised_rgb.push_str(" ");
                    }
                }
        }   
        let mut row =  String::new();

        // We keep writing words to row and we push the row to image when we hit 70
        for word in normalised_rgb.split_whitespace() {
            if row.len() + word.len() + 1 > 70 {
                header.push_str(&row);
                header.push_str("\n");
                row.clear();
            }
            row.push_str(word);
            row.push(' ');
        }

        header.push_str(&row);
        
        // Some process programs need file to end with newline
        header.push_str("\n");

        println!("{}", header);
        // Use as_bytes() to write variable to file instead of just a string
        // Change location
        let mut f = File::create("foo.ppm").expect("Unable to create file");
        f.write_all(header.as_bytes()).expect("Unable to write data");

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
    fn write_color_to_canvas() {
        // Does this need to be mut?
        let mut new_canvas = Canvas::new(10, 20);

        // change pixel at x,y
        // recall that pixel
        let rood = Color::new(24.0, 250.0, 20.0);
        new_canvas.write_pixel(6, 6, rood);

        // TODO:
        // Compare the whole vector
        let pixel = new_canvas.pixel_at(6, 6);
        assert!(pixel.red == 24.0);
        assert!(pixel.green == 250.0);
        assert!(pixel.blue == 20.0);
    }

    #[test]
    fn canvas_to_ppm_header() {
        // Should remove this test.
        // canvas_to_ppm return the array, not just the header.
        let new_canvas = Canvas::new(10, 20);

        let test_header = format!("P3\n\
                                          10 20\n\
                                          255");
        let header = new_canvas.canvas_to_ppm();
        assert!(test_header == header);
    }

// Clean up canvas_pixel_data and longer lines?
// Combine the test when both work?

    #[test]
    fn canvas_to_ppm_pixel_data() {
        let mut new_canvas = Canvas::new(5, 3);

        let rood = Color::new(1.5, 0.0, 0.0);
        let not_rood = Color::new(0.0, 0.5, 0.0);
        let defo_not_rood = Color::new(-0.5, 0.0, 1.0);

        new_canvas.write_pixel(0, 0, rood);
        new_canvas.write_pixel(2, 1, not_rood);
        new_canvas.write_pixel(4, 2, defo_not_rood);
        // Indent with _ because, it is not used anywhere.
        let _pixels_boys = new_canvas.canvas_to_ppm();
    }

    #[test]
    fn canvas_to_ppm_longer_lines() {
        let mut new_canvas = Canvas::new(100, 20);

        let rainbow = Color::new(1.0, 0.0, 0.0);

        for j in 0..new_canvas.height {
            for i in 0..new_canvas.width {
                new_canvas.write_pixel(i, j, rainbow);
            }
        } 
        let _canvas = new_canvas.canvas_to_ppm();
        // Do some type of assert here
    }
}
