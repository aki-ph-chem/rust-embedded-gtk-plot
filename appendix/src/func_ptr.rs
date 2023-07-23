fn q_func(x: f64) -> f64 {
    2.0 * x.powi(2)
}

fn diff_x(f: fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x)) / h
}

// 以下はコンパイルできない
/*
fn diff_x_c(f: Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x)) / h
}
*/

fn main() {
    let res = diff_x(q_func, 1.0, 1.0E-3);
    println!("res = {}", res);

    let res = diff_x(|x| x * x , 1.0, 1.0E-3);
    println!("res = {}", res);
}
