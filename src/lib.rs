#![warn(clippy::all, rust_2018_idioms)]
#![allow(non_snake_case)]
mod app;
mod data;
mod loader;
mod config_info;
mod config_window;
mod graph;
mod view;
pub use app::TelemApp;
pub use data::Buff;
pub use data::BUFF_SIZE;
