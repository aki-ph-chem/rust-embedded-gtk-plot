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

fn main() -> Result<(),Box<dyn error::Error>> {
    let mut result_path = path::PathBuf::new();  
    result_path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
    result_path.push("target");
    result_path.push("figure");
    mkdir(result_path.to_str().unwrap())?;
    result_path.push("scatter_3d.png");
    
    let root = BitMapBackend::new(result_path.to_str().unwrap(), (640, 480))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let (x_min, x_max, y_min, y_max, z_min, z_max) 
        = (-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Scatter 3D ", ("sans-serif", 40))
        .build_cartesian_3d(x_min..x_max, y_min..y_max, z_min..z_max)?;

    chart.configure_axes().draw()?;

    chart.draw_series(
        DATA1.iter().map(|p| Circle::new(*p, 5, &RED))
                    )?;

    Ok(())
}

const DATA1: [(f64, f64, f64); 8] = [
(0.0,0.0,0.0), (0.0, 0.5, 0.0),(0.5, 0.0, 0.0), (0.1, 0.0, 0.5),
(0.1,-0.3,0.0), (0.1, -0.5, 0.3),(-0.5, 0.1, 0.2), (-0.2, 0.6, 0.5)
];
