use std::error;
use gtk;
use gtk::glib;
use gtk::prelude::*;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug,Clone,Copy)]
struct PlotRange {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

#[derive(Debug,Clone,Copy)]
struct QFunc {
    a: f64,
    b: f64,
    c: f64,
    range: PlotRange,
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
        let (x_min, x_max, y_min, y_max) = (self.range.x_min, self.range.x_max, self.range.y_min, self.range.y_max);

        let formula_caption = "y = a(x - b)^2 + c";
        let mut chart = ChartBuilder::on(&root)
            .caption(formula_caption, ("sans-serif", 25).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
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
    let ui = include_str!("ui/plot_slider_range.ui");
    let builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window").expect("Error: window");
    window.set_application(Some(app));

    // スライダー、DrawingArea
    let a_control: gtk::Scale = builder.object("a_control").expect("Error: a_control"); 
    let b_control: gtk::Scale = builder.object("b_control").expect("Error: a_control"); 
    let c_control: gtk::Scale = builder.object("c_control").expect("Error: a_control"); 
    let drawing_area: gtk::DrawingArea = builder.object("area_plot").expect("Error: draw");

    // plot範囲を変える用のEtnry,button 
    let entry_x_min: gtk::Entry = builder.object("entry_x_min").expect("Error: entry_x_min");
    let entry_x_max: gtk::Entry = builder.object("entry_x_max").expect("Error: entry_x_max");
    let entry_y_min: gtk::Entry = builder.object("entry_y_min").expect("Error: entry_y_min");
    let entry_y_max: gtk::Entry = builder.object("entry_y_max").expect("Error: entry_y_max");
    let button_redraw: gtk::Button = builder.object("button_redraw").expect("Error: button_redraw");

    // パラメータを調節する
    // パラメータの初期値を取得
    let app_state = Rc::new(RefCell::new(QFunc{
        a: a_control.value() + 1.0,
        b: b_control.value(),
        c: c_control.value(),
        range: PlotRange{
           x_min: -2.0, 
           x_max: 2.0,
           y_min: -4.2, 
           y_max: 4.2},
    }));
    // Entryに初期値を設定する
    entry_x_min.set_text(&app_state.borrow().range.x_min.to_string());
    entry_x_max.set_text(&app_state.borrow().range.x_max.to_string());
    entry_y_min.set_text(&app_state.borrow().range.y_min.to_string());
    entry_y_max.set_text(&app_state.borrow().range.y_max.to_string());

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
            let app_state_clone = app_state.clone(); 
            let drawing_area = drawing_area.clone();
            control.connect_value_changed(move |target| {
                let mut state = app_state_clone.borrow_mut();
                *action(&mut *state) = target.value();
                drawing_area.queue_draw();
            });
        };
    handle_change(&a_control, Box::new(|s| &mut s.a));
    handle_change(&b_control, Box::new(|s| &mut s.b));
    handle_change(&c_control, Box::new(|s| &mut s.c));

    // x,yの範囲をGtkEntryから入力した数値で変更
    let handle_range =
        |control: &gtk::Entry, action: Box<dyn Fn(&mut QFunc) -> &mut f64 + 'static>| {
            let app_state_clone = app_state.clone(); 
            let drawing_area = drawing_area.clone();

            button_redraw.connect_clicked(glib::clone!(@weak control => move |_| {
                let mut state = app_state_clone.borrow_mut();
                match control.text().parse::<f64>() {
                    Ok(value) =>{
                        *action(&mut *state) = value;
                        drawing_area.queue_draw();
                    },
                    Err(error) => {
                        eprintln!("Error {}", error);
                    }
                };

            }));
        };
    handle_range(&entry_x_min, Box::new(|s| &mut s.range.x_min));
    handle_range(&entry_x_max, Box::new(|s| &mut s.range.x_max));
    handle_range(&entry_y_min, Box::new(|s| &mut s.range.y_min));
    handle_range(&entry_y_max, Box::new(|s| &mut s.range.y_max));

    window.show_all();
} 

fn main(){
    let application = gtk::Application::new(
        Some("gtk.plot"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
