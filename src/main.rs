#![allow(unused_imports)]

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use std::io;

mod terrain;
mod flood;
fn main() {
    let (path, height) = get_map();
    match terrain::read_terrain(path.as_str()) {
        Ok(mut terr) => {
            flood::flood(&mut terr.terrain, terr.sources[0], height);
            let (height, width) = terr.size;
            match render(terr.terrain, width, height, terr.range) {
                Ok(_) => {}
                Err(err) => eprintln!("{}", err)
            }
        },
        Err(err) => panic!("{}", err)
    }
}

fn render(terr: Vec<Vec<(f64, bool)>>, width: u32, height: u32, range: (f64, f64)) -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
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
        Pixels::new(width, height, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            draw(pixels.frame_mut(), width, &terr, range);
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

fn get_map() -> (String, f64) {
    println!("Select map:");
    let map = terrain::get_terrain_files("./terrain");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read index input.");
    let index: usize = index.trim().parse().expect("Could not convert index to a numbe.r");
    let path = map[index-1].clone();

    println!("Input water height (SEA LEVEL: 0): ");
    let mut water_height = String::new();
    io::stdin().read_line(&mut water_height).expect("Failed to read water height input.");
    let water_height: f64 = water_height.trim().parse().expect("Could not convert water height to a number.");
    (path, water_height)
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn draw(frame: &mut [u8], width: u32, terr: &Vec<Vec<(f64, bool)>>, range: (f64, f64)) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % width as usize;
        let y = i / width as usize;
        let data = terr[y][x];
        let rgba = if data.1 {
            [0x00, 0x00, 0xff, 0xff]
        } else {
           color_map(data.0, range)
        };
        pixel.copy_from_slice(&rgba);
    }
}

fn color_map(height: f64, range: (f64, f64)) -> [u8; 4] {
    let map: [[u8; 4]; 9] = [
        [0xf7, 0xfc, 0xf0, 0xff], 
        [0xe0, 0xf3, 0xdb, 0xff], 
        [0xcc, 0xeb, 0xc5, 0xff], 
        [0xa8, 0xdd, 0xb5, 0xff],
        [0x7b, 0xcc, 0xc4, 0xff],
        [0x4e, 0xb3, 0xd3, 0xff],
        [0x2b, 0x8c, 0xb3, 0xff],
        [0x08, 0x68, 0xac, 0xff],
        [0x08, 0x40, 0x81, 0xff]
        ];
        
    let min = range.1;
    let max = range.0;
    let normalized: usize = (((height - min) / (max - min))*10.0).floor() as usize;
    if normalized >= 9{
        map[8]
    } else {
        map[normalized]
    }
}