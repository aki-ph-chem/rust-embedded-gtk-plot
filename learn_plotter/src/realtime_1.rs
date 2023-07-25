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

// real time: gif画像を生成する
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut result_path = PathBuf::new();  
    result_path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
    result_path.push("target");
    result_path.push("figure");
    mkdir(result_path.to_str().unwrap())?;
    result_path.push("real_time.gjf");

    let drawing_area = BitMapBackend::gif(
        result_path.to_str().unwrap(), 
        (600, 400),
        500)? // 500 ms(0.5 s)
        .into_drawing_area();

    // 10 ~ 0の間の数字を表示するアニメーション 
    for i in 0..=10 {
        drawing_area.fill(&WHITE)?;
        drawing_area.draw(
            &Text::new(
                format!("{}", 10 - i),
                (100, 20),
                ("sans-serif", 80)
                      )
                         )?;
        drawing_area.present()?;
    }

    Ok(())
}
