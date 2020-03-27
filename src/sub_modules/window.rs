
use ggez;
use nalgebra;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::convert::Into;
use std::convert::TryFrom;


use crate::sub_modules::config::{Resolution, Config};
use crate::sub_modules::tracer::{Camera, Intersects, Frustrum, Sphere, RGBA};
use crate::sub_modules::tracer;

//realistically this needs separating from the tracer code but mod is confusing me right now
pub struct MainState {
    pub resolution: Resolution,
    pub frame: Vec<u8>,
    pub scene:Vec<Box<Intersects>>,
    pub camera:Camera
}


pub fn run(config:Config) -> GameResult {
    let window_setup = ggez::conf::WindowSetup::default().title("Bad Ray Tracer");
    let window_mode = ggez::conf::WindowMode::default().dimensions(config.resolution.width.into(), config.resolution.height.into());

    let cb = ggez::ContextBuilder::new("helloworld", "ggez")
        .window_setup(window_setup )
        .window_mode(window_mode);
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(config.resolution)?;
    event::run(ctx, event_loop, state)
}

impl MainState {
    fn new(res:Resolution, ) -> GameResult<MainState> {
        let byte_stride = 4;
        let pixels:u32 = res.width as u32 * res.height as u32;
        let array = vec![255; usize::try_from(pixels * byte_stride).unwrap()];

        let camera = Camera{
            pos: nalgebra::Vector3::new(0.0,0.0,0.0),
            dir: nalgebra::Vector3::new(1.0,0.0,0.0),
            fov: 90.0,
            frustrum: Frustrum{
                height: 9.0,
                width: 16.0,
                distance: 8.0
            }
        };
        let s = MainState { resolution: res,
            frame: array,
            scene: vec![Box::new(Sphere{
            colour: RGBA{
                r:255,
                g:255,
                b:0,
                a:255
            },
            radius: 8.0,
            pos: nalgebra::Vector3::new(6.0,-2.0,24.0)})],
            camera
        } ;
        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let dst = nalgebra::Point2::new(0.0,0.0);
        tracer::rasterise(self);
        let image = graphics::Image::from_rgba8(ctx, self.resolution.width,self.resolution.height, &self.frame).unwrap();
        graphics::draw(
            ctx,
            &image,
            graphics::DrawParam::new()
                .dest(dst)
            ,
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
}