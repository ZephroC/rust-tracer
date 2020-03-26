//! Basic hello world example.

use ggez;
use nalgebra;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use ggez::graphics::Image;


struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let array: [u8; 1280 * 720 * 4] = [255; 1280 * 720 * 4];

        let dst = nalgebra::Point2::new(0.0,0.0);
        let image = graphics::Image::from_rgba8(ctx, 1280,720, &array).unwrap();
        graphics::draw(
            ctx,
            &image,
            graphics::DrawParam::new()
                .dest(dst)
                ,
        )?;
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            nalgebra::Point2::new(0.0, 0.0),
            100.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (nalgebra::Point2::new(self.pos_x, 380.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

// Now our main function, which does three things:
//
// * First, create a new `ggez::ContextBuilder`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn main() -> GameResult {

    let window_setup = ggez::conf::WindowSetup::default().title("Bad Ray Tracer");
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
    let window_mode = ggez::conf::WindowMode::default().dimensions(1280.0, 720.0);

    let cb = ggez::ContextBuilder::new("helloworld", "ggez")
        .window_setup(window_setup )
        .window_mode(window_mode);
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}