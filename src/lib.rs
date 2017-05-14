#![crate_name="orbclient"]
#![crate_type="lib"]
#![feature(asm)]
#![feature(collections)]
#![feature(const_fn)]
#![no_std]

#![deny(warnings)]

extern crate collections;

pub static FONT: &'static [u8] = include_bytes!("../res/unifont.font");

pub use color::Color;
pub use event::*;
pub use graphicspath::GraphicsPath;
pub use renderer::Renderer;

pub mod color;
pub mod event;
pub mod graphicspath;
pub mod renderer;

#[derive(Clone, Copy, Debug)]
pub enum WindowFlag {
    Async,
    Resizable,
    Unclosable
}
