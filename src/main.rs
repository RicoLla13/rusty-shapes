extern crate sdl2;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod canvas;
mod points;
mod utils;

use canvas::*;
use points::*;
use utils::*;

fn main() {
    let mut context = SdlContext::init();
    let mut canvas1 = context.new_canvas(&"3D Shapes");
    let mut canvas2 = context.new_canvas(&"2D Shapes");

    let mut frame_count = 0;

    let rotation = 0.02;
    let mut angle = Vector3D::new(0.0, 0.0, 0.0);
    'running: loop {
        for event in context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window {
                    win_event: WindowEvent::Close,
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas1.clear();
        canvas2.clear();

        // 3D Shapes Window
        let center_c = Point3D::new(70, 0, -70);
        let center_p = Point3D::new(-70, 0, 70);

        canvas1.draw_line_3d(
            &Point3D::new(0, 100, 0),
            &Point3D::new(0, -100, 0),
            Color::RED,
        );
        canvas1.draw_cube(&center_c, &angle);
        canvas1.draw_pyramid(&center_p, &angle);

        // angle.x += rotation;
        angle.y += rotation;
        // angle.z += rotation;
        angle.angle_overshoot();

        // 2D Shapes Window
        let center_circ = Point2D::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGTH as i32 / 2);
        let radius = 50;
        canvas2.draw_pixel(&center_circ, Color::RED);
        canvas2.draw_circle(&center_circ, radius, Color::WHITE);
        canvas2.draw_line_2d(
            &center_circ,
            &Point2D::new(center_circ.x, center_circ.y + 2 * radius),
            Color::RGB(252, 128, 5),
        );
        canvas2.draw_line_2d(
            &center_circ,
            &Point2D::new(center_circ.x, center_circ.y - 2 * radius),
            Color::RGB(252, 128, 5),
        );
        canvas2.draw_line_2d(
            &center_circ,
            &Point2D::new(center_circ.x + 2 * radius, center_circ.y),
            Color::RGB(252, 128, 5),
        );
        canvas2.draw_line_2d(
            &center_circ,
            &Point2D::new(center_circ.x - 2 * radius, center_circ.y),
            Color::RGB(252, 128, 5),
        );

        frame_count += 1;
        if frame_count > FRAMERATE {
            frame_count = 0;
        }

        canvas1.present();
        canvas2.present();
        std::thread::sleep(Duration::new(0, FRAMERATE_CALC));
    }
}
