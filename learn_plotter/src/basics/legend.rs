use plotters::prelude::*;
use std::path::PathBuf;
use std::env;
use std::fs;

fn mkdir(path: &str) -> std::io::Result<()> {
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                return Ok(());
            }
            Err(e)
        }
    }
}

// 関数値を計算する区間を生成する
struct Range {
    x_current: f64,
    x_fin: f64,
    step: f64,
}

impl Range {
    fn new(x_ini: f64, x_fin: f64, step: f64) -> Range {
        Range {x_current: x_ini, x_fin, step}
    }
}

impl Iterator for Range {
    type Item = f64;

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

const TWO_PI: f64 = 2.0 * std::f64::consts::PI;

// 簡単なplot 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut result_path = PathBuf::new();  
    result_path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
    result_path.push("target");
    result_path.push("figure");
    mkdir(result_path.to_str().unwrap())?;
    result_path.push("legend.png");

    let drawing_area = BitMapBackend::new(result_path.to_str().unwrap(), (600, 400))
        .into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let (x_min, x_max, y_min, y_max) = (-TWO_PI, TWO_PI, -2.0, 40.0); 
    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("Figure Sample", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    // plot 1
    let range_1 = Range::new(x_min, x_max, 0.01);
    chart.draw_series(
        LineSeries::new(range_1.map(|x| (x, x * x)),&RED)
            )?
        .label("plot 1: y = x^2")
        .legend(|(x,y)| PathElement::new(vec![(x,y), (x + 20, y)], &RED));

    // plot 2
    let range_2 = Range::new(x_min, x_max, 0.01);
    chart.draw_series(
        LineSeries::new(range_2.map(|x| (x, 2.0 * x.sin() + 5.0)), &BLUE)
                     )?
        .label("plot 2: y = 2sinx(x) + 5")
        .legend(|(x,y)| PathElement::new(vec![(x,y), (x + 20, y)], &BLUE));

    // plot 3
    let range_3 = Range::new(x_min, x_max, 0.01);
    chart.draw_series(
        LineSeries::new(range_3.map(|x| (x, 1.5_f64.powf(x))), &GREEN)
                     )?
        .label("plot 3: y = (1.5)^x")
        .legend(|(x,y)| PathElement::new(vec![(x,y), (x + 20, y)], &GREEN));

    // legendの設定
    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    Ok(())
}
