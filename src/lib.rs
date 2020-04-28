pub mod tracer;
pub mod config;
pub mod window;
pub mod buffer;

#[derive(Debug, Copy, Clone)]
pub struct Resolution {
    pub width: u16,
    pub height: u16,
}