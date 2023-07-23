// クロージャーを返す
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new( |x| x + 1 )
}

// Boc<dyn Fn(args) -> retur>を受け取る
fn diff_x(f: Box<dyn Fn(f64) -> f64>, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x)) / h
}

fn main() {
    let closure_x = return_closure();
    println!("closure_x(99) = {}", closure_x(99));

    let res = diff_x(Box::new(|x| 2.0 * x.powi(2)), 1.0, 1.0E-3);
    println!("res = {}", res);
}
