#[macro_use]
extern crate wgpu;

use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod bus;
pub mod components;
mod emulator;
mod graphics;

use emulator::Emulator;
use graphics::state;
use components::rom::Rom;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run(rom: &Rom) {
    let scale_factor = 2;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(256 * scale_factor, 240 * scale_factor))
        .with_title("nes")
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")] { use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(256 * scale_factor, 240 * scale_factor));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-nes")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }
    let mut diffuse_bytes = vec![0x00; 256 * 240 * 4];
    let mut state = state::State::new(&window).await;
    let mut emulator = Emulator::new();

    emulator.reset();
    emulator.load_rom(rom);

    emulator.run();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            for i in (0..diffuse_bytes.len()).step_by(2048) {
                let color = 0x66;
                for x in (i..(i + 1024)).step_by(4) {
                    diffuse_bytes[x as usize] = color;
                    diffuse_bytes[x as usize + 1] = color;
                    diffuse_bytes[x as usize + 2] = color;
                }
            }

            state.update(&diffuse_bytes);

            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    state.resize(state.size)
                }
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(_) => {}
            }
        }
        _ => {}
    });
}
