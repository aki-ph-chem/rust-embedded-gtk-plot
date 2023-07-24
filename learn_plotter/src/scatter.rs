use std::error;
use gtk;
use gtk::prelude::*;
use plotters;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;

    const DATA1: [(f32, f32); 10] = [
(1.1, 2.22), (0.6, 1.2), (0.1, -2.15),
(3.1, 0.82), (1.6, -1.2), (-0.2, -2.15),
(2.8, 2.12), (-0.6, 1.2), (0.1, -2.15), (-2.1, -2.7)
    ];

    const DATA2: [(f32, f32); 10] = [
(1.1, 2.22), (0.1, 1.2), (0.1, 2.15),
(3.1, 1.22), (0.6, -1.2), (-0.1, -2.15),
(3.1, 1.22), (0.6, -1.2), (-0.1, -2.15), (1.2, 2.7)
    ];

struct PlotRange {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

impl PlotRange {
    fn new(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> PlotRange {
        PlotRange{x_min, x_max, y_min, y_max}
    }
}

// backendに対してplot
fn plot_q_func(backend: CairoBackend) -> Result<(), Box<dyn error::Error>> {
        let root = backend.into_drawing_area();
        root.fill(&WHITE)?;

        // 範囲を設定
        let plot_range = PlotRange::new(-3.0, 3.0, -3.0, 3.0);

        let mut chart = ChartBuilder::on(&root)
            .caption("scatters", ("sans-serif", 20).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(plot_range.x_min..plot_range.x_max, plot_range.y_min..plot_range.y_max)?;
        chart.configure_mesh().draw()?;

        // 散布図 1をplot
        chart
            .draw_series(
                DATA1.iter()
                .map(|point| Circle::new(*point, 5, &RED))
                        ).unwrap()
            .label("plot 1")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        // 散布図 2をplot
        chart
            .draw_series(
                    DATA2.iter()
                    .map(|point| TriangleMarker::new(*point, 5, &BLUE))
                    ).unwrap()
            .label("plot 2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
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
    let ui = include_str!("ui/plot.ui");
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
