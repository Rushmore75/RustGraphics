use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{Event, MouseButton, VirtualKeyCode}};

use crate::{bounding_box::Boxes, window::{WindowState, StatusState}, vertex::{Vertex, self}};


/// Create the window and start the event loop (BLOCKS!)
pub async fn run() {
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();


    // FIXME these functions aren't getting called when they
    // should be /shrug
    // The problem is inside the butt class
    fn click() {
        println!("click");
    }
    fn hover() {
        println!("hover");
    }
    let mut buttons = Boxes::new();
    buttons.add_button(
        [0.0,0.0],
        [100.0, 100.0],
        click,
        hover
    );

    let mut state = WindowState::new(&window, buttons).await;

    event_loop.run(move | event, _, control_flow: &mut ControlFlow| {

        
        let (i, v) = update(&mut state);
        /*
            Window loop
        */
        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // these will be dynamically replaced
                state.queue_buffer(&v, i);
                match state.render() {
                    Ok(_) => {}
                    // reconfigure if the surface is lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size()),
                    // the system is out of memory, die
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // Other errors
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
                state.window_events(event, control_flow);               
            }   
            _ => {}
        }
    });        
}

fn update(state: &mut WindowState) -> (&'static [u16], [vertex::Vertex; 3]) {
    // this gets called 1/frame (i think)
    let p_state = &mut state.status_state.peripheral;
    let b_state = &mut state.status_state.button_widget;
    

    //===================================================
    //                 Peripherals
    //===================================================
    match p_state.last_key {
        Some(key) => {
            println!("{:?}", key);
            p_state.last_key = None; // we read it


            if key == VirtualKeyCode::W && p_state.get_key(&VirtualKeyCode::LControl) {
                println!("🥳");
            }
        }
        None => {},
    }
    match p_state.last_mouse {
        Some(key) => {
            println!("{:?}", key);
            p_state.last_mouse = None; // we read it
        }
        None => {},
    }

    if p_state.pointer_moved {
        b_state.process_mouse(
            p_state.get_pointer(),
            p_state.get_mouse(&MouseButton::Left)
        );
        println!("[x, y] {:?}", p_state.get_pointer());
        p_state.pointer_moved = false; // denotes that we read the value
    }    

    //===================================================
    //              Building next frame
    //===================================================
    let vertices: [Vertex; 3] = [
        Vertex { position: [
            (state.status_state.peripheral.get_pointer()[0] as f32 / state.size().width as f32 * 2.0 ) -1.0,
            -((state.status_state.peripheral.get_pointer()[1] as f32 / state.size().height as f32 * 2.0) -1.0),
            0.0], color: [0.5, 0.0, 0.5]
        },
        Vertex { position: [-0.5, -1.0, 0.0],  color: [0.5, 0.0, 0.5] },
        Vertex { position: [0.5, -1.0, 0.0], color: [0.5, 0.0, 0.5] },
    ];

    let indices: &[u16] = &[
        0, 1, 2
    ];

    return (indices, vertices);

}