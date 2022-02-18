extern crate glium;
extern crate glam;
use crate::math::lin_alg;
use glium::{implement_vertex, program, uniform};
use glium::glutin;
use glium::Surface;
use glium::DrawParameters;
use glium::Depth;
use std;
use crate::math::phys::Phys;
use glutin::platform::windows::EventLoopExtWindows;
use crate::math::vec::Vec3;
use crate::p_engine;

pub fn init_Render<'a>(bodyrx : std::sync::mpsc::Receiver<p_engine::PEngine>)
{
    #[derive(Copy, Clone)]
    struct Normals {
        position : [f32; 3]
    }
    implement_vertex!(Normals, position);
    #[derive(Copy, Clone)]
    struct Vertex {
        position : [f32; 3],
        color : [f32; 3]
    }
    implement_vertex!(Vertex, position, color);



    let eventloop = glutin::event_loop::EventLoop::<glutin::event::Event<glutin::event::WindowEvent>>::new_any_thread();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &eventloop).unwrap();
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()

        },
        .. Default::default()
    };

    let first_iter = bodyrx.recv().unwrap();
    
    let farther_object = &first_iter.world.bodylist[&first_iter.world.bodylist.len() - 1];
    let dist_scale = farther_object.position.get_distance_sum(&Vec3::default());
    println!("{}" , dist_scale);


    let (mut vertices, normalsvec, tex) : (Vec<Vec3>, Vec<Vec3>, Vec<f32>) = lin_alg::create_sphere(2.0);
    for iter in 0..vertices.len(){
        vertices[iter] = vertices[iter].fast_normalize();
    }

    let vertex_buffer = {

        let size : usize = vertices.len();
        let mut vertex_buf = Vec::with_capacity(size);
        for index in 0..size
        {
            let curr_vertices : &Vec3 = &vertices[index];
            vertex_buf.push(Vertex{
                position : [curr_vertices.x as f32, curr_vertices.y as f32, curr_vertices.z as f32],
                color : [1.0, 1.0, 1.0]
            })
        }
        glium::VertexBuffer::new(&display, &vertex_buf).unwrap()
    };
    

    let normals = {


        let size : usize = normalsvec.len();
        let mut normals_buf = Vec::with_capacity(size);
        for index in 0..size
        {
            let curr_vertices : &Vec3 = &vertices[index];
            normals_buf.push(Normals{
                position : [curr_vertices.x as f32, curr_vertices.y as f32, curr_vertices.z as f32]
            });
        }
        glium::VertexBuffer::new(&display, &normals_buf).unwrap()
    };
    
    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
    let program = program!(&display, 410 => {vertex: "
    #version 410
    uniform mat4 matrix;
    
    in vec3 position;
    in vec3 color;

    out vec3 vColor;
    
    void main(){
        gl_Position = vec4(position, 1.0) * matrix;
        vColor = color;
    }
    ", 
    fragment: "
    #version 410
    in vec3 vColor;
    out vec4 f_color;

    void main(){
        f_color = vec4(vColor, 1.0);
    }
    "}).unwrap();


    let draw = move || {
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        let mut res = display.draw();
        res.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        res.draw((&vertex_buffer, &normals), &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        res.finish().unwrap();
    };

    draw();

    eventloop.run(move |event, _, control_flow| {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    draw();
                    glutin::event_loop::ControlFlow::Poll
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
        let incoming = bodyrx.try_recv();
        match incoming {
            Ok(res) => {
                let blist = res.world.get_body_list();
                for body in blist
                {
                    
                }
            }
            Err(err) => match err {
                std::sync::mpsc::TryRecvError::Disconnected => return,
                std::sync::mpsc::TryRecvError::Empty => ()
            }
        } 
    });

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