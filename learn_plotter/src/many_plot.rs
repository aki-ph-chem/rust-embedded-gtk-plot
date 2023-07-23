use std::error;
use gtk;
use gtk::prelude::*;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;

fn sin(x: f32) -> f32 {
    x.sin()
}

fn sqrt(x: f32) -> f32 {
    if x > 0.0 {
        x.sqrt()
    } else {
        0.0
    }
}

fn q_func(x: f32) -> f32 {
    x.powi(2)
}

struct Range {
    x_current: f32,
    x_fin: f32,
    step: f32,
}

impl Range {
    fn new(x_ini: f32, x_fin: f32, step: f32) -> Range {
        Range{x_current: x_ini, x_fin, step}
    }
}

impl Iterator for Range {
    type Item = f32;

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
        const TWO_PI: f32 = std::f32::consts::PI;
        let plot_range = PlotRange::new(-TWO_PI, TWO_PI, -2.0, 10.0);

        let mut chart = ChartBuilder::on(&root)
            .caption("plots", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(plot_range.x_min..plot_range.x_max, plot_range.y_min..plot_range.y_max)?;
        chart.configure_mesh().draw()?;

        // plot 1 sinx(x)
        let range_1 = Range::new(-TWO_PI, TWO_PI, 0.01);
        chart
            .draw_series(LineSeries::new(
                    range_1.map(|x| (x, sin(x))),
                    &RED,
                    )).unwrap()
            .label("sin(x)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        // plot 2 sqrt(x)
        let range_2 = Range::new(0.0, TWO_PI, 0.01);
        chart
            .draw_series(LineSeries::new(
                    range_2.map(|x| (x, sqrt(x))),
                    &BLACK,
                    )).unwrap()
            .label("sqrt(x)")
            .legend(|(x,y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

        // plot 3 q_func(x)
        let range_3 = Range::new(-TWO_PI, TWO_PI, 0.01);
        chart
            .draw_series(LineSeries::new(
                    range_3.map(|x| (x, q_func(x))),
                    &BLUE,
                    )).unwrap()
            .label("x^2")
            .legend(|(x,y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

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
