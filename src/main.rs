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

fn main() -> Result<(), Error> {
    let (mut terr, size, source) = terrain::read_terrain("terrain/NewYorkCity.terrain").unwrap();

    //let (mut terr, size, source) = terrain::read_terrain("terrain/iceland.terrain").unwrap();
    /*let terr: Vec<Vec<(f64, bool)>> = vec![
        vec![(1, false), (0, false), (2, false), (2, false), (0, false)],
        vec![(0, false), (2, false), (0, false), (2, false), (0, false)],
        vec![(2, false), (2, false), (2, false), (2, false), (2, false)],
        vec![(0, false), (0, false), (2, false), (0, false), (2, false)],
        vec![(1, false), (0, false), (0, false), (0, false), (0, false)],
    ];*/

    println!("SIZE: {:?}", size);

    let height: u32 = size.0;
    let width: u32 = size.1;

    let water_height: f64 = 0.0;
    flood::flood(&mut terr, source, water_height);

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
            draw(pixels.frame_mut(), width, &terr);
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

fn draw(frame: &mut [u8], width: u32, terr: &Vec<Vec<(f64, bool)>>) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % width as usize;
        let y = i / width as usize;
        let data = terr[y][x];
        let rgba = if data.1 {
            [0x00, 0x00, 0xff, 0xff]
        } else {
           color_map(data.0)
        };
        pixel.copy_from_slice(&rgba);
    }
}

fn color_map(height: f64) -> [u8; 4] {
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
    
    let min = -42.0;
    let max = 274.0;
    let normalized: usize = (((height - min) / (max - min))*10.0).floor() as usize;
    if normalized >= 9{
        map[0]
    } else {
        map[normalized]
    }
}
