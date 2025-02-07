extern crate raytracer;
use raytracer::matrix::*;
use raytracer::tuple::*;
use std::f64::consts::PI;
use raytracer::canvas::*;
use crate::raytracer::approx_eq::ApproxEq;
fn main(){

    let point_a = Tuple::point(1.0, 0.0, 1.0);
    let rotation_a = Matrix::rotation_x(PI / 2.0);
    let scaling_b = Matrix::scaling(5.0, 5.0, 5.0);
    let translation_c = Matrix::translation(10.0, 5.0, 7.0);

    let point_2 = rotation_a * point_a;
    let point_2_check = Tuple::point(1.0, -1.0, 0.0);
    assert!((point_2_check).fuzzy_eq(point_2));
    
    let point_3 = scaling_b * point_2;
    let point_3_check = Tuple::point(5.0, -5.0, 0.0);
    assert!((point_3_check).fuzzy_eq(point_3));

    let point_4 = translation_c * point_3;
    let point_4_check = Tuple::point(15.0, 0.0, 7.0);
    assert!((point_4_check).fuzzy_eq(point_4));

    let rotation_aa = Matrix::rotation_x(PI / 2.0);
    let scaling_bb = Matrix::scaling(5.0, 5.0, 5.0);
    let translation_cc = Matrix::translation(10.0, 5.0, 7.0);

    let point_t = translation_cc * scaling_bb * rotation_aa;
    let point_t_check = Tuple::point(15.0, 0.0, 7.0);
    assert!((point_t * point_a).fuzzy_eq(point_t_check));


    // draw a clock
    let mid_point = Tuple::point(50.0, 50.0, 0.0);

    // create loop that keeps rotating mid_point
    // and write to canvas

    // create canvas
    // Maybe give to ppm an extra var -> file_name 
    let mut new_canvas = Canvas::new(100, 100);
    let rainbow = Color::new(1.0, 0.0, 0.0);
    

    new_canvas.write_pixel(mid_point.x, mid_point.y, rainbow);

    let _canvas = new_canvas.canvas_to_ppm();

    //TODO: REWRITE canvas so it works with float instead of usize
    // page 56
}
