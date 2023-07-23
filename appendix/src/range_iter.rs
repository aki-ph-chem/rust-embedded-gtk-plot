struct Range {
    x_current: f32,
    x_fin: f32,
    step: f32,
}

impl Range {
    fn new(x_ini: f32, x_fin: f32, step: f32) -> Range {
        Range{x_current: x_ini, x_fin, step}
    }
}

impl Iterator for Range {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x_current <= self.x_fin {
            let value = self.x_current;
            self.x_current += self.step;
            Some(value)
        } else {
            None
        }
    }
}

fn main() {
    let range_1 = Range::new(0f32, 3f32, 0.1);
    let range_1_array: Vec<_> = range_1.collect();
    println!("range_1_array = \n{:?}", range_1_array);

    let range: Vec<_> = (-50..=50).map(|x| x as f32 / 50.0).collect();
    println!("range = \n{:?}", range);
}
