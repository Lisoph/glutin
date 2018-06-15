extern crate glutin;

mod support;

use glutin::{GlContext, dpi::LogicalSize};
use glutin::os::macos::{WindowExt, BlurMaterial};

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("A fantastic window!")
        //.with_decorations(false)
        .with_blur(true);
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    gl_window.set_blur_material(BlurMaterial::Dark);

    let _ = unsafe { gl_window.make_current() };

    println!("Pixel format of the window's GL context: {:?}", gl_window.get_pixel_format());

    let gl = support::load(&gl_window);

    events_loop.run_forever(|event| {
        println!("{:?}", event);
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => return glutin::ControlFlow::Break,
                glutin::WindowEvent::Resized(LogicalSize { width: w, height: h }) => gl_window.resize(w as u32, h as u32),
                _ => (),
            },
            _ => (),
        }

        gl.draw_frame([0.0, 0.0, 0.0, 0.0]);
        let _ = gl_window.swap_buffers();
        glutin::ControlFlow::Continue
    });
}
