use std::error;
use gtk;
use gtk::prelude::*;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;

fn q_func(x: f32) -> f32 {
    x.powi(2)
}

// backendに対してplot
fn plot_q_func(backend: CairoBackend) -> Result<(), Box<dyn error::Error>> {
        let root = backend.into_drawing_area();
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
                    (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, q_func(x))),
                    &RED,
                    )).unwrap()
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

// gtk windowの中にplotを埋め込む
fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/q_func.ui");
    let builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window").expect("Error: window");
    window.set_application(Some(app));

    let drawing_area: gtk::DrawingArea = builder.object("draw")
        .expect("Error: draw");

    drawing_area.connect_draw(move |widget, cr| {
        let width = widget.allocated_width();
        let height = widget.allocated_height();

        let backend = CairoBackend::new(cr, (width as u32, height as u32))
            .unwrap();

        plot_q_func(backend).unwrap();
        Inhibit(false)
    });

    window.show_all();
} 

fn main() -> Result<(), Box<dyn error::Error>> {
    let application = gtk::Application::new(
        Some("gtk.plot"), Default::default());

    application.connect_activate(build_ui);
    application.run();

    Ok(())
}
