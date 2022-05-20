use crate::core::Status;
use crate::trust::{self, UserInput};
use sdl2::event::Event;
use sdl2::mouse::MouseButton;

pub struct EventPump {
    event_pump: sdl2::EventPump,
}

impl EventPump {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, Status> {
        Ok(EventPump {
            event_pump: sdl_context.event_pump()?,
        })
    }

    pub fn poll(&mut self) -> UserInput {
        match self.event_pump.poll_event() {
            Some(event) => self.build_event(event),
            None => UserInput {
                event: Some(trust::user_input::Event::NoEvent(trust::NoEvent {})),
            },
        }
    }

    fn build_event(&self, event: Event) -> UserInput {
        match event {
            Event::Quit { .. } => UserInput {
                event: Some(trust::user_input::Event::QuitEvent(trust::QuitEvent {})),
            },
            Event::KeyDown {
                keycode: Some(key),
                repeat,
                ..
            } => UserInput {
                event: Some(trust::user_input::Event::KeyEvent(trust::KeyEvent {
                    key: key.to_string(),
                    key_state: match repeat {
                        false => trust::KeyState::Pressed as i32,
                        true => trust::KeyState::None as i32,
                    },
                    ..Default::default()
                })),
            },
            Event::KeyUp {
                keycode: Some(key), ..
            } => UserInput {
                event: Some(trust::user_input::Event::KeyEvent(trust::KeyEvent {
                    key: key.to_string(),
                    key_state: trust::KeyState::Released as i32,
                    ..Default::default()
                })),
            },
            Event::MouseMotion {
                x, y, xrel, yrel, ..
            } => UserInput {
                event: Some(trust::user_input::Event::MouseEvent(trust::MouseEvent {
                    absolute_position: Some(trust::Vector {
                        x: x as f64,
                        y: y as f64,
                        z: 0.0,
                    }),
                    relative_position: Some(trust::Vector {
                        x: xrel as f64,
                        y: yrel as f64,
                        z: 0.0,
                    }),
                    ..Default::default()
                })),
            },
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => UserInput {
                event: Some(trust::user_input::Event::MouseEvent(trust::MouseEvent {
                    button: translate_mouse_button(mouse_btn),
                    key_state: trust::KeyState::Pressed as i32,
                    absolute_position: Some(trust::Vector {
                        x: x as f64,
                        y: y as f64,
                        z: 0.0,
                    }),
                    ..Default::default()
                })),
            },
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => UserInput {
                event: Some(trust::user_input::Event::MouseEvent(trust::MouseEvent {
                    button: translate_mouse_button(mouse_btn),
                    key_state: trust::KeyState::Released as i32,
                    absolute_position: Some(trust::Vector {
                        x: x as f64,
                        y: y as f64,
                        z: 0.0,
                    }),
                    ..Default::default()
                })),
            },
            _ => UserInput {
                event: Some(trust::user_input::Event::NoEvent(trust::NoEvent {})),
            },
        }
    }
}

fn translate_mouse_button(mouse_btn: MouseButton) -> String {
    match mouse_btn {
        MouseButton::Left => String::from("Left"),
        MouseButton::Middle => String::from("Middle"),
        MouseButton::Right => String::from("Right"),
        _ => String::from(""),
    }
}