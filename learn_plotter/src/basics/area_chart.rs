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

// Area Chart 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut result_path = PathBuf::new();  
    result_path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
    result_path.push("target");
    result_path.push("figure");
    mkdir(result_path.to_str().unwrap())?;
    result_path.push("area_chart.png");

    let drawing_area = BitMapBackend::new(result_path.to_str().unwrap(), (600, 400))
        .into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Area Chart", ("sana-serif", 40))
        .build_cartesian_2d(0..10, 0..50)?;
    chart.configure_mesh().draw()?;

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 29, 0, 21];
    chart
        .draw_series(
            AreaSeries::new(
                //横軸が0 以上の値を表示する
                (0..).zip(data.iter().map(|x| *x)),
                0,
                &RED.mix(0.2)).border_style(&RED)
                )?;

    Ok(())
}
