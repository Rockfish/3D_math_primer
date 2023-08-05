use math_lib_3d::add;
use std::f64::consts::*;

fn main() {
    println!("3d math - baby steps");

    let angle: f64 = 60.0;
    let radius: f64 = 1.0;

    let (x, y) = get_x_y(angle.to_radians(), radius);
    println!(
        "angle: {:?}  radius: {:?}  x: {:.5}  y: {:.5}",
        angle, radius, x, y
    );

    let (sin, cos) = angle.to_radians().sin_cos();
    println!(
        "angle: {:?}  radius: {:?}  x: {:.5}  y: {:.5}",
        angle,
        radius,
        radius * cos,
        radius * sin
    );

    let angle = get_angle(x, y);
    println!(
        "x: {:.5}  y: {:.5}  angle: {:.5}  degrees: {:.5} ",
        x,
        y,
        angle,
        angle.to_degrees()
    );

    let angle = get_degree(x, y);
    println!("x: {:.5}  y: {:.5}  angle: {:.5}", x, y, angle);

    // println!("PI * 6.0/4.0: {:.6}  PI+FRAC_PI_2: {:.6}  TAU * 3.0/4.0: {:.6}", PI * 6.0/ 4.0f64, PI + FRAC_PI_2, TAU * 3.0/4.0);

    println!("% : {:?}", 6 % 2);

    println!("add: {}", add(5, 6));
}

fn get_x_y(angle: f64, radius: f64) -> (f64, f64) {
    let x = radius * angle.cos();
    let y = radius * angle.sin();
    (x, y)
}

fn get_angle(x: f64, y: f64) -> f64 {
    let angle = (y / x).atan();
    match (x.is_sign_negative(), y.is_sign_negative()) {
        (false, false) => angle,
        (true, false) => angle + FRAC_PI_2,
        (true, true) => angle + PI,
        (false, true) => angle + PI + FRAC_PI_2,
    }
}

fn get_degree(x: f64, y: f64) -> f64 {
    let angle = (y / x).atan().to_degrees();
    match (x.is_sign_negative(), y.is_sign_negative()) {
        (false, false) => angle,
        (true, false) => angle + 90.0,
        (true, true) => angle + 180.0,
        (false, true) => angle + 270.0,
    }
}
