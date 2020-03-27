use ggez;
use nalgebra;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::convert::Into;
use std::convert::TryFrom;

use crate::sub_modules::config::Resolution;
use crate::sub_modules::tracer::SceneState;

pub struct WindowState<'lifetime> {
    pub scene: &'lifetime SceneState,
    pub res: Resolution,
    frame: Vec<u8>,
}

pub fn run(state: &mut WindowState) -> GameResult {
    let window_setup = ggez::conf::WindowSetup::default().title("Bad Ray Tracer");
    let window_mode = ggez::conf::WindowMode::default()
        .dimensions(state.res.width.into(), state.res.height.into());

    let cb = ggez::ContextBuilder::new("rust-tracer", "ZephroC")
        .window_setup(window_setup)
        .window_mode(window_mode);
    let (ctx, event_loop) = &mut cb.build()?;

    event::run(ctx, event_loop, state)
}

impl WindowState<'_> {
    pub fn new(res: Resolution, scene: &SceneState) -> WindowState {
        let byte_stride = 4;
        let pixels: u32 = res.width as u32 * res.height as u32;

        let s = WindowState {
            res: res,
            scene: scene,
            frame: vec![255; usize::try_from(pixels * byte_stride).unwrap()],
        };
        s
    }
}

//I'm not sure the event handler should know about the geometry
impl event::EventHandler for WindowState<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        println!("FPS: {}", ggez::timer::fps(ctx));


        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        let dst = nalgebra::Point2::new(0.0, 0.0);
        self.scene.rasterise(&mut self.frame, &self.res);
        let image =
            graphics::Image::from_rgba8(ctx, self.res.width, self.res.height, &self.frame).unwrap();

        graphics::draw(ctx, &image, graphics::DrawParam::new().dest(dst))?;
        graphics::present(ctx)?;
        ggez::timer::yield_now();

        Ok(())
    }
}
