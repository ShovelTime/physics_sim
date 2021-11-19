extern crate rend3;
use std;

pub struct Rend3SolarSys
{
    objects : Vec<Option<rend3::types::ObjectHandle>>,
    directional_lights : Vec<Option<rend3::types::DirectionalLightHandle>>,
}


pub fn Init_Render(input : Rend3SolarSys) -> i32
{
    let eventloop = winit::event_loop::EventLoop::new();
    let window;
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("Physics Simulation");
    window = builder.build(&eventloop).expect("windows failed to build!");

    let window_size = window.inner_size();
    let iad = pollster::block_on(rend3::create_iad(None, None, None)).unwrap();
    let surface = std::sync::Arc::new(unsafe { iad.instance.create_surface(&window) });
    let format = surface.get_preferred_format(&iad.adapter).unwrap();

    rend3::configure_surface(
        &surface,
        &iad.device,
        format,
        glam::UVec2::new(window_size.width, window_size.height),
        rend3::types::PresentMode::Mailbox,
    );

    let renderer = rend3::Renderer::new(iad, Some(window_size.width as f32 / window_size.height as f32)).unwrap();
    return 0;


}