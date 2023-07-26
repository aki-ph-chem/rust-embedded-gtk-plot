use std::error;
use gtk;
use gtk::glib;
use gtk::prelude::*;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;
use std::rc::Rc;
use std::cell::RefCell;

// 関数値を計算する区間を生成
struct Range {
    x_current: f64,
    x_fin: f64,
    step: f64,
}

impl Iterator for Range {
    type Item = f64;

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

// plotの範囲
#[derive(Debug,Clone,Copy)]
struct PlotRange {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
    coeff_a: f64,
    coeff_b: f64,
}

#[derive(Debug,Clone, Copy)]
struct AppState {
    pitch: f64,
    yaw: f64,
    scale: f64,
    plot_range: PlotRange, 
}

impl AppState {

    fn q_func(&self, x: f64, y: f64) -> f64 {
        self.plot_range.coeff_a * x.powi(2) + self.plot_range.coeff_b * y.powi(2)
    }

    // backendに対してplot
    fn plot_q_func(&self, backend: CairoBackend) -> Result<(), Box<dyn error::Error>> {
        let root = backend.into_drawing_area();
        root.fill(&WHITE)?;

        let (x_min, x_max, y_min, y_max, z_min, z_max) 
            = (self.plot_range.x_min, self.plot_range.x_max,  // x 
               self.plot_range.y_min, self.plot_range.y_max,  // y
               self.plot_range.z_min, self.plot_range.z_max); // z

        let formula_caption = "z = ax^2 + by^2";
        let mut chart = ChartBuilder::on(&root)
            .margin(3)
            .caption(formula_caption, ("sans-serif", 25))
            //.build_cartesian_3d(x_min..x_max, y_min..y_max, z_min..z_max)?;
            .build_cartesian_3d(x_min..x_max, z_min..z_max, y_min..y_max)?;

        // 視点を変更する
        chart.with_projection(|mut pb| {
            pb.pitch = self.pitch;
            pb.yaw = self.yaw;
            pb.scale = self.scale;
            pb.into_matrix()
        });
        chart.configure_axes().draw()?;

        // plotの描画処理
        let range_x = Range{x_current: x_min * 0.5, x_fin: x_max * 0.5, step: 0.1};
        let range_y = Range{x_current: y_min * 0.5, x_fin: y_max * 0.5, step: 0.1};
        chart.draw_series(
            SurfaceSeries::xoz(
                range_x,
                range_y,
                |x: f64, y: f64| self.q_func(x,y) ).style(&BLUE.mix(0.2))
            )?;

        root.present()?;
        Ok(())
    }
}

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/q_func_2.ui");
    let builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window").expect("Error: window");
    window.set_application(Some(app));

    // スライダー、DrawingArea
    let pitch_control: gtk::Scale = builder.object("scale_pitch").expect("Error: scale_pitch"); 
    let yaw_control: gtk::Scale = builder.object("scale_yaw").expect("Error: scale_yaw"); 
    let scale_control: gtk::Scale = builder.object("scale_scale").expect("Error: scale_scale"); 
    let drawing_area: gtk::DrawingArea = builder.object("area_plot").expect("Error: area_plot");

    // plot範囲を変える用のEtnry,button 
    let entry_x_min: gtk::Entry = builder.object("entry_x_min").expect("Error: entry_x_min");
    let entry_x_max: gtk::Entry = builder.object("entry_x_max").expect("Error: entry_x_max");
    let entry_y_min: gtk::Entry = builder.object("entry_y_min").expect("Error: entry_y_min");
    let entry_y_max: gtk::Entry = builder.object("entry_y_max").expect("Error: entry_y_max");
    let entry_z_min: gtk::Entry = builder.object("entry_z_min").expect("Error: entry_z_min");
    let entry_z_max: gtk::Entry = builder.object("entry_z_max").expect("Error: entry_z_max");
    let button_redraw: gtk::Button = builder.object("button_redraw").expect("Error: button_redraw");

    let entry_a: gtk::Entry = builder.object("entry_a").expect("Error: entry_a");
    let entry_b: gtk::Entry = builder.object("entry_b").expect("Error: entry_b");

    // パラメータを初期化する
    let app_state = Rc::new(RefCell::new(AppState{
        pitch: 0.3,
        yaw: 0.5,
        scale: 0.8,
        plot_range: PlotRange {
            x_min: -3.0,
            x_max: 3.0,
            y_min: -3.0,
            y_max: 3.0,
            z_min: 0.0,
            z_max: 10.0,
            coeff_a: 1.0,
            coeff_b: 1.0,
        }
    }));
    // Entryに初期値を与える
    entry_x_min.set_text(&app_state.borrow().plot_range.x_min.to_string());
    entry_x_max.set_text(&app_state.borrow().plot_range.x_max.to_string());
    entry_y_min.set_text(&app_state.borrow().plot_range.y_min.to_string());
    entry_y_max.set_text(&app_state.borrow().plot_range.y_max.to_string());
    entry_z_min.set_text(&app_state.borrow().plot_range.z_min.to_string());
    entry_z_max.set_text(&app_state.borrow().plot_range.z_max.to_string());
    entry_a.set_text(&app_state.borrow().plot_range.coeff_a.to_string());
    entry_b.set_text(&app_state.borrow().plot_range.coeff_b.to_string());

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

    // x,yの範囲をGtkEntryから入力した数値で変更
    let handle_plot_range =
        |control: &gtk::Entry, action: Box<dyn Fn(&mut AppState) -> &mut f64 + 'static>| {
            button_redraw.connect_clicked(glib::clone!(@weak control, @weak drawing_area, @weak app_state => move |_| {
                let mut state = app_state.borrow_mut();
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
    handle_plot_range(&entry_x_min, Box::new(|s| &mut s.plot_range.x_min));
    handle_plot_range(&entry_x_max, Box::new(|s| &mut s.plot_range.x_max));
    handle_plot_range(&entry_y_min, Box::new(|s| &mut s.plot_range.y_min));
    handle_plot_range(&entry_y_max, Box::new(|s| &mut s.plot_range.y_max));
    handle_plot_range(&entry_z_min, Box::new(|s| &mut s.plot_range.z_min));
    handle_plot_range(&entry_z_max, Box::new(|s| &mut s.plot_range.z_max));
    handle_plot_range(&entry_a, Box::new(|s| &mut s.plot_range.coeff_a));
    handle_plot_range(&entry_b, Box::new(|s| &mut s.plot_range.coeff_b));

    let handle_perspective = 
        |control: &gtk::Scale, action: Box<dyn Fn(&mut AppState) -> &mut f64 + 'static>| {
            control.connect_value_changed(glib::clone!(@weak app_state, @weak drawing_area => move |target| {
                let mut state = app_state.borrow_mut();
                *action(&mut *state) = target.value();
                drawing_area.queue_draw();
            }));
        };
    handle_perspective(&pitch_control, Box::new(|s| &mut s.pitch));
    handle_perspective(&yaw_control, Box::new(|s| &mut s.yaw));
    handle_perspective(&scale_control, Box::new(|s| &mut s.scale));


    window.show_all();
}


fn main() {
    let application = gtk::Application::new(
        Some("gtk.plot3d"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
