extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate graphics;
extern crate scroll_controller;

use piston::event_loop::{Events, EventSettings};
use piston::window::WindowSettings;
use piston::input::RenderEvent;
use graphics::*;
use graphics::draw_state::DrawState;
use sdl2_window::Sdl2Window;
use opengl_graphics::{OpenGL, GlGraphics};
use scroll_controller::{ScrollController, ScrollSettings};

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("scroll_test", [512; 2])
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: Sdl2Window = settings.build().unwrap();
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut scroll = {
        let scroll_settings = ScrollSettings::new().speed(5.0);
        let scroll_layout = [10.0, 10.0, 100.0, 200.0];
        ScrollController::new(scroll_layout, [250.0; 2], &scroll_settings)
    };

    let black = color::hex("000000");
    let violet = color::hex("ff00ff");
    let circle_red = Ellipse::new(color::hex("ff000033")).resolution(8);
    let circle_blue = Ellipse::new(color::hex("0000ff33")).resolution(8);

    while let Some(e) = events.next(&mut window) {
        scroll.event(&e);

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear([1.0; 4], g);

                Rectangle::new([0.0, 0.0, 1.0, 1.0])
                    .draw(scroll.bounds, &DrawState::new_clip(), c.transform, g);

                let mat = c.transform
                    .trans(scroll.bounds[0], scroll.bounds[0])
                    .trans(scroll.offset[0], scroll.offset[1]);
                let size = 10.0;
                for y in 0..(scroll.area[0]/size) as u32 {
                    for x in 0..(scroll.area[1]/size) as u32 {
                        let rect = [x as f64 * size, y as f64 * size, size, size];
                        circle_red.draw(rect, &DrawState::new_inside(), mat, g);
                        circle_blue.draw(rect, &DrawState::new_outside(), mat, g);
                    }
                }

                Rectangle::new_border(black, 1.0)
                    .draw(scroll.bounds, &c.draw_state, c.transform, g);

                let rect = scroll.rect_from_area_to_view(scroll.visible_area_rect());
                Rectangle::new_border(violet, 1.0)
                    .draw(rect, &c.draw_state, c.transform, g);
            });
        }
    }
}
