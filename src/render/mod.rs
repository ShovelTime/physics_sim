extern crate glium;
use glium::glutin;
use std;
use glutin::platform::windows::EventLoopExtWindows;
use crate::p_engine;
pub struct Rend3SolarSys
{
    camera : rend3::types::Camera,
    objects : Vec<Option<rend3::types::ObjectHandle>>,
    directional_lights : Vec<Option<rend3::types::DirectionalLightHandle>>,
}

pub fn init_Render<'b>(bodyrx : std::sync::mpsc::Receiver<Vec<p_engine::Body>>)
{
    let eventloop = glutin::event_loop::EventLoop::<glutin::event::Event<glutin::event::WindowEvent>>::new_any_thread();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &eventloop);
    let 

    /*
    let eventloop = winit::event_loop::EventLoop::<winit::event::Event<winit::event::WindowEvent>>::new_any_thread();
    let window;
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("Physics Simulation");
    window = builder.build(&eventloop).expect(" the Window object failed to build! Use linux instead. Nerd.");

    let window_size = window.inner_size();
    let iad = pollster::block_on(rend3::create_iad(None, None, None)).unwrap();
    let surface = std::sync::Arc::new(unsafe { iad.instance.create_surface(&window) });
    let format = surface.get_preferred_format(&iad.adapter).unwrap();
/*  
    rend3::configure_surface(
        &surface,
        &iad.device,
        format,
        glam::UVec2::new(window_size.width, window_size.height),
        rend3::types::PresentMode::Mailbox,
    );

    let renderer = rend3::Renderer::new(iad, Some(window_size.width as f32 / window_size.height as f32)).unwrap();

    let base_rendergraph = rend3_routine
*/
    //renderer.set_camera_data(data: Camera)
    
    let env_loop = eventloop.run(move |event, _, control| match event {
        // Close button was clicked, we should close.
        winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::CloseRequested,
            ..
        } => {
            *control = winit::event_loop::ControlFlow::Exit;
        }
        // Window was resized, need to resize renderer.
        winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::Resized(size),
            ..
        } => {
            let size = glam::UVec2::new(size.width, size.height);
        
            // Reconfigure the surface for the new size.
            rend3::configure_surface(
                &surface,
                &renderer.device,
                format,
                glam::UVec2::new(size.x, size.y),
                rend3::types::PresentMode::Mailbox,
            );
            // Tell the renderer about the new aspect ratio.
            renderer.set_aspect_ratio(size.x as f32 / size.y as f32);
            // Resize the internal buffers to the same size as the screen.
            /*
            pbr_routine.resize(
                &renderer,
                rend3_routine::RenderTextureOptions {
                    resolution: size,
                    samples: rend3_routine::SampleCount::One,
                },
            );
            tonemapping_routine.resize(size);
            */
        }
        _ => {
                
        }
    });
    */

}