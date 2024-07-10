// input.rs
use crate::entities::Direction;
use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct InputHandler {
    device_state: DeviceState,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
        }
    }

    pub fn get_direction(&self) -> Option<Direction> {
        let keys: Vec<Keycode> = self.device_state.get_keys();

        if keys.contains(&Keycode::W) {
            Some(Direction::Up)
        } else if keys.contains(&Keycode::S) {
            Some(Direction::Down)
        } else if keys.contains(&Keycode::A) {
            Some(Direction::Left)
        } else if keys.contains(&Keycode::D) {
            Some(Direction::Right)
        } else {
            None
        }
    }

    pub fn should_quit(&self) -> bool {
        self.device_state.get_keys().contains(&Keycode::Q)
    }
}
