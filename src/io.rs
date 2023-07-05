use device_query::{DeviceQuery, DeviceState, Keycode};

// A function that returns a vector of keys pressed in the terminal since its last call
pub fn get_keys_pressed() -> Vec<Keycode> {
    // Create a device state object
    let device_state = DeviceState::new();
    // Get the keys from the device state
    let keys = device_state.get_keys();
    // Return the keys
    keys
}
