use gtk;
use gtk::prelude::*;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;
use std::error;

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

fn sin_4x(x: f32) -> f32 {
    (4.0 * x).sin()
}

struct Range {
    x_ini: f32,
    x_fin: f32,
    step: f32,
}

impl Iterator for Range {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x_ini <= self.x_fin {
            let value = self.x_ini;
            self.x_ini += self.step;
            Some(value)
        } else {
            None
        }
    }
}


// backendに対してplot
fn plot_q_func(backend: CairoBackend) -> Result<(), Box<dyn error::Error>> {
        let root = backend.into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("y=sin(4x)", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-TWO_PI..TWO_PI, -1.3f32..1.3f32)?;
        chart.configure_mesh().draw()?;

        let range = Range{x_ini: -TWO_PI, x_fin: TWO_PI, step: 0.01};
        chart
            .draw_series(LineSeries::new(
                    range.map(|x| (x, sin_4x(x))),
                    &RED,
                    )).unwrap()
            .label("y = sin(4x)")
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

    drawing_area.connect_button_press_event(move |_,event| {
        if event.button() == 1 {
            let (x, y) = event.position();
            println!("(x, y) = ({}, {})", x, y);
        }
        Inhibit(false)
    });

    window.show_all();
} 

fn main(){
    let application = gtk::Application::new(
        Some("gtk.plot"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
