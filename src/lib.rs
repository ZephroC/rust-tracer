pub mod tracer;
pub mod config;
pub mod window;

#[derive(Debug, Copy, Clone)]
pub struct Resolution {
    pub width: u16,
    pub height: u16,
}