//! MoFA Studio - Main entry point

mod app;
mod config;

fn main() {
    env_logger::init();
    log::info!("Starting MoFA Studio");
    app::app_main();
}
