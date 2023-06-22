#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod terrain;
mod flood;

const WIDTH: u32 = 360;
const HEIGHT: u32 = 360;

fn main() -> Result<(), Error> {

   
    let terr = terrain::read_terrain("terrain/iceland_test.terrain");
    /*let terr: Vec<Vec<(i64, bool)>> = vec![
        vec![(1, false), (0, false), (2, false), (2, false), (0, false)],
        vec![(0, false), (2, false), (0, false), (2, false), (0, false)],
        vec![(2, false), (2, false), (2, false), (2, false), (2, false)],
        vec![(0, false), (0, false), (2, false), (0, false), (2, false)],
        vec![(1, false), (0, false), (0, false), (0, false), (0, false)],
    ];*/

    println!("{:?}", terr);

    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Flood Simulator")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
             if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
        }

        window.request_redraw();
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn draw(frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % WIDTH as usize) as i16;
        let y = (i / WIDTH as usize) as i16;
        //println!("{}", i);
        pixel.copy_from_slice(&[0, 0xff, 0xff, 0xff]);
    }
}

