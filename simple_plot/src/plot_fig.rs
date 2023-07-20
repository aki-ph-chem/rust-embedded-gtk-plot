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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut result_path = PathBuf::new();  
    result_path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
    result_path.push("target");
    result_path.push("figure");
    mkdir(result_path.to_str().unwrap())?;

    result_path.push("plot_quadratic_func.png");
    let root = BitMapBackend::new(result_path.to_str().unwrap(), (640, 480))
        .into_drawing_area();

    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED,
                ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
