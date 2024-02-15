#![warn(clippy::all, rust_2018_idioms)]
#![allow(non_snake_case)]
mod app;
pub mod graph;
mod data;

pub use app::TemplateApp;
pub use data::Buff;
pub use data::BUFF_SIZE;
