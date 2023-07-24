use plotters::prelude::*;
use std::path::PathBuf;
use std::env; use std::fs;

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
    result_path.push("scatter.png");

    let drawing_area = BitMapBackend::new(result_path.to_str().unwrap(), (600, 400))
        .into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let (x_min, x_max, y_min, y_max) = (-10, 50, -10, 50); 
    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("scatter", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    // plot 1
    chart.draw_series(
        DATA1.iter().map(|p| TriangleMarker::new(*p, 5, &BLUE)),
            )?
        .label("plot 1: DATA1")
        .legend(|(x,y)| PathElement::new(vec![(x,y), (x + 20, y)], &BLUE));

    // plot 2
    chart.draw_series(
        DATA2.iter().map(|p| Circle::new(*p, 5, &RED))
                     )?
        .label("plot 2: DATA2")
        .legend(|(x,y)| PathElement::new(vec![(x,y), (x + 20, y)], &RED));


    // legendの設定
    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    Ok(())
}

const DATA1: [(i32, i32); 30] =  [(-3, 1), (-2, 3), (4, 2), (3, 0), (6, -5), (3, 11), (6, 0), (2, 14), (3, 9), (14, 7), (8, 11), (10, 16), (7, 15), (13, 8), (17, 14), (13, 17), (19, 11), (18, 8), (15, 8), (23, 23), (15, 20), (22, 23), (22, 21), (21, 30), (19, 28), (22, 23), (30, 23), (26, 35), (33, 19), (26, 19)];
const DATA2: [(i32, i32); 30] = [(1, 22), (0, 22), (1, 20), (2, 24), (4, 26), (6, 24), (5, 27), (6, 27), (7, 27), (8, 30), (10, 30), (10, 33), (12, 34), (13, 31), (15, 35), (14, 33), (17, 36), (16, 35), (17, 39), (19, 38), (21, 38), (22, 39), (23, 43), (24, 44), (24, 46), (26, 47), (27, 48), (26, 49), (28, 47), (28, 50)];
