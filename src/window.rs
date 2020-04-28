use ggez;
use nalgebra;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use std::convert::Into;

use crate::Resolution;
use super::buffer::FrameBuffer;


struct WindowState<'lifetime> {
    pub res: Resolution,
    frame: &'lifetime FrameBuffer
}

pub fn run(res:Resolution,frame:&FrameBuffer) -> GameResult {
    let mut state =  WindowState::new(res, frame);
    let window_setup = ggez::conf::WindowSetup::default().title("Bad Ray Tracer");
    let window_mode = ggez::conf::WindowMode::default()
        .dimensions(state.res.width.into(), state.res.height.into());

    let cb = ggez::ContextBuilder::new("rust-tracer", "ZephroC")
        .window_setup(window_setup)
        .window_mode(window_mode);
    let (ctx, event_loop) = &mut cb.build()?;

    event::run(ctx, event_loop, &mut state)
}

impl WindowState<'_> {
    fn new(res: Resolution, frame: &FrameBuffer) -> WindowState {
        WindowState {
            res,
            frame
        }
    }
}

//Just event handling for the window.
impl event::EventHandler for WindowState<'_> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        graphics::clear(_ctx, [0.0, 0.0, 0.0, 1.0].into());

        println!("Window Time: {:?}", ggez::timer::delta(_ctx));

        let dst = nalgebra::Point2::new(0.0, 0.0);
        let image =
            graphics::Image::from_rgba8(_ctx, self.res.width, self.res.height, &self.frame.read_buffer().read().unwrap()).unwrap();

        graphics::draw(_ctx, &image, graphics::DrawParam::new().dest(dst))?;
        graphics::present(_ctx)?;

        ggez::timer::yield_now();

        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        false
    }
}
