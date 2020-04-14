use ggez;
use nalgebra;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use std::convert::Into;
use std::convert::TryFrom;

use super::tracer::SceneState;
use crate::Resolution;


struct WindowState<'lifetime> {
    pub scene: &'lifetime SceneState,
    pub res: Resolution,
    frame: Vec<u8>,
}

pub fn run(res:Resolution, scene:&SceneState) -> GameResult {
    let mut state =  WindowState::new(res, scene);
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
    fn new(res: Resolution, scene: &SceneState) -> WindowState {
        let byte_stride = 4;
        let pixels: u32 = res.width as u32 * res.height as u32;
        WindowState {
            res,
            scene,
            frame: vec![255; usize::try_from(pixels * byte_stride).unwrap()],
        }
    }
}

//I'm not sure the event handler should know about the geometry
impl event::EventHandler for WindowState<'_> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        false
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        graphics::clear(_ctx, [0.0, 0.0, 0.0, 1.0].into());

        println!("Frame Time: {:?}", ggez::timer::delta(_ctx));

        let dst = nalgebra::Point2::new(0.0, 0.0);
        self.scene.rasterise(&mut self.frame, &self.res);
        let image =
            graphics::Image::from_rgba8(_ctx, self.res.width, self.res.height, &self.frame).unwrap();

        graphics::draw(_ctx, &image, graphics::DrawParam::new().dest(dst))?;
        graphics::present(_ctx)?;


        ggez::timer::yield_now();

        Ok(())
    }
}
