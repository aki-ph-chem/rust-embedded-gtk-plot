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

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/q_func.ui");
    let builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window").expect("Error: window");
    window.set_application(Some(app));

    window.show_all();
}


fn main() {
    let application = gtk::Application::new(
        Some("gtk.plot3d"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
