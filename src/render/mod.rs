extern crate glium;
extern crate glam;
use crate::math::draw;
use crate::constants::PI;

use glium::{implement_vertex, program, uniform};
use glium::glutin;
use glium::Surface;
use glium::DrawParameters;
use glium::Depth;

use std;
use std::time::{Duration, Instant};
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
    let fast_scalar = 0.8 / dist_scale; //minimize amount of divisions we will need to do, precision is also less of a concern for rendering
    println!("{}" , dist_scale);


    let (mut vertices, normalsvec, tex) : (Vec<Vec3>, Vec<Vec3>, Vec<f32>) = draw::create_sphere(2.0, Vec3::default());
    for iter in 0..vertices.len(){
        vertices[iter] = vertices[iter].fast_normalize();
    }

 

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

    fn create_vertex_buffer(vertices : &Vec<Vec3>, display : &glium::Display) -> glium::VertexBuffer<Vertex>
    {

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
        glium::VertexBuffer::new(display, &vertex_buf).unwrap()
    }
    

    let vertex_buffer = create_vertex_buffer(&vertices, &display);
    

    fn create_normals(normalsvec : &Vec<Vec3>, vertices : &Vec<Vec3>, display : &glium::Display) -> glium::VertexBuffer<Normals>
    {


        let size : usize = normalsvec.len();
        let mut normals_buf = Vec::with_capacity(size);
        for index in 0..size
        {
            let curr_vertices : &Vec3 = &vertices[index];
            normals_buf.push(Normals{
                position : [curr_vertices.x as f32, curr_vertices.y as f32, curr_vertices.z as f32]
            });
        }
        glium::VertexBuffer::new(display, &normals_buf).unwrap()
    }
    
    let normals = create_normals(&normalsvec, &vertices, &display);

    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ]
    };
   
    eventloop.run(move |event, _, control_flow| { 
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    glutin::event_loop::ControlFlow::Poll
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
        let incoming = bodyrx.try_recv();
        
        match incoming {
            Ok(res) => {
                let mut disp = display.draw();
                let blist = res.world.get_body_list();
                let l_indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
                
                disp.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
                for body in blist
                {
                    let r_space_vec : Vec3 = body.position * fast_scalar;
                    let sphere_r_coords = draw::create_sphere(0.05, r_space_vec);
                    let r_vert_buf = create_vertex_buffer(&sphere_r_coords.0, &display);
                    let r_norm_buf = create_normals(&sphere_r_coords.1, &sphere_r_coords.0, &display);
                    disp.draw((&r_vert_buf, &r_norm_buf), &index_buffer, &program, &uniforms, &Default::default()).unwrap();
                    /*
                    if body.bID == 1
                    {
                        let sma = body.get_semimajor_axis(res.world.barycenter_mass);
                        let ecc = body.get_eccentricity(res.world.barycenter_mass);
                        let body_ang = body.position.get_angle(&Vec3::default());
                        let init_angle = (body_ang - body.get_true_anomaly(res.world.barycenter_mass)) * 180.0 / PI;
                        let p_dist = sma * (1.0 - ecc);
                        let kepler_scalar = r_space_vec.abs_self().length() / p_dist;
                        let plot_vec = draw::plot_kepler_orbit(sma, ecc, kepler_scalar, init_angle);
                        let plot_vert_buf = create_vertex_buffer(&plot_vec, &display);
                        disp.draw(&plot_vert_buf, &l_indices, &program, &uniforms, &Default::default()).unwrap();
                    }
                    */

                }
                
                disp.finish().unwrap()
            }
            Err(err) => match err {
                //std::sync::mpsc::TryRecvError::Disconnected => panic!("physics thread killed, exiting"),
                std::sync::mpsc::TryRecvError::Disconnected => {
                    panic!("ControlFlow did not exit!")
                },
                std::sync::mpsc::TryRecvError::Empty => ()
            }
        }

    });
}
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

