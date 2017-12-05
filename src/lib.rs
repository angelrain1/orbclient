#![crate_name="orbclient"]
#![crate_type="lib"]
#![feature(alloc)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]

#![deny(warnings)]

extern crate alloc;

pub static FONT: &'static [u8] = include_bytes!("../res/unifont.font");

pub use color::Color;
pub use event::*;
pub use renderer::Renderer;

pub mod color;
pub mod event;
pub mod renderer;

#[derive(Clone, Copy, Debug)]
pub enum WindowFlag {
    Async,
    Back,
    Front,
    Resizable,
    Unclosable
}
