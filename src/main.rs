extern crate sdl2;

use sdl2::{
    pixels::{
        Color,
    },
    event::{
        Event
    },
    keyboard::{
        Keycode
    },
    rect::{
        Point,
        Rect   
    },
};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("test", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }

        draw(&mut canvas);
    }
}

fn draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 40));
    canvas.clear();

    let mut rectangle = Rect::new(0, 0, 1, 1);
    rectangle.w = 100;
    rectangle.h = 100;
    rectangle.x = 0;
    rectangle.y = 0;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    // draw rect outline
    canvas.draw_rect(rectangle).unwrap();
    
    // fill rect outline
    rectangle.x = 101;
    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.fill_rect(rectangle).unwrap();

    // draw line
    let start_x = 0;
    let start_y = 0;
    let end_x = 200;
    let end_y = 200;

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.draw_line(Point::new(start_x, start_y), Point::new(end_x, end_y)).unwrap();

    // draw point
    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas.draw_point(Point::new(100, 100)).unwrap();

    canvas.present();
}
