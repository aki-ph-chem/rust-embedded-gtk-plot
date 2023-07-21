use std::error;
use gtk;
use gtk::prelude::*;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;

struct QFunc {
    a: f32,
    b: f32,
    c: f32,
}

impl QFunc {
    fn new(a: f32, b: f32, c: f32) -> QFunc {
        QFunc{a, b, c}
    }

    // f(x) = a(x - b)^2 + c
    fn q_func(&self, x: f32) -> f32 {
        self.a * (x - self.b).powi(2) + self.c
    }

    // backendに対してplot
    fn plot_q_func(&self, backend: CairoBackend) -> Result<(), Box<dyn error::Error>> {
        let root = backend.into_drawing_area();
        root.fill(&WHITE)?;

        let formula_caption = format!("y = {}(x - {})^2 + {}",self.a , self.b, self.c);
        let mut chart = ChartBuilder::on(&root)
            .caption(&formula_caption, ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -1.4f32..1.6f32)?;
        chart.configure_mesh().draw()?;
        chart
            .draw_series(LineSeries::new(
                    (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, self.q_func(x))),
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

        // f(x) =a(x - b)^2 + cの(a,b,c)を与える
        let q_func = QFunc::new(1.1, 0.1, 0.2);
        q_func.plot_q_func(backend).unwrap();
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
