mod gltf_parser;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Game", 640, 480)
        .opengl()
        .resizable()
        .build()
        .expect("Could not initialize window with OpenGL support.");

    // these need to remain alive so that the context stays valid
    let _gl_context = window.gl_create_context().expect("Could not create OpenGL context.");
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::ClearColor(0.9, 0.3, 0.5, 1.0);
    }

    let monkey = gltf_parser::parse_gltf("Monkey.gltf").
        expect("Could not parse model.");

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } | 
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}
