use plotters::prelude::*;
use std::error;
use std::{env,fs};
use std::path;

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

fn main() -> Result<(),Box<dyn error::Error>> {
    let mut result_path = path::PathBuf::new();  
    result_path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
    result_path.push("target");
    result_path.push("figure");
    mkdir(result_path.to_str().unwrap())?;
    result_path.push("line_3d_perspective.png");
    
    let root = BitMapBackend::new(result_path.to_str().unwrap(), (640, 480))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let (x_min, x_max, y_min, y_max, z_min, z_max) 
        = (-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("line 3D ", ("sans-serif", 40))
        .build_cartesian_3d(x_min..x_max, y_min..y_max, z_min..z_max)?;

    // 視点を変更する
    chart.with_projection(|mut pb| {
        pb.pitch = 1.2;
        pb.yaw = 0.5;
        pb.scale = 0.6;
        pb.into_matrix()
    });

    chart.configure_axes().draw()?;
    let range = Range::new(-1.0, 1.0, 0.01);
    chart.draw_series(LineSeries::new(
            range.map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
            &RED
                                     ))?;

    Ok(())
}
