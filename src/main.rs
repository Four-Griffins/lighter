use clap::Parser;
use glium::Surface;
use glium::{glutin, glutin::event::{Event, WindowEvent}, uniform, implement_vertex};
use glium::glutin::event_loop::ControlFlow;

use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};

#[derive(Parser)]
struct Args {
    ///Redraw every frame. Use this for animations
    #[arg(short, long, default_value_t = false)]
    live: bool,

    ///Path to a GLSL fragment shader to apply
    #[arg()]
    path: String,
}

#[derive(Clone, Copy)]
struct Vertex { position: [f32; 2] }
implement_vertex!(Vertex, position);

fn get_shader(display: &glium::Display, path: &str) -> glium::Program {
    let vert = include_str!("./vert.glsl");
    let frag = std::fs::read_to_string(path).expect("failed to read file");
    //TODO better handling here to not panic
    glium::Program::from_source(display, vert, &frag, None).expect("compilation failed")
}

fn draw(
    display: &glium::Display,
    vertices: &glium::VertexBuffer<Vertex>,
    indices: &glium::IndexBuffer<u8>,
    shader: &glium::Program,
    time: f32, frame: i32,
) {
    let size = display.get_framebuffer_dimensions();
    let uniforms = uniform! {
        u_resolution: [size.0 as f32, size.1 as f32, size.1 as f32 / size.0 as f32],
        u_time: time,
        u_frame: frame,
    };
    let mut target = display.draw();
    target.clear_color(0., 0., 0., 0.);
    target.draw(vertices, indices, shader, &uniforms, &Default::default()).unwrap();
    target.finish().unwrap();
}

fn main() {
    let args = Args::parse();
    
    //standard glium initialization
    let eventloop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &eventloop).unwrap();
    let mut shader = get_shader(&display, &args.path);
    
    // Simply A Quad(TM)
    let vert1 = Vertex { position: [-1., -1.] };
    let vert2 = Vertex { position: [-1., 1.] };
    let vert3 = Vertex { position: [1., -1.] };
    let vert4 = Vertex { position: [1., 1.] };

    let vertices = glium::VertexBuffer::new(&display, &vec![vert1, vert2, vert3, vert4]).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &[0u8, 1, 2, 1, 2, 3]).unwrap();
    
    //draw once to make the window appear
    draw(&display, &vertices, &indices, &shader, 0., 0);
    
    //watching changes in the user file, and sending a custom event to the glium event loop further down
    let (tx, rx) = std::sync::mpsc::channel();
    let proxy = eventloop.create_proxy();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    //this is nonblocking, the blocking heappens when waiting for events from the receiver
    watcher.watch(args.path.as_ref(), RecursiveMode::NonRecursive).unwrap();

    //the main thread will be busy with the glium eventloop, so listen for file changes in a different thread
    //and send those updates to the main thread using the handy dandy glium custom events
    std::thread::spawn(move || {
        for ev in rx {
            match ev {
                Ok(_) => { proxy.send_event(()).unwrap(); },
                Err(why) => { eprintln!("failed to send event: {why}"); },
            }
        }
    });
    
    let start_time = std::time::Instant::now();
    let mut frame = 0;
    eventloop.run(move |ev, _, flow| {
        *flow = if args.live { ControlFlow::Poll } else { ControlFlow::Wait };
        
        let time = (std::time::Instant::now() - start_time).as_secs_f32();
        match ev {
            Event::RedrawRequested(..) => {
                if !args.live { frame += 1; draw(&display, &vertices, &indices, &shader, time, frame); }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
                WindowEvent::Resized(..) => {
                    if !args.live { frame += 1; draw(&display, &vertices, &indices, &shader, time, frame); }
                }
                _ => {},
            }
            Event::UserEvent(..) => {
                shader = get_shader(&display, &args.path);
                if !args.live { frame += 1; draw(&display, &vertices, &indices, &shader, time, frame); }
            }
            Event::MainEventsCleared => {
                if args.live { frame += 1; draw(&display, &vertices, &indices, &shader, time, frame); }
            }
            _ => {},
        }
    });
}