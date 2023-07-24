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

// とりあえず直線を描画してみる
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut result_path = PathBuf::new();  
    result_path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
    result_path.push("target");
    result_path.push("figure");
    mkdir(result_path.to_str().unwrap())?;
    result_path.push("line.png");

    let drawing_area = BitMapBackend::new(result_path.to_str().unwrap(), (600, 400))
        .into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0..100, 0..100)?;

    chart
        .draw_series(LineSeries::new(
                (0..100).map(|x| (x, 100 - x)), &RED),
                )?;

    Ok(())
}
