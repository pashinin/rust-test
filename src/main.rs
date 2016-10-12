#[macro_use]
extern crate glium;
use glium::glutin::{Event, VirtualKeyCode, WindowBuilder};
use glium::backend::glutin_backend::GlutinFacade;


fn draw(display:&GlutinFacade){
    use glium::{Surface};
    let mut target = display.draw();
    target.clear_color(0.0, 0.16, 0.2, 1.0);
    target.finish().unwrap();
}

fn main() {
    use glium::{DisplayBuild};
    let display = WindowBuilder::new()
        .with_title(format!("Rusted"))
        .build_glium().unwrap();

    loop {
        // for ev in display.poll_events() {   // slower
        for ev in display.wait_events() {
            // display.get_window().swap_buffers();

            match ev {
                glium::glutin::Event::Closed => return,
                // Quit on Esc:
                Event::KeyboardInput(_ , _, Some(VirtualKeyCode::Escape)) => return,
                Event::Refresh => {
                    draw(&display);
                },
                _ => ()
            }
        }
    }
}
