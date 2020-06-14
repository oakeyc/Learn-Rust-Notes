fn main() {
    let tests: [f32; 4] = [0.0, -42.0, 100.0, 212.0];
    for far in tests.iter() {
        println!("{} F is equal to: {} C", far, convert_f_to_c(*far));
    }
}

fn convert_c_to_f(far: f32) -> f32 {
    // the equation is F = C * 8/5 + 32
    far * 9.0 / 5.0 + 32.0
}

fn convert_f_to_c(far: f32) -> f32 {
    // the equation is C = F * 9/5 + 32
    (far - 32.0) * 5.0 / 9.0
}
