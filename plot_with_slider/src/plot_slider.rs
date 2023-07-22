use std::error;
use gtk;
use gtk::prelude::*;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug,Clone,Copy)]
struct QFunc {
    a: f64,
    b: f64,
    c: f64,
}

impl QFunc {
    // f(x) = a(x - b)^2 + c
    fn q_func(&self, x: f64) -> f64 {
        self.a * (x - self.b).powi(2) + self.c
    }

    // backendに対してplot
    fn plot_q_func(&self, backend: CairoBackend) -> Result<(), Box<dyn error::Error>> {
        let root = backend.into_drawing_area();
        root.fill(&WHITE)?;

        let formula_caption = "y = a(x - b)^2 + c";
        let mut chart = ChartBuilder::on(&root)
            .caption(formula_caption, ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-2f64..2f64, -4.2f64..4.2f64)?;
        chart.configure_mesh().draw()?;
        chart
            .draw_series(LineSeries::new(
                    (-100..=100).map(|x| x as f64 / 50.0).map(|x| (x, self.q_func(x))),
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
    let ui = include_str!("ui/plot_slider.ui");
    let builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window").expect("Error: window");
    window.set_application(Some(app));

    // パラメータを調節する
    let a_control: gtk::Scale = builder.object("a_control").expect("Error: a_control"); 
    let b_control: gtk::Scale = builder.object("b_control").expect("Error: a_control"); 
    let c_control: gtk::Scale = builder.object("c_control").expect("Error: a_control"); 
    let drawing_area: gtk::DrawingArea = builder.object("area_plot")
        .expect("Error: draw");
    // パラメータの初期値を取得
    let app_state = Rc::new(RefCell::new(QFunc{
        a: a_control.value() + 1.0,
        b: b_control.value(),
        c: c_control.value(),
    }));
    // plotの描画
    let app_state_clone = app_state.clone();
    drawing_area.connect_draw(move |widget, cr| {
        let width = widget.allocated_width();
        let height = widget.allocated_height();
        let backend = CairoBackend::new(cr, (width as u32, height as u32))
            .unwrap();
        let state = app_state_clone.borrow().clone();
        state.plot_q_func(backend).unwrap();
        Inhibit(false)
    });

    // パラメータが変化した時の再描画処理
    let handle_change = 
        |control: &gtk::Scale, action: Box<dyn Fn(&mut QFunc) -> &mut f64 + 'static>| {
            let app_state_state = app_state.clone(); 
            let drawing_area = drawing_area.clone();
            control.connect_value_changed(move |target| {
                let mut state = app_state_state.borrow_mut();
                *action(&mut *state) = target.value();
                drawing_area.queue_draw();
            });
        };
    handle_change(&a_control, Box::new(|s| &mut s.a));
    handle_change(&b_control, Box::new(|s| &mut s.b));
    handle_change(&c_control, Box::new(|s| &mut s.c));

    window.show_all();
} 

fn main(){
    let application = gtk::Application::new(
        Some("gtk.plot"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
