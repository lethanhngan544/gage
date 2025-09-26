use crate::engine::state::State;
use crate::engine::bus::Bus;
use crate::engine::types::{CVarValue, Event};

use log;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct WindowState {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

pub struct Window {
    state: Arc<Mutex<WindowState>>,
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    reciever: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    
}

impl Window {
    fn error_callback(error: glfw::Error, str: String) {
        log::error!("GLFW error: {:?}, {}", error, str);
    }
    
    pub fn new(width: u32, height: u32, title: String) -> Self {
        // glfw::init_hint(glfw::InitHint::Platform(glfw::Platform::Win32));
        let mut glfw = glfw::init(Window::error_callback).unwrap();

        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        let (mut window, reciever) = glfw.create_window(
            width as u32,
            height as u32,
            title.as_str(),
            glfw::WindowMode::Windowed
        ).unwrap();
        window.set_all_polling(true);

        Self {
            state: Arc::new(Mutex::new(WindowState { width, height, title })),
            glfw,
            window,
            reciever,
        }
    }

    pub fn register(&mut self, state: Arc<Mutex<State>>, bus: Arc<Mutex<Bus>>) {
        let mut engine_state_locked = state.lock().unwrap();
        let mut engine_bus_locked = bus.lock().unwrap();
        let state_clone = self.state.clone();
        {
            let state_locked = state_clone.lock().unwrap();
            engine_state_locked.register("window_width", CVarValue::Int(state_locked.width as i32));
            engine_state_locked.register("window_height", CVarValue::Int(state_locked.height as i32));
            engine_state_locked.register("window_title", CVarValue::Str(state_locked.title.clone()));

        }

        engine_bus_locked.subscribe(move |event| {
            match event {
                Event::WindowResized(width, height) => {
                    let mut state_locked = state_clone.lock().unwrap();
                    state_locked.width = width;
                    state_locked.height = height;
                }
                _ => {}
            }
        });

    }

    pub fn running(&self) -> bool {
        !self.window.should_close()
    }

    pub fn update(&mut self, state: Arc<Mutex<State>>, bus: Arc<Mutex<Bus>>) {
        self.glfw.poll_events();
        let mut bus_lock = bus.lock().unwrap();
        let mut state_lock = state.lock().unwrap();
        //Reciever events and dispatch to message bus
        for (_, event) in glfw::flush_messages(&self.reciever) {
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    bus_lock.post(Event::MouseMoved(x as u32, y as u32));
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    bus_lock.post(Event::WindowResized(width as u32, height as u32));
                    state_lock.set("window_width", CVarValue::Int(width as i32));
                    state_lock.set("window_height", CVarValue::Int(height as i32));
                }
                _ => ()
            }
        }

    }
}