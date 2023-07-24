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

// 簡単なplot 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut result_path = PathBuf::new();  
    result_path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
    result_path.push("target");
    result_path.push("figure");
    mkdir(result_path.to_str().unwrap())?;
    result_path.push("line_series.png");

    let drawing_area = BitMapBackend::new(result_path.to_str().unwrap(), (600, 400))
        .into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("Figure Sample", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(-10..10, 0..110)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        LineSeries::new((-10..=10).map(|x| (x, x * x)),&RED)
            )?;

    Ok(())
}
