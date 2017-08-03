#![deny(missing_docs)]

//! Ludum Dare "Running Out of Power" - Murder Lord

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate texture;

use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL, Texture};
use std::path::Path;
use graphics::rectangle::square;
use graphics::{Image, clear};

pub use fight::Fight;
pub use fight_controller::FightController;
pub use fight_view::*;
pub use characters::*;

mod fight;
mod fight_controller;
mod fight_view;
mod characters;


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;


    // Create an Glutin window.
    let mut window = Window::new(
            opengl,
            WindowSettings::new(
                "murder_lord",
                [1280, 720]
            )
            .exit_on_esc(true)
            .build()
            .unwrap());
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    let image = Image::new().rect(square(0.0,0.0,200.0));
    let image2 = Image::new().rect(square(500.0, 500.0, 200.0));
    let tex = Texture::from_path(&mut window.factory, "test.jpg", Flip::None,&TextureSettings::new()).unwrap();
    let tex2 = Texture::from_path(&mut window.factory, "test.jpg", Flip::None,&TextureSettings::new()).unwrap();

    let mut player = Player::new(tex, image);
    let mut enemy = Enemy::new(tex2, image2);

    let fight = Fight::new(player.visuals(), enemy.visuals(), player.stats(), enemy.stats());
    let fight_view_settings = FightViewSettings::new();
    let fight_view = FightView::new(fight_view_settings);

    for e in window.events() {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c,g|{

                clear([0.0, 0.0, 0.0, 1.0], g);
                //fight.player_visuals.image().draw(fight.player_visuals.texture(), &graphics::DrawState::default(), c.transform, g);
                //fight_view.draw(&fight_controller, &c, g);
                let board_rect = [
                    0.0, 0.0, 1280.0, 720.0
                ];

                Rectangle::new([0.8, 0.8, 1.0, 1.0]).draw(board_rect, &c.draw_state, c.transform, g);
                player.image.draw(&player.texture, &graphics::DrawState::default(), c.transform, g);
                //image.draw(&tex, &graphics::DrawState::default(), c.transform, g);
            });
        }
    }
}
