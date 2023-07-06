use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct IO {
    device_state: DeviceState,
}

impl IO {
    pub fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
        }
    }

    pub fn get_keys(&self) -> Vec<Keycode> {
        self.device_state.get_keys()
    }
}
