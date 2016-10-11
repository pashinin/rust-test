#[macro_use]
extern crate glium;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_title(format!("Rusted"))
        .build_glium().unwrap();

    loop {
        let mut target = display.draw();
        // 002a35   42 53
        target.clear_color(0.0, 0.16, 0.2, 1.0);
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
