#![warn(clippy::all, rust_2018_idioms)]
#![allow(non_snake_case)]
mod app;
mod data;
mod loader;
pub mod graph;
mod config_info;
pub use app::TelemApp;
pub use data::Buff;
pub use data::BUFF_SIZE;

mod view;
