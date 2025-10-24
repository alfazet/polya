mod app;
mod calc;
mod constants;
mod dialog;
mod polygon;
mod render;
mod state;
mod vertex;

use crate::app::App;

fn main() {
    simple_logging::log_to_stderr(log::LevelFilter::Warn);
    let program_name = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        &program_name,
        native_options,
        Box::new(|_| Ok(Box::new(App::default()))),
    );
}
